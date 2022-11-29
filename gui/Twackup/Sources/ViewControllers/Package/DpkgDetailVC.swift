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

        func rebuild(_ package: Package) {

        }
    }
}
