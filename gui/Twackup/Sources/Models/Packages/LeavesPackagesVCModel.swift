//
//  LeavesPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

struct LeavesPackagesVCModel : PackagesVCModel {
    
    var detailDelegate: PackageDetailDelegate?

    let dpkg: Dpkg

    let navTitle: String = "Leaves"

    var tabbarItem: UITabBarItem {
        return UITabBarItem(title: "Leaves", image: UIImage(systemName: "cube"), tag: 0)
    }

    var packages: [Package] {
        mutating get {
            return _packages
        }
    }

    lazy var _packages: [Package] = {
        return dpkg.parsePackages(leaves: true)
    }()

    init(_ dpkg: Dpkg) {
        self.dpkg = dpkg
    }
}
