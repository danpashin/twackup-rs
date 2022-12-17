//
//  DpkgListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

extension PackageVC {
    class DpkgListVC: PackageSelectableListVC {
        private var isAnyPackageSelected: Bool = false

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

            let spinner = UIActivityIndicatorView(style: .large)
            tableView.backgroundView = spinner
            spinner.startAnimating()

            dpkgModel.dpkgProvider.reload {
                DispatchQueue.main.async { [self] in
                    spinner.stopAnimating()
                    tableView.reloadData()
                }
            }
        }

        func tableView(_ tableView: UITableView, didUpdateSelection selected: [IndexPath]?) {
            if isAnyPackageSelected != selected?.isEmpty ?? true { return }
            isAnyPackageSelected.toggle()

            guard var buttons = toolbarItems, !buttons.isEmpty else { return }
            buttons[0] = (selected?.isEmpty ?? true) ? rebuildAllBarBtn : rebuildSelectedBarBtn
            setToolbarItems(buttons, animated: false)
        }

        override func actionEdit() {
            super.actionEdit()

            setToolbarItems([rebuildAllBarBtn], animated: false)
            navigationController?.setToolbarHidden(false, animated: true)
        }

        override func actionDoneEdit() {
            super.actionDoneEdit()
            isAnyPackageSelected = false

            navigationItem.leftBarButtonItem = nil
            navigationController?.setToolbarHidden(true, animated: true)
        }

        @objc
        func actionRebuildSelected() {
            guard let selectedPackages else { return }
            rebuild(packages: selectedPackages)
        }

        @objc
        func actionRebuildAll() {
            actionDoneEdit()

            rebuild(packages: model.dataProvider.packages)
        }

        func rebuild(packages: [Package]) {
            guard !packages.isEmpty else { return }
            let hud = RJTHud.show()
            hud?.text = "rebuild-packages-status-title".localized
            hud?.style = .spinner

            let rebuilder = PackagesRebuilder(dpkg: model.mainModel.dpkg, database: model.mainModel.database)
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
}
