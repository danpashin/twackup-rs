//
//  DpkgDetailVC.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

class DpkgDetailVC: DetailVC, RebuildPackageDetailedViewDelegate {
    private lazy var _container = RebuildPackageDetailedView(delegate: self)
    override var containerView: PackageDetailedView { _container }

    func rebuild(_ package: Package) {
        let hud = RJTHud.show()

        // swiftlint:disable trailing_closure
        let rebuilder = PackagesRebuilder(dpkg: mainModel.dpkg, database: mainModel.database)
        rebuilder.rebuild(packages: [package], completion: {
            hud?.hide(animated: true)
        })
    }
}
