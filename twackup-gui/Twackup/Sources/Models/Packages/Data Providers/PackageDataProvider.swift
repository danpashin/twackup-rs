//
//  PackageDataProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

class PackageDataProvider {
    enum Filter {
        case name(String)
    }

    var packages: [Package] {
        guard let filteredPackages else { return allPackages }
        return filteredPackages
    }

    var allPackages: [Package] {
        didSet {
            applyFilter(currentFilter)
        }
    }

    private(set) var filteredPackages: [Package]?

    private(set) var currentFilter: Filter?

    init(packages: [Package]) {
        self.allPackages = packages
    }

    func applyFilter(_ filter: Filter?) {
        currentFilter = filter
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
