//
//  BuildedPackagesDataProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import UIKit

extension ViewControllers.Package.Metadata {
    struct BuildedPkgsVC: PackagesControllerMetadata {
        let navTitle: String = "DEBs"

        var tabbarItem: UITabBarItem {
            return UITabBarItem(title: "DEBs", image: UIImage(systemName: "list.bullet.rectangle"), tag: 0)
        }
    }
}

extension ViewControllers.Package.DataProvider {
    class BuildedPkgsVC: BasicProvider {
        private let database: Database

        init(_ database: Database) {
            self.database = database

            super.init(packages: database.fetchBuildedPackages())
        }
    }
}
