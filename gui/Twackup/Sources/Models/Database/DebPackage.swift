//
//  DebPackage.swift
//  
//
//  Created by Daniil on 28.11.2022.
//
//

import CoreData

class DebPackage: DatabasePackage {
    static let entityName = "DebPackage"

    class func fetchRequest() -> NSFetchRequest<DebPackage> {
        return NSFetchRequest<DebPackage>(entityName: entityName)
    }

    class func fetchRequest(package: Package) -> NSFetchRequest<DebPackage> {
        let request = NSFetchRequest<DebPackage>(entityName: entityName)
        request.predicate = fetchSinglePredicate(package: package)
        return request
    }

    @NSManaged var buildDate: Date
    @NSManaged var path: String
    @NSManaged var size: Int64

    func setProperties(file: URL, pathRelativeTo: URL) {
        let metadata = try? FileManager.default.attributesOfItem(atPath: file.path)
        size = (metadata?[.size] as? Int64) ?? 0

        path = file.path.deletePrefix(pathRelativeTo.path)
    }
}
