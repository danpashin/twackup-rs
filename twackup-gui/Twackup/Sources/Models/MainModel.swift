//
//  MainModel.swift
//  Twackup
//
//  Created by Daniil on 17.12.2022.
//

class MainModel {
    let database: Database

    let dpkg: Dpkg

    let logger = FFILogger.shared

    private(set) lazy var databasePackageProvider = DatabasePackageProvider(database)

    init(database: Database, dpkg: Dpkg) {
        self.database = database
        self.dpkg = dpkg
    }
}
