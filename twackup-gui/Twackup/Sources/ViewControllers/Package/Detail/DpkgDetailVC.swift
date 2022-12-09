//
//  DpkgDetailVC.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

extension PackageVC {
    class DpkgDetailVC: DetailVC, RebuildPackageDetailedViewDelegate {
        private lazy var _container = RebuildPackageDetailedView(delegate: self)
        override var containerView: PackageDetailedView { _container }

        let dpkg: Dpkg

        init(dpkg: Dpkg, database: Database) {
            self.dpkg = dpkg
            super.init(database: database)
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        func rebuild(_ package: Package) {
            let hud = RJTHud.show()

            let rebuilder = PackagesRebuilder(dpkg: dpkg, database: database)
            rebuilder.rebuild(packages: [package]) {
                hud?.hide(animated: true)
            }
        }
    }
}
