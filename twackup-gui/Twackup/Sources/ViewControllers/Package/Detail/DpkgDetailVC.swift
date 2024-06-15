//
//  DpkgDetailVC.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

class DpkgDetailVC: PackageDetailVC, RebuildPackageDetailedViewDelegate {
    private lazy var _container = RebuildPackageDetailedView(delegate: self)
    override var detailView: PackageDetailedView { _container }

    nonisolated func rebuild(_ package: FFIPackage) {
        Task {
            let hud = await Hud.show()

            let rebuilder = PackagesRebuilder(mainModel: mainModel)
            await rebuilder.rebuild(packages: [package])

            await hud?.hide(animated: true)
        }
    }
}
