//
//  DatabasePackageProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

class DatabasePackageProvider: PackageDataProvider, @unchecked Sendable {
    private let database: Database

    init(_ database: Database) {
        self.database = database

        super.init()
    }

    func reload() async throws {
        allPackages = try await database.fetchPackages()
    }

    func deletePackages(at indexes: [Int]) async -> Bool {
        let toDelete = packages.enumerated().filter { indexes.contains($0.offset) }.map { $0.element }
        if toDelete.isEmpty {
            return false
        }

        // refactor to use of SET
        allPackages = allPackages.filter { package in
            !toDelete.contains { $0.isEqualTo(package) }
        }

        for package in toDelete {
            guard let dbPackage = package.asDEB else { continue }
            do {
                try FileManager.default.removeItem(at: dbPackage.fileURL)
            } catch {
                await FFILogger.shared.log(error.localizedDescription, level: .warning)
            }
        }

        await database.delete(packages: toDelete)
        applyFilter(currentFilter)

        return true
    }

    func deletePackage(at index: Int) async -> Bool {
        await deletePackages(at: [index])
    }

    func deleteAll() async -> Bool {
        await deletePackages(at: allPackages.indices.map { $0 })
    }
}
