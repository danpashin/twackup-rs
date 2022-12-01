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

        let database: Database

        var hud: RJTHud?

        init(dpkg: Dpkg, database: Database) {
            self.dpkg = dpkg
            self.database = database
            super.init(nibName: nil, bundle: nil)
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        func rebuild(_ package: Package) {
            self.hud = RJTHud.show()
            dpkg.buildDelegate = self
            print("\(String(describing: self))")

            DispatchQueue.global().async {
                let docsDir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
                self.dpkg.rebuild(packages: [package], outDir: docsDir)
            }
        }

        func printMessage(_ message: String, level: Dpkg.MessageLevel) {

        }

        func startProcessing(package: Package) {

        }

        func finishedProcessing(package: Package, debPath: String) {
            print("finished \(debPath)")

            let model = database.createBuildedPackage()
            model.fillFrom(package: package)
            model.debRelativePath = debPath
            model.buildDate = Date()

            self.database.addBuildedPackage(model)
        }

        func finishedAll() {
            self.hud?.hide(animated: true)

            NotificationCenter.default.post(name: kDebsReloadNotification, object: nil)
        }
    }
}
