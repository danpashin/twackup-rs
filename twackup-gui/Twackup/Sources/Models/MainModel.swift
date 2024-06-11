//
//  MainModel.swift
//  Twackup
//
//  Created by Daniil on 17.12.2022.
//

class MainModel: @unchecked Sendable {
    let database: Database

    let dpkg: Dpkg

    let logger = FFILogger.shared

    let preferences: Preferences

    private(set) lazy var databasePackageProvider = DatabasePackageProvider(database)

    init(database: Database, dpkg: Dpkg, preferences: Preferences) {
        self.database = database
        self.dpkg = dpkg
        self.preferences = preferences
    }
}
