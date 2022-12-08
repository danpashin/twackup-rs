//
//  DpkgDetailVC.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

extension PackageVC {
    class DpkgDetailVC: DetailVC, DpkgBuildDelegate, RebuildPackageDetailedViewDelegate {
        private lazy var _container = RebuildPackageDetailedView(delegate: self)
        override var containerView: PackageDetailedView { _container }

        let dpkg: Dpkg

        var hud: RJTHud?

        init(dpkg: Dpkg, database: Database) {
            self.dpkg = dpkg
            super.init(database: database)
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        func rebuild(_ package: Package) {
            hud = RJTHud.show()
            dpkg.buildDelegate = self

            DispatchQueue.global().async {
                self.dpkg.rebuild(packages: [package])
            }
        }

        func printMessage(_ message: String, level: Dpkg.MessageLevel) {

        }

        func startProcessing(package: Package) {

        }

        func finishedProcessing(package: Package, debPath: URL) {
            let model = database.createBuildedPackage()
            model.setProperties(package: package)
            model.setProperties(file: debPath, pathRelativeTo: Dpkg.defaultSaveDirectory)

            database.addBuildedPackage(model)
        }

        func finishedAll() {
            hud?.hide(animated: true)

            NotificationCenter.default.post(name: DebsListModel.NotificationName, object: nil)
        }
    }
}
