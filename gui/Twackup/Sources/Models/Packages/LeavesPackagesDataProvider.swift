//
//  LeavesPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

struct LeavesPackagesDataProvider : PackagesDataProvider {

    let dpkg: Dpkg

    let navTitle: String = "Leaves"

    var tabbarItem: UITabBarItem {
        return UITabBarItem(title: "Leaves", image: UIImage(systemName: "cube"), tag: 0)
    }

    var packages: [Package] {
        mutating get {
            guard let filteredPackages else { return allPackages }
            return filteredPackages
        }
    }

    lazy var allPackages: [Package] = {
        return dpkg.parsePackages(leaves: true)
    }()

    private var filteredPackages: [Package]?

    init(_ dpkg: Dpkg) {
        self.dpkg = dpkg
    }

    mutating func filter(_ filter: PackageFilter?) {
        guard let filter else {
            filteredPackages = nil
            return
        }

        filteredPackages = allPackages.filter({ package in
            switch filter {
            case .Name(let name):
                return package.name.range(of: name, options: .caseInsensitive) != nil
            }
        })
    }
}
