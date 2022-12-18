//
//  DpkgListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

class DpkgListVC: PackageSelectableListVC {
    let dpkgModel: DpkgListModel

    private lazy var rebuildAllBarBtn: UIBarButtonItem = {
        let title = "debs-rebuildall-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRebuildAll))
    }()

    private lazy var rebuildSelectedBarBtn: UIBarButtonItem = {
        let title = "debs-rebuildselected-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRebuildSelected))
    }()

    init(model: DpkgListModel, detail: DetailVC) {
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

    override func actionEdit() {
        super.actionEdit()

        setToolbarItems([rebuildAllBarBtn], animated: false)
        navigationController?.setToolbarHidden(false, animated: true)
    }

    override func actionDoneEdit() {
        super.actionDoneEdit()

        navigationItem.leftBarButtonItem = nil
        navigationController?.setToolbarHidden(true, animated: true)
    }

    @objc
    func actionRebuildSelected() {
        rebuild(packages: model.selectedItems.map { $0.package })
    }

    @objc
    func actionRebuildAll() {
        actionDoneEdit()

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
        rebuilder.rebuild(packages: packages) { progress in
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
