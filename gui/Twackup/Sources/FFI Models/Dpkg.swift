//
//  PackageParser.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

import Foundation

class Dpkg {

    private let innerDpkg: UnsafeMutablePointer<TwDpkg_t>

    init(path: String = "/var/lib/dpkg", lock: Bool = false) {
        innerDpkg = tw_init(path, false)
    }

    deinit {
        tw_free(innerDpkg)
    }

    func parsePackages(leaves: Bool) -> [Package] {
        let ffiPackages = tw_get_packages(innerDpkg, leaves, TwPackagesSort_t(TW_PACKAGES_SORT_NAME))

        let rawPackages =  UnsafeBufferPointer(start: ffiPackages.ptr, count: ffiPackages.len)

        var packages: [Package] = []
        packages.reserveCapacity(rawPackages.count)

        for package in rawPackages {
            if let packageModel = FFIPackage(package) {
                packages.append(packageModel as any Package)
            }
        }

        rawPackages.deallocate()

        return packages
    }
}
