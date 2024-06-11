//
//  DebPackage.swift
//  
//
//  Created by Daniil on 28.11.2022.
//
//

import CoreData

struct BuildedPackage: @unchecked Sendable {
    let package: Package

    let debURL: URL
}

class DebPackage: NSManagedObject, Package {
    static let entityName = "DebPackage"

    class func fetchRequest() -> NSFetchRequest<DebPackage> {
        let request = NSFetchRequest<DebPackage>(entityName: entityName)
        request.sortDescriptors = [NSSortDescriptor(key: "name", ascending: true)]
        return request
    }

    class func fetchRequest(package: Package) -> NSFetchRequest<DebPackage> {
        let request = NSFetchRequest<DebPackage>(entityName: entityName)
        request.predicate = NSPredicate(format: "id == %@ && version == %@", package.id, package.version)
        return request
    }

    class func debsSizeRequest() -> NSFetchRequest<NSFetchRequestResult> {
        let sumDesc = NSExpressionDescription()
        sumDesc.expression = NSExpression(forFunction: "sum:", arguments: [NSExpression(forKeyPath: "debSize")])
        sumDesc.name = "totalSize"
        sumDesc.expressionResultType = .integer64AttributeType

        let request = NSFetchRequest<NSFetchRequestResult>(entityName: entityName)
        request.returnsObjectsAsFaults = false
        request.propertiesToFetch = [sumDesc]
        request.resultType = .dictionaryResultType

        return request
    }

    @NSManaged var name: String
    @NSManaged var architecture: String?
    @NSManaged var id: String
    @NSManaged var version: String
    @NSManaged var section: PackageSection
    @NSManaged var humanDescription: String?
    @NSManaged var installedSize: Int64
    @NSManaged var debSize: Int64

    @NSManaged var buildDate: Date
    @NSManaged var relPath: String

    var icon: URL?
    var depiction: URL?

    var fileURL: URL {
        Dpkg.defaultSaveDirectory.appendingPathComponent(relPath)
    }

    func setProperties(file: URL) {
        let metadata = try? FileManager.default.attributesOfItem(atPath: file.path)
        debSize = (metadata?[.size] as? Int64) ?? 0

        relPath = file.path.deletePrefix(Dpkg.defaultSaveDirectory.path)
    }

    func setProperties(package: Package) {
        assert(!package.name.isEmpty)
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
