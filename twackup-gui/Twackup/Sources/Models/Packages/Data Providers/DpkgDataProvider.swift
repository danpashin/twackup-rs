//
//  DpkgDataProvider.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

import Sentry

class DpkgDataProvier: PackageDataProvider<FFIPackage>, @unchecked Sendable {
    let dpkg: Dpkg

    let onlyLeaves: Bool

    init(_ dpkg: Dpkg, leaves: Bool = false) {
        self.dpkg = dpkg
        onlyLeaves = leaves

        super.init(packages: [])
    }

    override func reload() async throws {
        do {
            let transaction = SentrySDK.startTransaction(name: "database-parse", operation: "lib")
            defer { transaction.finish() }

            allPackages = try await dpkg.parsePackages(onlyLeaves: onlyLeaves)
        } catch {
            SentrySDK.capture(error: error)
            throw error
        }
    }
}
