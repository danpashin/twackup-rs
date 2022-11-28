//
//  PackagesVCDataProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import UIKit

protocol PackagesControllerMetadata {
    var navTitle: String { get }

    var tabbarItem: UITabBarItem { get }
}

extension ViewControllers.Package.DataProvider {
    class BasicProvider {
        var packages: [Package] {
            guard let filteredPackages else { return allPackages }
            return filteredPackages
        }

        private(set) var allPackages: [Package]
        private(set) var filteredPackages: [Package]?

        init(packages: [Package]) {
            self.allPackages = packages
        }

        func applyFilter(_ filter: Filter?) {
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
}

extension ViewControllers.Package.DataProvider.BasicProvider {
    enum Filter {
        case name(String)
    }
}
