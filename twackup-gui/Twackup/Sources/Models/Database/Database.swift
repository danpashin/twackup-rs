//
//  Database.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

@preconcurrency import CoreData
import os

final class Database: Sendable {
    private let persistentContainer: NSPersistentContainer

    private let context: NSManagedObjectContext

    init() {
        persistentContainer = NSPersistentContainer(name: "Twackup")
        persistentContainer.loadPersistentStores { _, error in
            if let error = error as NSError? {
                Task(priority: .utility) {
                    await FFILogger.shared.log("Unresolved error \(error), \(error.userInfo)")
                }
            }
        }
        persistentContainer.viewContext.mergePolicy = NSMergeByPropertyObjectTrumpMergePolicy

        context = persistentContainer.newBackgroundContext()
    }

    private func saveContext(_ context: NSManagedObjectContext) {
        if !context.hasChanges { return }

        do {
            try context.save()
        } catch {
            context.rollback()
            os_log("Unresolved error \(error), \((error as NSError).userInfo)")
        }
    }

    private func execute(
        request: NSPersistentStoreRequest,
        context: NSManagedObjectContext
    ) -> NSPersistentStoreResult? {
        do {
            let result = try context.execute(request)
            try context.save()
            return result
        } catch {
            context.rollback()
            os_log("Unresolved error \(error), \((error as NSError).userInfo)")
        }

        return nil
    }

    func add(packages: [BuildedPackage]) async {
        await background { context in
            var index = 0
            let total = packages.count

            // swiftlint:disable trailing_closure
            let request = NSBatchInsertRequest(entity: DebPackage.entity(), managedObjectHandler: { object in
                guard index < total else { return true }

                if let debPackage = object as? DebPackage {
                    let package = packages[index]

                    debPackage.setProperties(package: package.package)
                    debPackage.setProperties(file: package.debURL)
                }

                index += 1
                return false
            })
            // swiftlint:enable trailing_closure

            _ = self.execute(request: request, context: context)
        }
    }

    func fetchPackages() throws -> [DebPackage] {
        try self.context.fetch(DebPackage.fetchRequest())
    }

    func fetch(package: Package) throws -> DebPackage? {
        let possiblePackages = try context.fetch(DebPackage.fetchRequest(package: package))
        return possiblePackages.first
    }

    func delete(package: DebPackage) async {
        await delete(packages: [package])
    }

    func delete(packages: [DebPackage]) async {
        await background { context in
            if packages.isEmpty {
                return
            }

            let request = NSBatchDeleteRequest(objectIDs: packages.map { $0.objectID })
            _ = self.execute(request: request, context: context)
        }
    }

    func delete(packages: [Package]) async {
        await delete(packages: packages.compactMap { $0 as? DebPackage })
    }

    func packagesSize() -> Int64 {
        guard let results = try? self.context.fetch(DebPackage.debsSizeRequest()) else { return 0 }
        guard let results = results as? [[String: Int64]] else { return 0 }
        guard let result = results.first else { return 0 }

        return result["totalSize"] ?? 0
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

    private func background(_ work: @escaping (NSManagedObjectContext) -> Void) async {
        await withCheckedContinuation { continuation in
            persistentContainer.performBackgroundTask { context in
                work(context)
                continuation.resume()
            }
        }
    }
}
