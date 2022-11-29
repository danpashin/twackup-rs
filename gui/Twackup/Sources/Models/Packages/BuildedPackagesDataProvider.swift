//
//  BuildedPackagesDataProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import UIKit

extension PackageVC {
    class DatabaseProvider: DataProvider {
        private let database: Database

        init(_ database: Database) {
            self.database = database

            super.init(packages: database.fetchBuildedPackages())
        }
    }
}
