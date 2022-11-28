//
//  DatabasePackageModel.swift
//  
//
//  Created by Daniil on 28.11.2022.
//
//

import Foundation
import CoreData

class DatabasePackageModel: NSManagedObject, Package {
    class func fetchRequest() -> NSFetchRequest<DatabasePackageModel> {
        return NSFetchRequest<DatabasePackageModel>(entityName: "DatabasePackageModel")
    }

    @NSManaged var name: String
    @NSManaged var architecture: String
    @NSManaged var id: String
    @NSManaged var version: String
    @NSManaged var section: PackageSection
    @NSManaged var pdescription: String

    var icon: URL?
    var depiction: URL?

    func fillFrom(package: Package) {
        name = package.name
        id = package.id
        version = package.version
        architecture = package.architecture
    }

    override var description: String {
        return pdescription
    }
}
