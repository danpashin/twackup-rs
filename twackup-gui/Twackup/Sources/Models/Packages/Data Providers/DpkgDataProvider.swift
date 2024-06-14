//
//  DpkgDataProvider.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

import Sentry

class DpkgDataProvier: PackageDataProvider, @unchecked Sendable {
    let dpkg: Dpkg

    let onlyLeaves: Bool

    init(_ dpkg: Dpkg, leaves: Bool = false) {
        self.dpkg = dpkg
        onlyLeaves = leaves

        super.init(packages: [])
    }

    func reload() async {
        let transaction = SentrySDK.startTransaction(name: "database-parse", operation: "lib")

        do {
            allPackages = try await dpkg.parsePackages(onlyLeaves: onlyLeaves)
        } catch {
            await FFILogger.shared.log("Error \(error): \(error.localizedDescription)", level: .error)
            SentrySDK.capture(error: error)
        }

        transaction.finish()
    }
}
