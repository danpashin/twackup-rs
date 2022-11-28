//
//  BuildedPackagesDataProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import UIKit

struct BuildedPackagesDataProvider: PackagesDataProvider {

    let database: Database

    let navTitle: String = "DEBs"

    var tabbarItem: UITabBarItem {
        return UITabBarItem(title: "DEBs", image: UIImage(systemName: "list.bullet.rectangle"), tag: 0)
    }

    var packages: [Package] {
        guard let filteredPackages else { return allPackages }
        return filteredPackages
    }

    private var allPackages: [Package]
    private var filteredPackages: [Package]?

    init(_ database: Database) {
        self.database = database
        allPackages = database.fetchBuildedPackages()
        print(allPackages)
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
