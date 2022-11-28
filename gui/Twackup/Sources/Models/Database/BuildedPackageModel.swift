//
//  BuildedPackageModel.swift
//  
//
//  Created by Daniil on 28.11.2022.
//
//

import Foundation
import CoreData

class BuildedPackageModel: DatabasePackageModel {
    class func fetchRequest() -> NSFetchRequest<BuildedPackageModel> {
        return NSFetchRequest<BuildedPackageModel>(entityName: "BuildedPackageModel")
    }

    @NSManaged var buildDate: Date
    @NSManaged var debRelativePath: String
}
