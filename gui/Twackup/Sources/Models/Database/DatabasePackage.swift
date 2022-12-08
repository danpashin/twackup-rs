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
        return NSPredicate(format: "id == %@ && version == %@", package.id, package.version)
    }

    @NSManaged var name: String
    @NSManaged var architecture: String?
    @NSManaged var id: String
    @NSManaged var version: String
    @NSManaged var section: PackageSection
    @NSManaged var humanDescription: String?
    @NSManaged var installedSize: Int64
    @NSManaged var debSize: Int64

    var icon: URL?
    var depiction: URL?

    func setProperties(package: Package) {
        name = package.name
        id = package.id
        version = package.version
        architecture = package.architecture
        section = package.section
        installedSize = package.installedSize
    }

    func isEqualTo(_ other: Package) -> Bool {
        id == other.id && version == other.version
    }
}
