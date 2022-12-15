//
//  DebsListVC.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

extension PackageVC {
    class DebsListVC: PackageSelectableListVC {
        private var debsModel: DebsListModel
        override var model: PackageListModel {
            get { return debsModel }
            set { }
        }

        private lazy var removeAllBarBtn: UIBarButtonItem = {
            let title = Bundle.appLocalize("debs-remove-all-btn")
            return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRemoveAll))
        }()

        private lazy var removeSelectedBarBtn: UIBarButtonItem = {
            let title = Bundle.appLocalize("debs-remove-selected-btn")
            return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRemoveSelected))
        }()

        private var isAnyPackageSelected: Bool = false

        private var reloadObserver: NSObjectProtocol?

        init(model: DebsListModel, detail: PackageVC.DetailVC) {
            debsModel = model
            super.init(model: model, detail: detail)
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        override func viewDidLoad() {
            super.viewDidLoad()

            reloadObserver = NotificationCenter.default.addObserver(
                forName: DebsListModel.NotificationName,
                object: nil,
                queue: .current
            ) { [weak self] _  in
                guard let self else { return }
                self.debsModel.debsProvider.reload()
                DispatchQueue.main.async {
                    self.reloadTableView()
                }
            }
        }

        deinit {
            NotificationCenter.default.removeObserver(reloadObserver as Any)
        }

        func tableView(_ tableView: UITableView, didUpdateSelection selected: [IndexPath]?) {
            if isAnyPackageSelected != selected?.isEmpty ?? true { return }
            isAnyPackageSelected.toggle()

            guard var buttons = toolbarItems, !buttons.isEmpty else { return }
            buttons[0] = (selected?.isEmpty ?? true) ? removeAllBarBtn : removeSelectedBarBtn
            setToolbarItems(buttons, animated: false)
        }

        @objc
        func actionShare() {
            guard let selected = selectedPackages else { return }
            let debURLS: [URL] = selected.compactMap { package in
                guard let package = package as? DebPackage else { return nil }
                return package.fileURL()
            }

            let activityViewController = UIActivityViewController(activityItems: debURLS, applicationActivities: nil)
            activityViewController.popoverPresentationController?.sourceView = view

            present(activityViewController, animated: true, completion: nil)
        }

        override func actionEdit() {
            super.actionEdit()

            setToolbarItems([
                removeAllBarBtn,
                UIBarButtonItem(barButtonSystemItem: .flexibleSpace, target: nil, action: nil),
                UIBarButtonItem(
                    title: Bundle.appLocalize("debs-share-btn"),
                    style: .plain,
                    target: self,
                    action: #selector(actionShare)
                )
            ], animated: false)
            navigationController?.setToolbarHidden(false, animated: true)
        }

        override func actionDoneEdit() {
            super.actionDoneEdit()
            isAnyPackageSelected = false

            navigationController?.setToolbarHidden(true, animated: true)
        }

        @objc
        func actionRemoveSelected() {
            guard let indexPaths = tableView.indexPathsForSelectedRows else { return }
            if debsModel.debsProvider.deletePackages(at: indexPaths.map({ $0.row })) {
                tableView.deleteRows(at: indexPaths, with: .automatic)
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
            }
        }
    }
}
