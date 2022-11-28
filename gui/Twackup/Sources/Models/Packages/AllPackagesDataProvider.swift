//
//  AllPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

struct AllPackagesDataProvider: PackagesDataProvider {

    let dpkg: Dpkg

    let navTitle: String = "All packages"

    var tabbarItem: UITabBarItem {
        return UITabBarItem(title: "All", image: UIImage(systemName: "list.bullet.rectangle"), tag: 0)
    }

    var packages: [Package] {
        guard let filteredPackages else { return allPackages }
        return filteredPackages
    }

    private var allPackages: [Package]
    private var filteredPackages: [Package]?

    init(_ dpkg: Dpkg) {
        self.dpkg = dpkg
        allPackages = dpkg.parsePackages(leaves: false)
    }

    mutating func filter(_ filter: PackageFilter?) {
        guard let filter else {
            filteredPackages = nil
            return
        }

        filteredPackages = allPackages.filter({ package in
            switch filter {
            case .name(let name):
                return package.name.contains(name)
            }
        })
    }
}
