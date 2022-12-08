//
//  DebsListVC.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

extension PackageVC {
    class DebsListVC: PackageListVC {
        private var debsModel: DebsListModel
        override var model: PackageListModel {
            get { return debsModel }
            set { }
        }

        private lazy var editBarBtn: UIBarButtonItem = {
            UIBarButtonItem(barButtonSystemItem: .edit, target: self, action: #selector(actionEdit))
        }()

        private lazy var editDoneBarBtn: UIBarButtonItem = {
            UIBarButtonItem(barButtonSystemItem: .done, target: self, action: #selector(actionDoneEdit))
        }()

        private lazy var removeAllBarBtn: UIBarButtonItem = {
            let title = Bundle.appLocalize("debs-remove-all-btn")
            return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRemoveAll))
        }()

        private lazy var removeSelectedBarBtn: UIBarButtonItem = {
            let title = Bundle.appLocalize("debs-remove-selected-btn")
            return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRemoveSelected))
        }()

        private lazy var selectAllBarBtn: UIBarButtonItem = {
            let title = Bundle.appLocalize("debs-selectall-btn")
            return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionSelectAll))
        }()

        private var isAnyPackageSelected: Bool = false

        init(model: DebsListModel, detail: PackageVC.DetailVC) {
            debsModel = model
            super.init(model: model, detail: detail)
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        override func viewDidLoad() {
            super.viewDidLoad()
            tableView.allowsMultipleSelectionDuringEditing = true

            let center = NotificationCenter.default
            center.addObserver(forName: DebsListModel.NotificationName, object: nil, queue: .current) { _ in
                self.reload()
            }

            navigationItem.rightBarButtonItem = editBarBtn
        }

        func reload() {
            DispatchQueue.global().async {
                self.debsModel.debsProvider.reload()
                DispatchQueue.main.async {
                    self.reloadTableView()
                }
            }
        }

        override func reloadTableView() {
            super.reloadTableView()
        }

        func tableView(_ tableView: UITableView, didUpdateSelection selected: [IndexPath]?) {
            if isAnyPackageSelected != selected?.isEmpty ?? true { return }
            isAnyPackageSelected = !isAnyPackageSelected

            // 3 items in toolbar - remove all, space and share
            guard var buttons = toolbarItems, buttons.count == 3 else { return }

            buttons[0] = (selected?.isEmpty ?? true) ? removeAllBarBtn : removeSelectedBarBtn

            setToolbarItems(buttons, animated: false)
        }

        @objc func actionShare() {
            guard let selected = selectedPackages else { return }
            let debURLS: [URL] = selected.compactMap({
                guard let package = $0 as? DebPackage else { return nil }
                return package.fileURL()
            })

            let activityViewController = UIActivityViewController(activityItems: debURLS, applicationActivities: nil)
            activityViewController.popoverPresentationController?.sourceView = view

            present(activityViewController, animated: true, completion: nil)
        }

        @objc func actionEdit() {
            tableView.setEditing(true, animated: true)

            navigationItem.leftBarButtonItem = selectAllBarBtn
            navigationItem.rightBarButtonItem = editDoneBarBtn
            navigationController?.setToolbarHidden(false, animated: true)
            setToolbarItems([
                removeAllBarBtn,
                UIBarButtonItem(barButtonSystemItem: .flexibleSpace, target: nil, action: nil),
                UIBarButtonItem(title: Bundle.appLocalize("debs-share-btn"),
                                style: .plain, target: self, action: #selector(actionShare))
            ], animated: false)
        }

        @objc func actionDoneEdit() {
            isAnyPackageSelected = false
            tableView.setEditing(false, animated: true)

            navigationItem.leftBarButtonItem = nil
            navigationItem.rightBarButtonItem = editBarBtn
            navigationController?.setToolbarHidden(true, animated: true)
        }

        @objc func actionRemoveSelected() {
            guard let indexPaths = tableView.indexPathsForSelectedRows else { return }
            if debsModel.debsProvider.deletePackages(at: indexPaths.map({ $0.row })) {
                tableView.deleteRows(at: indexPaths, with: .automatic)
            }
        }

        @objc func actionRemoveAll() {
            var indexPaths: [IndexPath] = []
            for row in 0..<debsModel.dataProvider.packages.count {
                indexPaths.append(IndexPath(row: row, section: 0))
            }

            if debsModel.debsProvider.deletePackages(at: indexPaths.map({ $0.row })) {
                tableView.deleteRows(at: indexPaths, with: .automatic)
            }
        }

        @objc func actionSelectAll() {
            for row in 0..<debsModel.dataProvider.packages.count {
                tableView.selectRow(at: IndexPath(row: row, section: 0), animated: true, scrollPosition: .none)
            }
        }
    }
}
