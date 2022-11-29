//
//  PackagesVCDataProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import UIKit

extension PackageVC {
    class DataProvider {
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
extension PackageVC.DataProvider {
    enum Filter {
        case name(String)
    }
}

extension PackageVC {
    class DpkgProvier: DataProvider {
        let dpkg: Dpkg

        init(_ dpkg: Dpkg, leaves: Bool = false) {
            self.dpkg = dpkg

            super.init(packages: dpkg.parsePackages(leaves: leaves))
        }
    }
}
