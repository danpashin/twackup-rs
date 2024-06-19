//
//  DatabasePackageProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

class DatabasePackageProvider: PackageDataProvider<DebPackage>, @unchecked Sendable {
    private let database: Database

    init(_ database: Database) {
        self.database = database

        super.init()
    }

    override func reload() async throws {
        allPackages = try await database.fetchPackages()
    }

    func deletePackages(at indexes: [Int]) async throws {
        let toDelete = packages.enumerated().filter { indexes.contains($0.offset) }.map(\.element)
        if toDelete.isEmpty {
            return
        }

        try await delete(packages: toDelete)
    }

    func deletePackage(at index: Int) async throws {
        try await deletePackages(at: [index])
    }

    func delete(packages: [DebPackage]) async throws {
        let snapshot = Set(allPackages).subtracting(Set(packages))
        try await database.delete(packages: packages)

        allPackages = snapshot.sorted(by: \.name)
        applyFilter(currentFilter)
    }

    func deleteAll() async throws {
        try await delete(packages: allPackages)
    }
}
