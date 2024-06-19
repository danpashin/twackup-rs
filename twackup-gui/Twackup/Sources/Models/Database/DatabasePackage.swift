//
//  DatabasePackage.swift
//  Twackup
//
//  Created by Daniil on 16.06.2024.
//

import CoreData

final class DebPackage: Package, Sendable {
    static func == (lhs: DebPackage, rhs: DebPackage) -> Bool {
        lhs.id == rhs.id && lhs.version == rhs.version
    }

    let databaseID: NSManagedObjectID

    let id: String

    let name: String

    let version: String

    let section: PackageSection

    let architecture: String?

    let icon: URL?

    let depiction: URL?

    let humanDescription: String?

    let installedSize: Int64

    let fileURL: URL

    let debSize: Measurement<UnitInformationStorage>

    init(object: DebPackageObject) {
        databaseID = object.objectID

        id = object.id
        name = object.name
        version = object.version
        section = object.section
        architecture = object.architecture
        icon = object.icon
        depiction = object.depiction
        humanDescription = object.humanDescription
        installedSize = object.installedSize

        fileURL = Dpkg.defaultSaveDirectory.appendingPathComponent(object.relPath)

        debSize = Measurement(value: Double(object.debSize), unit: .bytes)
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(id)
        hasher.combine(version)
    }
}
