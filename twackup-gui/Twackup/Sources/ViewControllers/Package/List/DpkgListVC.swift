//
//  DpkgListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

class DpkgListVC: SelectablePackageListVC {
    let dpkgModel: DpkgListModel

    private lazy var rebuildAllBarBtn: UIBarButtonItem = {
        let title = "debs-rebuildall-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRebuildAll))
    }()

    private lazy var rebuildSelectedBarBtn: UIBarButtonItem = {
        let title = "debs-rebuildselected-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRebuildSelected))
    }()

    init(model: DpkgListModel, detail: PackageDetailVC) {
        dpkgModel = model
        super.init(model: model, detail: detail)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        let refresh = UIRefreshControl()
        refresh.addTarget(self, action: #selector(actionRefresh), for: .valueChanged)

        tableView.refreshControl = refresh
    }

    override func reloadData() {
        dpkgModel.dpkgProvider.reload {
            super.reloadData()
        }
    }

    override func endReloadingData() {
        super.endReloadingData()

        // Twackup parses database so quick that user can think it is not working as he expects
        // That's why there's half-of-a-second delay
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [self] in
            tableView.refreshControl?.endRefreshing()
        }
    }

    override func didSelect(items: [PackageListModel.TableViewItem], inEditState: Bool) {
        super.didSelect(items: items, inEditState: inEditState)

        if inEditState {
            guard var buttons = toolbarItems, !buttons.isEmpty else { return }
            buttons[0] = items.isEmpty ? rebuildAllBarBtn : rebuildSelectedBarBtn
            setToolbarItems(buttons, animated: false)
        }
    }

    // MARK: - Actions

    override func setEditing(_ editing: Bool, animated: Bool) {
        super.setEditing(editing, animated: animated)

        if editing {
            setToolbarItems([rebuildAllBarBtn], animated: false)
        } else {
            navigationItem.leftBarButtonItem = nil
        }

        navigationController?.setToolbarHidden(!editing, animated: animated)
    }

    @objc
    func actionRebuildSelected() {
        rebuild(packages: model.selectedItems.map { $0.package })
    }

    @objc
    func actionRebuildAll() {
        setEditing(false, animated: true)

        rebuild(packages: model.dataProvider.packages)
    }

    @objc
    func actionRefresh() {
        tableView.refreshControl?.beginRefreshing()
        reloadData()
    }

    func rebuild(packages: [Package]) {
        guard !packages.isEmpty else { return }

        let hud = RJTHud.show()
        hud?.text = "rebuild-packages-status-title".localized
        hud?.style = .spinner

        let rebuilder = PackagesRebuilder(mainModel: model.mainModel)
        rebuilder.rebuild(packages: packages.compactMap { $0 as? FFIPackage }) { progress in
            hud?.detailedText = String(
                format: "rebuild-packages-status".localized,
                progress.completedUnitCount,
                progress.totalUnitCount,
                Int(progress.fractionCompleted * 100)
            )
        } completion: {
            hud?.hide(animated: true)
        }
    }
}
