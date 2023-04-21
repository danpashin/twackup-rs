//
//  Database.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import CoreData
import os

class Database {
    private lazy var persistentContainer: NSPersistentContainer = {
        let container = NSPersistentContainer(name: "Twackup")
        container.loadPersistentStores { _, error in
            if let error = error as NSError? {
                FFILogger.shared.log("Unresolved error \(error), \(error.userInfo)")
            }
        }
        container.viewContext.mergePolicy = NSMergeByPropertyObjectTrumpMergePolicy
        return container
    }()

    private lazy var context: NSManagedObjectContext = persistentContainer.newBackgroundContext()

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

    func addBuildedPackages(_ packages: [BuildedPackage], completion: (() -> Void)? = nil) {
        persistentContainer.performBackgroundTask { context in
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

            completion?()
        }
    }

    func fetchBuildedPackages() -> [DebPackage] {
        (try? self.context.fetch(DebPackage.fetchRequest())) ?? []
    }

    func fetch(package: Package) -> DebPackage? {
        try? context.fetch(DebPackage.fetchRequest(package: package)).first
    }

    func delete(package: DebPackage, completion: (() -> Void)? = nil) {
        delete(packages: [package], completion: completion)
    }

    func delete(packages: [DebPackage], completion: (() -> Void)? = nil) {
        persistentContainer.performBackgroundTask { context in
            if packages.isEmpty {
                completion?()
                return
            }

            let request = NSBatchDeleteRequest(objectIDs: packages.map { $0.objectID })
            _ = self.execute(request: request, context: context)

            completion?()
        }
    }

    func delete(packages: [Package], completion: (() -> Void)? = nil) {
        delete(packages: packages.compactMap { $0 as? DebPackage }, completion: completion)
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
}
