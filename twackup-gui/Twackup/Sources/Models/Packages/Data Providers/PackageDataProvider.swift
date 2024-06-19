//
//  PackageDataProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

class PackageDataProvider<P: Package>: @unchecked Sendable {
    enum Filter {
        case name(String)
    }

    var packages: [P] {
        guard let filteredPackages else { return allPackages }
        return filteredPackages
    }

    @UnfairLockWrap var allPackages: [P] {
        didSet {
            applyFilter(currentFilter)
        }
    }

    @UnfairLockWrap private var filteredPackages: [P]?

    @UnfairLockWrap private(set) var currentFilter: Filter?

    init(packages: [P] = []) {
        allPackages = packages
    }

    func applyFilter(_ filter: Filter?) {
        currentFilter = filter
        guard let filter else {
            filteredPackages = nil
            return
        }

        filteredPackages = allPackages.filter { package in
            switch filter {
            case .name(let name):
                return package.name.localizedCaseInsensitiveContains(name)
            }
        }
    }

    func reload() async throws {
    }
}
