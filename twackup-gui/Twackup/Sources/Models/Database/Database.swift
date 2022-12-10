//
//  Database.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import CoreData

class Database {
    lazy private var persistentContainer: NSPersistentContainer = {
        let container = NSPersistentContainer(name: "Twackup")
        container.viewContext.mergePolicy = NSMergeByPropertyObjectTrumpMergePolicy
        container.loadPersistentStores(completionHandler: { (_, error) in
            if let error = error as NSError? {
                FFILogger.shared.log("Unresolved error \(error), \(error.userInfo)")
            }
        })
        return container
    }()

    lazy private var context: NSManagedObjectContext = persistentContainer.newBackgroundContext()

    private func saveContext(_ context: NSManagedObjectContext) {
        if !context.hasChanges { return }

        do {
            try context.save()
        } catch {
            context.rollback()
            print("Unresolved error \(error), \((error as NSError).userInfo)")
        }
    }

    private func execute(request: NSPersistentStoreRequest,
                         context: NSManagedObjectContext) -> NSPersistentStoreResult? {
        do {
            let result = try context.execute(request)
            try context.save()
            return result
        } catch {
            context.rollback()
            print("Unresolved error \(error), \((error as NSError).userInfo)")
        }

        return nil
    }

    func addBuildedPackages(_ packages: [BuildedPackage], completion: (() -> Void)? = nil) {
        persistentContainer.performBackgroundTask { context in
            var index = 0
            let total = packages.count

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

    func delete(package: DebPackage) {
        delete(packages: [package])
    }

    func delete(packages: [DebPackage]) {
        persistentContainer.performBackgroundTask { context in
            let request = NSBatchDeleteRequest(objectIDs: packages.map({ $0.objectID }))
            _ = self.execute(request: request, context: context)
        }
    }

    func delete(packages: [Package]) {
        delete(packages: packages.compactMap({ $0 as? DebPackage}))
    }
}
