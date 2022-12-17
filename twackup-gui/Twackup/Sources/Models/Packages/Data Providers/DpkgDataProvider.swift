//
//  DpkgDataProvider.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

import Sentry

class DpkgDataProvier: PackageDataProvider {
    let dpkg: Dpkg

    let leavesOnly: Bool

    init(_ dpkg: Dpkg, leaves: Bool = false) {
        self.dpkg = dpkg
        leavesOnly = leaves

        super.init(packages: [])
    }

    func reload(completion: (() -> Void)? = nil) {
        DispatchQueue.global(qos: .userInitiated).async { [self] in
            let transaction = SentrySDK.startTransaction(name: "database-parse", operation: "lib")

            allPackages = dpkg.parsePackages(leaves: leavesOnly)

            transaction.finish()

            completion?()
        }
    }
}
