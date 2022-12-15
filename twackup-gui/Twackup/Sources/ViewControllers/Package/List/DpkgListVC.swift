//
//  DpkgListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import Foundation

extension PackageVC {
    class DpkgListVC: PackageSelectableListVC {

        let dpkg: Dpkg

        let database: Database

        private var isAnyPackageSelected: Bool = false

        private lazy var rebuildAllBarBtn: UIBarButtonItem = {
            let title = Bundle.appLocalize("debs-rebuildall-btn")
            return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRebuildAll))
        }()

        private lazy var rebuildSelectedBarBtn: UIBarButtonItem = {
            let title = Bundle.appLocalize("debs-rebuildselected-btn")
            return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionRebuildSelected))
        }()

        init(dpkg: Dpkg, database: Database, model: PackageListModel, detail: PackageVC.DetailVC) {
            self.dpkg = dpkg
            self.database = database
            super.init(model: model, detail: detail)
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        func tableView(_ tableView: UITableView, didUpdateSelection selected: [IndexPath]?) {
            if isAnyPackageSelected != selected?.isEmpty ?? true { return }
            isAnyPackageSelected = !isAnyPackageSelected

            guard var buttons = toolbarItems, !buttons.isEmpty else { return }
            buttons[0] = (selected?.isEmpty ?? true) ? rebuildAllBarBtn : rebuildSelectedBarBtn
            setToolbarItems(buttons, animated: false)
        }

        override func actionEdit() {
            super.actionEdit()

            setToolbarItems([
                rebuildAllBarBtn,
                UIBarButtonItem(barButtonSystemItem: .flexibleSpace, target: nil, action: nil)
            ], animated: false)
            navigationController?.setToolbarHidden(false, animated: true)
        }

        override func actionDoneEdit() {
            super.actionDoneEdit()
            isAnyPackageSelected = false

            navigationItem.leftBarButtonItem = nil
            navigationController?.setToolbarHidden(true, animated: true)
        }

        @objc func actionRebuildSelected() {
            guard let selectedPackages else { return }
            rebuild(packages: selectedPackages)
        }

        @objc func actionRebuildAll() {
            actionDoneEdit()

            rebuild(packages: model.dataProvider.packages)
        }

        func rebuild(packages: [Package]) {
            guard !packages.isEmpty else { return }
            let hud = RJTHud.show()

            let rebuilder = PackagesRebuilder(dpkg: dpkg, database: database)
            rebuilder.rebuild(packages: packages) {
                hud?.hide(animated: true)
            }
        }
    }
}
