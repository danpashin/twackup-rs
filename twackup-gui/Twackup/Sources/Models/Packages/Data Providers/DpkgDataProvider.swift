//
//  DpkgDataProvider.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

import Sentry

class DpkgDataProvier: PackageDataProvider {
    let dpkg: Dpkg

    let onlyLeaves: Bool

    init(_ dpkg: Dpkg, leaves: Bool = false) {
        self.dpkg = dpkg
        onlyLeaves = leaves

        super.init(packages: [])
    }

    func reload(completion: (() -> Void)? = nil) {
        DispatchQueue.global(qos: .userInitiated).async { [self] in
            let transaction = SentrySDK.startTransaction(name: "database-parse", operation: "lib")

            do {
                allPackages = try dpkg.parsePackages(onlyLeaves: onlyLeaves)
            } catch {
                FFILogger.shared.log("\(error)", level: .error)
                SentrySDK.capture(error: error)
            }

            transaction.finish()

            completion?()
        }
    }
}
