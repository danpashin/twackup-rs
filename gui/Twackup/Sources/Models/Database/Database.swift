//
//  Database.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import Foundation
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
        if context.hasChanges {
            do {
                try context.save()
            } catch {
                context.rollback()
                let nserror = error as NSError
                fatalError("Unresolved error \(nserror), \(nserror.userInfo)")
            }
        }
    }
}

extension Database {
    func createBuildedPackage() -> BuildedPackageModel {
        BuildedPackageModel(context: context)
    }

    func addBuildedPackage(_ package: BuildedPackageModel) {
        DispatchQueue.global().async {
            self.context.insert(package)
            self.saveContext()
        }
    }

    func fetchBuildedPackages() -> [BuildedPackageModel] {
        // swiftlint:disable force_try
        try! self.context.fetch(BuildedPackageModel.fetchRequest())
    }
}
