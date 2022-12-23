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

        super.init()
    }

    func reload(completion: (() -> Void)? = nil) {
        DispatchQueue.global(qos: .userInitiated).async { [self] in
            allPackages = database.fetchBuildedPackages()
            completion?()
        }
    }

    func deletePackages(at indexes: [Int], completion: (() -> Void)? = nil) -> Bool {
        let toDelete = packages.enumerated().filter { indexes.contains($0.offset) }.map { $0.element }
        if toDelete.isEmpty {
            completion?()
            return false
        }

        // refactor to use of SET
        allPackages = allPackages.filter { package in
            !toDelete.contains { $0.isEqualTo(package) }
        }

        for package in toDelete {
            guard let dbPackage = package as? DebPackage else { continue }
            do {
                try FileManager.default.removeItem(at: dbPackage.fileURL)
            } catch {
                let err = error as NSError
                FFILogger.shared.log(err.localizedDescription, level: .warning)
            }
        }

        database.delete(packages: toDelete, completion: completion)
        applyFilter(currentFilter)

        return true
    }

    func deletePackage(at index: Int, completion: (() -> Void)? = nil) -> Bool {
        deletePackages(at: [index], completion: completion)
    }

    func deleteAll(completion: (() -> Void)? = nil) -> Bool {
        deletePackages(at: allPackages.enumerated().map { $0.offset }, completion: completion)
    }
}
