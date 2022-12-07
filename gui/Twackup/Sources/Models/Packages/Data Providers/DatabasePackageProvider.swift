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

    func deletePackage(at index: Int) -> Bool {
        var package: Package!
        if self.currentFilter == nil {
            package = allPackages.remove(at: index)
        } else {
            // Find package to delete in filtered
            // Then find it in allPackages
            // and finally delete
            guard let pkgs = filteredPackages,
                  let toRemove = allPackages.enumerated().first(where: { $1.isEqualTo(pkgs[index]) })
            else { return false }

            package = allPackages.remove(at: toRemove.offset)
        }

        guard let dbPackage = database.fetch(package: package),
              (try? FileManager.default.removeItem(at: dbPackage.fileURL())) != nil
        else { return false }

        database.delete(package: dbPackage)

        // reload filter
        applyFilter(currentFilter)

        return true
    }
}
