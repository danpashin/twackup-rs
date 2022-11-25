//
//  AllPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

struct AllPackagesVCModel : PackagesVCModel {
    var detailDelegate: PackageDetailDelegate?
    
    let dpkg: Dpkg

    let navTitle: String = "All packages"

    var tabbarItem: UITabBarItem {
        return UITabBarItem(title: "All", image: UIImage(systemName: "list.bullet.rectangle"), tag: 0)
    }

    var packages: [Package] {
        mutating get {
            return _packages
        }
    }

    lazy var _packages: [Package] = {
        return dpkg.parsePackages(leaves: false)
    }()

    init(_ dpkg: Dpkg) {
        self.dpkg = dpkg
    }
}
