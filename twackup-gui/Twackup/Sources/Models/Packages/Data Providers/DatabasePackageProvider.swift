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

    func deletePackages(at indexes: [Int]) -> Bool {
        let toDelete = packages.enumerated().filter({ indexes.contains($0.offset) }).map({ $0.element })

        // refactor to use of SET
        allPackages = allPackages.filter({ package in
            !toDelete.contains(where: { $0.isEqualTo(package) })
        })

        for package in toDelete {
            guard let dbPackage = package as? DebPackage else { continue }
            do {
                try FileManager.default.removeItem(at: dbPackage.fileURL())
            } catch {
                print("\(error)")
            }
        }

        database.delete(packages: toDelete)

        // reload filter
        applyFilter(currentFilter)

        return true
    }

    func deletePackage(at index: Int) -> Bool {
        deletePackages(at: [index])
    }
}
