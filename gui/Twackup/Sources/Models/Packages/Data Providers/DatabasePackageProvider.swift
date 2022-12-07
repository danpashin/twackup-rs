//
//  DatabasePackageProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

class DatabasePackageProvider: PackageDataProvider {
    private let database: Database

    init(_ database: Database) {
        self.database = database

        super.init(packages: database.fetchBuildedPackages())
    }

    func reload() {
        allPackages = database.fetchBuildedPackages()
    }

    func deletePackage(at index: Int) {
        let package = allPackages.remove(at: index)
        guard let dbPackage = database.fetch(package: package) else { return }

        try? FileManager.default.removeItem(at: dbPackage.fileURL())
        database.delete(package: dbPackage)
    }
}
