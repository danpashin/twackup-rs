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
                fatalError("Unresolved error \(error), \(error.userInfo)")
            }
        })
        return container
    }()

    lazy private var context: NSManagedObjectContext = {
        return persistentContainer.viewContext
    }()

    private func saveContext() {
        if !context.hasChanges { return }

        do {
            try context.save()
        } catch {
            context.rollback()
            let nserror = error as NSError
            fatalError("Unresolved error \(nserror), \(nserror.userInfo)")
        }
    }

    func createBuildedPackage() -> DebPackage {
        let package = DebPackage(context: context)
        package.buildDate = Date()
        return package
    }

    func addBuildedPackage(_ package: DebPackage) {
        context.perform { [self] in
            context.insert(package)
            saveContext()
        }
    }

    func fetchBuildedPackages() -> [DebPackage] {
        (try? self.context.fetch(DebPackage.fetchRequest())) ?? []
    }

    func fetch(package: Package) -> DebPackage? {
        try? context.fetch(DebPackage.fetchRequest(package: package)).first
    }

    func delete(package: DebPackage) {
        context.perform { [self] in
            context.delete(package)
            saveContext()
        }
    }

    func delete(packages: [DebPackage]) {
        context.perform { [self] in
            packages.forEach({ context.delete($0) })
            saveContext()
        }
    }

    func delete(packages: [Package]) {
        delete(packages: packages.compactMap({ $0 as? DebPackage}))
    }
}
