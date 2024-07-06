//
//  DebPackageObject.swift
//
//
//  Created by Daniil on 28.11.2022.
//
//

import CoreData

struct BuildedPackage: @unchecked Sendable {
    let package: any Package

    let debURL: URL
}

struct UpdateMetadataError: Error {
}

class DebPackageObject: NSManagedObject {
    static let entityName = "DebPackageObject"

    class func fetchRequest() -> NSFetchRequest<DebPackageObject> {
        let request = NSFetchRequest<DebPackageObject>(entityName: entityName)
        request.sortDescriptors = [NSSortDescriptor(key: "name", ascending: true)]
        return request
    }

    class func fetchRequest<P: Package>(package: P) -> NSFetchRequest<DebPackageObject> {
        let request = NSFetchRequest<DebPackageObject>(entityName: entityName)
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
    @NSManaged var inodeID: Int64

    var icon: URL?
    var depiction: URL?

    func fillFrom(file: URL) throws {
        let metadata = try FileManager.default.attributesOfItem(atPath: file.path)
        guard let size = metadata[.size] as? Int64,
              let inode = metadata[.systemFileNumber] as? Int64
        else {
            throw UpdateMetadataError()
        }

        debSize = size
        inodeID = inode
    }

    func fillFrom<P: Package>(package: P) {
        assert(!package.name.isEmpty)
        name = package.name
        id = package.id
        version = package.version
        architecture = package.architecture
        section = package.section
        installedSize = package.installedSize
        buildDate = Date()
    }
}
