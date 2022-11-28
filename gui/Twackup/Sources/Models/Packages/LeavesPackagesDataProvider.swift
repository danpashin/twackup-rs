//
//  LeavesPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

extension ViewControllers.Package.Metadata {
    struct LeavesPkgsVC: PackagesControllerMetadata {
        let navTitle: String = "Leaves"

        var tabbarItem: UITabBarItem {
            return UITabBarItem(title: "Leaves", image: UIImage(systemName: "cube"), tag: 0)
        }
    }
}

extension ViewControllers.Package.DataProvider {
    class LeavesPkgsVC: BasicProvider {
        private let dpkg: Dpkg

        init(_ dpkg: Dpkg) {
            self.dpkg = dpkg

            super.init(packages: dpkg.parsePackages(leaves: true))
        }
    }
}
