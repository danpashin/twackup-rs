//
//  DebPackage.swift
//  
//
//  Created by Daniil on 28.11.2022.
//
//

import CoreData

class DebPackage: DatabasePackage {
    class func fetchRequest() -> NSFetchRequest<DebPackage> {
        return NSFetchRequest<DebPackage>(entityName: "DebPackage")
    }

    @NSManaged var buildDate: Date
    @NSManaged var debRelativePath: String
}
