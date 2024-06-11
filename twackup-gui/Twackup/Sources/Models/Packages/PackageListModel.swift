//
//  PackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

@MainActor
protocol PackageListDelegate: AnyObject {
    func didSelect(items: [PackageListModel.TableViewItem], inEditState: Bool)

    func reloadData() async

    func endReloadingData() async
}

class PackageListModel: NSObject, UISearchResultsUpdating, UITableViewDelegate, UITableViewDataSource {
    struct TableViewItem {
        var indexPath: IndexPath

        var package: Package
    }

    /// Model that provides data for the table
    private(set) var dataProvider: PackageDataProvider

    private(set) var metadata: ViewControllerMetadata

    /// Main model instance
    let mainModel: MainModel

    weak var delegate: PackageListDelegate?

    var tableView: UITableView? {
        didSet {
            let cellID = String(describing: PackageTableViewCell.self)
            tableView?.register(PackageTableViewCell.self, forCellReuseIdentifier: cellID)
        }
    }

    /// Items that are currently selected
    var selectedItems: [TableViewItem] {
        guard let tableView else { return [] }
        guard let rows = tableView.indexPathsForSelectedRows?.map({ $0.row }) else { return [] }

        return dataProvider.packages
            .enumerated()
            .filter { rows.contains($0.offset) }
            .map { index, package in
                TableViewItem(indexPath: IndexPath(row: index, section: 0), package: package)
            }
    }

    /// All items that table view contains
    var allItems: [TableViewItem] {
        dataProvider.packages
            .enumerated()
            .map { index, package in
                TableViewItem(indexPath: IndexPath(row: index, section: 0), package: package)
            }
    }

    // MARK: - Public functions

    init(mainModel: MainModel, dataProvider: PackageDataProvider, metadata: ViewControllerMetadata) {
        self.mainModel = mainModel
        self.dataProvider = dataProvider
        self.metadata = metadata
    }

    /// Selects all items that are currently on table view
    func selectAll() {
        guard let tableView else { return }
        for row in 0..<dataProvider.packages.count {
            tableView.selectRow(at: IndexPath(row: row, section: 0), animated: true, scrollPosition: .none)
        }

        delegate?.didSelect(items: selectedItems, inEditState: tableView.isEditing)
    }

    // MARK: - UITableViewDataSource

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

    // MARK: - UITableViewDelegate

    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        delegate?.didSelect(items: selectedItems, inEditState: tableView.isEditing)
    }

    func tableView(_ tableView: UITableView, didDeselectRowAt indexPath: IndexPath) {
        delegate?.didSelect(items: selectedItems, inEditState: tableView.isEditing)
    }

    // MARK: - UISearchResultsUpdating

    func updateSearchResults(for searchController: UISearchController) {
        var filter: PackageDataProvider.Filter?
        if let text = searchController.searchBar.text, !text.isEmpty { filter = .name(text) }

        Task(priority: .userInitiated) {
            dataProvider.applyFilter(filter)

            await delegate?.reloadData()
        }
    }
}
