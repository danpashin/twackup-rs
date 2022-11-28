//
//  AllPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

extension ViewControllers.Package.Metadata {
    struct AllPkgsVC: PackagesControllerMetadata {
        let navTitle: String = "All packages"

        var tabbarItem: UITabBarItem {
            return UITabBarItem(title: "All", image: UIImage(systemName: "list.bullet.rectangle"), tag: 0)
        }
    }
}

extension ViewControllers.Package.DataProvider {
    class AllPkgsVC: BasicProvider {
        private let dpkg: Dpkg

        init(_ dpkg: Dpkg) {
            self.dpkg = dpkg

            super.init(packages: dpkg.parsePackages(leaves: false))
        }
    }
}
