//
//  DebsListDataSource.swift
//  Twackup
//
//  Created by Daniil on 17.06.2024.
//

class DebsListDataSource: PackageListDataSource<DebPackage> {
    private(set) var debsProvider: DatabasePackageProvider

    init(tableView: UITableView, dataProvider: DatabasePackageProvider, cellProvider: @escaping CellProvider) {
        self.debsProvider = dataProvider
        super.init(tableView: tableView, dataProvider: dataProvider, cellProvider: cellProvider)
    }

    override func delete(packages: [DebPackage], animated: Bool = true) async {
        do {
            try await debsProvider.delete(packages: packages)
            await super.delete(packages: packages, animated: animated)
        } catch {
            await FFILogger.shared.log(error.localizedDescription, level: .warning)
        }
    }
}
