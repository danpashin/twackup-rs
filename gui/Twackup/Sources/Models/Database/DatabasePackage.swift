//
//  DatabasePackage.swift
//  
//
//  Created by Daniil on 28.11.2022.
//
//

import CoreData

class DatabasePackage: NSManagedObject, Package {
    class func fetchRequest() -> NSFetchRequest<DatabasePackage> {
        return NSFetchRequest<DatabasePackage>(entityName: "DatabasePackage")
    }

    class func fetchSinglePredicate(package: Package) -> NSPredicate {
        return NSPredicate(format: "id == %@", package.id)
    }

    @NSManaged var name: String
    @NSManaged var architecture: String?
    @NSManaged var id: String
    @NSManaged var version: String
    @NSManaged var section: PackageSection
    @NSManaged var humanDescription: String?

    var icon: URL?
    var depiction: URL?

    func setProperties(package: Package) {
        name = package.name
        id = package.id
        version = package.version
        architecture = package.architecture
        section = package.section
    }
}
