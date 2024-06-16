//
//  Database.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

@preconcurrency import CoreData
import os

actor Database {
    private let persistentContainer: NSPersistentContainer

    var backroundContext: NSManagedObjectContext {
        let context = persistentContainer.newBackgroundContext()
        context.mergePolicy = NSMergeByPropertyObjectTrumpMergePolicy

        return context
    }

    init() {
        persistentContainer = NSPersistentContainer(name: "Twackup")
        persistentContainer.viewContext.mergePolicy = NSMergeByPropertyObjectTrumpMergePolicy
        persistentContainer.loadPersistentStores { _, error in
            guard let error else { return }

            Task(priority: .utility) {
                await FFILogger.shared.log("Unresolved error \(error),\(error.localizedDescription)")
            }
        }
    }

    private func execute(
        request: NSPersistentStoreRequest,
        context: NSManagedObjectContext
    ) async -> NSPersistentStoreResult? {
        do {
            let result = try context.execute(request)
            try context.save()
            return result
        } catch {
            context.rollback()
            await FFILogger.shared.log("Unresolved error \(error), \(error.localizedDescription)")
        }

        return nil
    }

    func add(packages: [BuildedPackage]) async {
        var index = 0
        let total = packages.count

        let request = NSBatchInsertRequest(entity: DebPackageObject.entity()) { (object: NSManagedObject) in
            guard index < total else { return true }

            if let object = object as? DebPackageObject {
                let package = packages[index]

                object.fillFrom(package: package.package)
                object.fillFrom(file: package.debURL)
            }

            index += 1
            return false
        }

        _ = await execute(request: request, context: backroundContext)
    }

    func fetchPackages() throws -> [DebPackage] {
        let request = DebPackageObject.fetchRequest()
        return try backroundContext.fetch(request).map { DebPackage(object: $0) }
    }

    func fetch(package: Package) throws -> DebPackage? {
        let request = DebPackageObject.fetchRequest(package: package)
        guard let object = try backroundContext.fetch(request).first else {
            return nil
        }

        return DebPackage(object: object)
    }

    func delete(package: DebPackage) async throws {
        try await delete(packages: [package])
    }

    func delete(packages: [DebPackage]) async throws {
        if packages.isEmpty {
            return
        }

        for pkg in packages {
            let manager = FileManager.default
            let path = pkg.fileURL.path
            if manager.fileExists(atPath: path) {
                try manager.removeItem(atPath: path)
            }
        }

        let request = NSBatchDeleteRequest(objectIDs: packages.map { $0.databaseID })
        _ = await execute(request: request, context: backroundContext)
    }

    func delete(packages: [Package]) async throws {
        try await delete(packages: packages.compactMap { $0.asDEB })
    }

    func packagesSize() -> Int64 {
        let request = DebPackageObject.debsSizeRequest()
        guard let results = try? backroundContext.fetch(request) as? [[String: Int64]] else {
            return 0
        }

        return results.first?["totalSize"] ?? 0
    }

    func databaseSize() -> Int64 {
        var size: Int64 = 0

        for store in persistentContainer.persistentStoreCoordinator.persistentStores {
            guard let url = store.url, url.isFileURL else { continue }
            guard let attributes = try? FileManager.default.attributesOfItem(atPath: url.path) else { continue }

            size += (attributes[.size] as? Int64) ?? 0
        }

        return size
    }
}
