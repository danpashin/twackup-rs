//
//  DpkgDataProvider.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

import Sentry

class DpkgDataProvier: PackageDataProvider {
    let dpkg: Dpkg

    init(_ dpkg: Dpkg, leaves: Bool = false) {
        self.dpkg = dpkg

        let transaction = SentrySDK.startTransaction(name: "database-parse", operation: "lib")

        super.init(packages: dpkg.parsePackages(leaves: leaves))

        transaction.finish()
    }
}
