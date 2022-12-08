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
        let request = NSFetchRequest<DebPackage>(entityName: entityName)
        request.sortDescriptors = [NSSortDescriptor(key: "name", ascending: true)]
        return request
    }

    class func fetchRequest(package: Package) -> NSFetchRequest<DebPackage> {
        let request = NSFetchRequest<DebPackage>(entityName: entityName)
        request.predicate = fetchSinglePredicate(package: package)
        return request
    }

    @NSManaged var buildDate: Date
    @NSManaged var relPath: String

    func setProperties(file: URL, pathRelativeTo: URL) {
        let metadata = try? FileManager.default.attributesOfItem(atPath: file.path)
        debSize = (metadata?[.size] as? Int64) ?? 0

        relPath = file.path.deletePrefix(pathRelativeTo.path)
    }

    func fileURL() -> URL {
        Dpkg.defaultSaveDirectory.appendingPathComponent(relPath)
    }
}
