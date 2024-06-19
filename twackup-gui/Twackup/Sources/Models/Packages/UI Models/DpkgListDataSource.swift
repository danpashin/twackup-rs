//
//  DpkgListDataSource.swift
//  Twackup
//
//  Created by Daniil on 17.06.2024.
//

class DpkgListDataSource: PackageListDataSource<FFIPackage> {
    let dpkgProvider: DpkgDataProvier

    init(tableView: UITableView, dataProvider: DpkgDataProvier, cellProvider: @escaping CellProvider) {
        self.dpkgProvider = dataProvider
        super.init(tableView: tableView, dataProvider: dataProvider, cellProvider: cellProvider)
    }
}
