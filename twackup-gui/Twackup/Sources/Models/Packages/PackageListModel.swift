//
//  PackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

protocol PackageListDelegate: AnyObject {
    func didSelect(items: [PackageListModel.TableViewItem], inEditState: Bool)

    func reloadData()

    func endReloadingData()
}

class PackageListModel: NSObject, UISearchResultsUpdating, UITableViewDelegate, UITableViewDataSource {
    struct TableViewItem {
        var indexPath: IndexPath

        var package: Package
    }

    private(set) var dataProvider: PackageDataProvider

    private(set) var metadata: ViewControllerMetadata

    let mainModel: MainModel

    weak var delegate: PackageListDelegate?

    var tableView: UITableView? {
        didSet {
            let cellID = String(describing: PackageTableViewCell.self)
            tableView?.register(PackageTableViewCell.self, forCellReuseIdentifier: cellID)
        }
    }

    var selectedItems: [TableViewItem] {
        guard let tableView else { return [] }
        guard let rows = tableView.indexPathsForSelectedRows?.map({ $0.row }) else { return [] }

        return dataProvider.packages.enumerated()
            .filter { rows.contains($0.offset) }
            .map { index, package in
                TableViewItem(indexPath: IndexPath(row: index, section: 0), package: package)
            }
    }

    init(mainModel: MainModel, dataProvider: PackageDataProvider, metadata: ViewControllerMetadata) {
        self.mainModel = mainModel
        self.dataProvider = dataProvider
        self.metadata = metadata
    }

    func selectAll() {
        guard let tableView else { return }
        for row in 0..<dataProvider.packages.count {
            tableView.selectRow(at: IndexPath(row: row, section: 0), animated: true, scrollPosition: .none)
        }
    }

    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return dataProvider.packages.count
    }

    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cellID = String(describing: PackageTableViewCell.self)
        let cell = tableView.dequeueReusableCell(withIdentifier: cellID, for: indexPath)
        if let cell = cell as? PackageTableViewCell {
            cell.package = dataProvider.packages[indexPath.row]
        }

        return cell
    }

    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        delegate?.didSelect(items: selectedItems, inEditState: tableView.isEditing)
    }

    func tableView(_ tableView: UITableView, didDeselectRowAt indexPath: IndexPath) {
        delegate?.didSelect(items: selectedItems, inEditState: tableView.isEditing)
    }

    func updateSearchResults(for searchController: UISearchController) {
        var filter: PackageDataProvider.Filter?
        if let text = searchController.searchBar.text, !text.isEmpty { filter = .name(text) }

        dataProvider.applyFilter(filter)

        delegate?.reloadData()
    }
}
