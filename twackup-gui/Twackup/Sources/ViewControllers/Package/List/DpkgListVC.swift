//
//  DpkgListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import BlankSlate

class DpkgListVC: SelectablePackageListVC<FFIPackage> {
    override class var metadata: (any ViewControllerMetadata)? {
        AllPkgsMetadata()
    }

    let dpkgProvider: DpkgDataProvier

    private lazy var rebuildAllBarBtn: UIBarButtonItem = {
        let title = "debs-rebuildall-btn".localized
        return UIBarButtonItem(title: title, primaryAction: UIAction { [self] _ in
            setEditing(false, animated: true)
            rebuild(packages: dataSource.dataProvider.packages)
        })
    }()

    private lazy var rebuildSelectedBarBtn: UIBarButtonItem = {
        let title = "debs-rebuildselected-btn".localized
        return UIBarButtonItem(title: title, primaryAction: UIAction { [self] _ in
            rebuild(packages: dataSource.selected())
        })
    }()

    private lazy var refreshControl: UIRefreshControl = {
        let refresh = UIRefreshControl()
        refresh.addAction(UIAction { [self] _ in
            refresh.beginRefreshing()
            Task {
                await reloadData(animated: false, force: true)
            }
        }, for: .valueChanged)

        return refresh
    }()

    init(mainModel: MainModel, detail: PackageDetailVC<FFIPackage>, leaves: Bool = false) {
        dpkgProvider = DpkgDataProvier(mainModel.dpkg, leaves: leaves)

        super.init(mainModel: mainModel, detail: detail)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        tableView.refreshControl = refreshControl
        tableView.bs.dataSource = self
    }

    override func reloadData(animated: Bool, force: Bool) async {
        await super.reloadData(animated: animated, force: force)

        // Twackup parses database so quick that user can think it is not working as he expects
        // That's why there's half-of-a-second delay
        try? await Task.sleep(nanoseconds: 500 * 1_000_000)
        refreshControl.endRefreshing()
    }

    override func selectionDidUpdate() {
        super.selectionDidUpdate()

        if isEditing {
            guard var buttons = toolbarItems, !buttons.isEmpty else { return }
            buttons[0] = dataSource.isAnySelected ? rebuildSelectedBarBtn : rebuildAllBarBtn
            setToolbarItems(buttons, animated: false)
        }
    }

    // MARK: - Actions

    override func setEditing(_ editing: Bool, animated: Bool) {
        super.setEditing(editing, animated: animated)

        if editing {
            setToolbarItems([rebuildAllBarBtn], animated: false)
        }

        navigationController?.setToolbarHidden(!editing, animated: animated)
    }

    override func configureDataSource() -> PackageListDataSource<FFIPackage> {
        DpkgListDataSource(tableView: tableView, dataProvider: dpkgProvider) { tableView, indexPath, package in
            let cellID = String(describing: PackageTableViewCell<FFIPackage>.self)
            let cell = tableView.dequeueReusableCell(withIdentifier: cellID, for: indexPath)
            if let cell = cell as? PackageTableViewCell<FFIPackage> {
                cell.package = package
            }

            return cell
        }
    }

    override func configureTableDelegate() -> PackageListDelegate<FFIPackage> {
        DpkgListDelegate(dataSource: dataSource, listController: self)
    }

    func rebuild(packages: [FFIPackage]) {
        guard !packages.isEmpty else { return }

        let hud = Hud.show()
        hud?.text = "rebuild-packages-status-title".localized
        hud?.style = .arcRotate

        let rebuilder = PackagesRebuilder(mainModel: mainModel) { progress in
            Task { @MainActor in
                hud?.detailedText = String(
                    format: "rebuild-packages-status".localized,
                    progress.completedUnitCount,
                    progress.totalUnitCount,
                    Int(progress.fractionCompleted * 100)
                )
            }
        }

        Task {
            await rebuilder.rebuild(packages: packages)
            await hud?.hide()
        }
    }
}
