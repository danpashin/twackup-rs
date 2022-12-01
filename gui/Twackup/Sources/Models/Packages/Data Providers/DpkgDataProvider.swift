//
//  DpkgDataProvider.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//


class DpkgDataProvier: PackageDataProvider {
    let dpkg: Dpkg

    init(_ dpkg: Dpkg, leaves: Bool = false) {
        self.dpkg = dpkg

        super.init(packages: dpkg.parsePackages(leaves: leaves))
    }
}
