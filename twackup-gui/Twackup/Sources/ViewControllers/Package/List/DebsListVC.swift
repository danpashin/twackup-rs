//
//  DebsListVC.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

class DebsListVC: SelectablePackageListVC, DebsListModelDelegate {
    private var debsModel: DebsListModel
    override var model: PackageListModel {
        get { return debsModel }
        set { }
    }

    private lazy var removeAllBarBtn: UIBarButtonItem = {
        let title = "debs-remove-all-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRemoveAll))
    }()

    private lazy var removeSelectedBarBtn: UIBarButtonItem = {
        let title = "debs-remove-selected-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRemoveSelected))
    }()

    private lazy var shareSelectedBarBtn: UIBarButtonItem = {
        let title = "debs-share-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionShareSelected))
    }()

    private var reloadObserver: NSObjectProtocol?

    init(model: DebsListModel, detail: PackageDetailVC) {
        debsModel = model
        super.init(model: model, detail: detail)

        debsModel.debsModelDelegate = self

        reloadObserver = NotificationCenter.default.addObserver(
            forName: DebsListModel.NotificationName,
            object: nil,
            queue: .current
        ) { [weak self] _  in
            self?.reloadData()
        }
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    deinit {
        NotificationCenter.default.removeObserver(reloadObserver as Any)
    }

    override func reloadData() {
        debsModel.debsProvider.reload {
            super.reloadData()
        }
    }

    override func didSelect(items: [PackageListModel.TableViewItem], inEditState: Bool) {
        super.didSelect(items: items, inEditState: inEditState)

        if inEditState {
            shareSelectedBarBtn.isEnabled = !items.isEmpty

            guard var buttons = toolbarItems, !buttons.isEmpty else { return }
            buttons[0] = items.isEmpty ? removeAllBarBtn : removeSelectedBarBtn
            setToolbarItems(buttons, animated: false)
        }
    }

    @objc
    func actionShareSelected(_ button: UIBarButtonItem) {
        let debURLS: [URL] = model.selectedItems.compactMap { item in
            guard let package = item.package as? DebPackage else { return nil }
            return package.fileURL
        }

        if debURLS.isEmpty { return }

        let activityVC = UIActivityViewController(activityItems: debURLS, applicationActivities: nil)
        activityVC.popoverPresentationController?.barButtonItem = button

        present(activityVC, animated: true, completion: nil)
    }

    override func actionEdit() {
        super.actionEdit()

        shareSelectedBarBtn.isEnabled = false

        setToolbarItems([
            removeAllBarBtn,
            UIBarButtonItem(barButtonSystemItem: .flexibleSpace, target: nil, action: nil),
            shareSelectedBarBtn
        ], animated: false)
        navigationController?.setToolbarHidden(false, animated: true)
    }

    override func actionDoneEdit() {
        super.actionDoneEdit()

        navigationController?.setToolbarHidden(true, animated: true)
    }

    @objc
    func actionRemoveSelected() {
        guard let indexPaths = tableView.indexPathsForSelectedRows else { return }
        if debsModel.debsProvider.deletePackages(at: indexPaths.map({ $0.row })) {
            tableView.deleteRows(at: indexPaths, with: .automatic)
            endReloadingData()
        }
    }

    @objc
    func actionRemoveAll() {
        actionDoneEdit()

        var indexPaths: [IndexPath] = []
        for row in 0..<debsModel.dataProvider.packages.count {
            indexPaths.append(IndexPath(row: row, section: 0))
        }

        if debsModel.debsProvider.deletePackages(at: indexPaths.map({ $0.row })) {
            tableView.deleteRows(at: indexPaths, with: .automatic)
            endReloadingData()
        }
    }

    // MARK: - DebsListModelDelegate

    func debsModel(
        _ debsModel: DebsListModel,
        didRecieveDebRemoveChallenge package: DebPackage,
        completion: @escaping (_ allow: Bool) -> Void
    ) {
        let alert = UIAlertController(
            title: "deb-remove-alert-title".localized,
            message: "deb-remove-alert-subtitle".localized,
            preferredStyle: .alert
        )

        alert.addAction(UIAlertAction(title: "deb-remove-alert-ok".localized, style: .destructive) { _ in
            completion(true)
        })

        alert.addAction(UIAlertAction(title: "cancel".localized, style: .cancel))

        present(alert, animated: true)
    }
}
