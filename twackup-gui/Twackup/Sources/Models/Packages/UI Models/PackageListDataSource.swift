//
//  PackageListDataSource.swift
//  Twackup
//
//  Created by Daniil on 17.06.2024.
//

import BlankSlate

class PackageListDataSource<P: Package>: UITableViewDiffableDataSource<Int, P>, UISearchResultsUpdating {
    private(set) var dataProvider: PackageDataProvider<P>

    let tableView: UITableView

    var isAnySelected: Bool {
        tableView.indexPathsForSelectedRows != nil
    }

    var isAllSelected: Bool {
        guard let indexPaths = tableView.indexPathsForSelectedRows else { return false }
        return indexPaths.count == snapshot().numberOfItems
    }

    init(tableView: UITableView, dataProvider: PackageDataProvider<P>, cellProvider: @escaping CellProvider) {
        self.dataProvider = dataProvider
        self.tableView = tableView

        super.init(tableView: tableView, cellProvider: cellProvider)
    }

    func updateData(animated: Bool, force: Bool = false) async {
        do {
            if force {
                try await dataProvider.reload()
            }
        } catch {
            await FFILogger.shared.log("Error \(error): \(error.localizedDescription)", level: .error)
        }

        var snapshot = snapshot()
        snapshot.deleteAllItems()
        snapshot.appendSections([0])
        snapshot.appendItems(dataProvider.packages, toSection: 0)

        await withCheckedContinuation { continuation in
            apply(snapshot, animatingDifferences: animated) {
                continuation.resume()
            }
        }
    }

    func delete(packages: [P], animated: Bool = true) async {
        var snapshot = snapshot()
        snapshot.deleteItems(packages)

        await withCheckedContinuation { continuation in
            apply(snapshot, animatingDifferences: animated) { [self] in
                tableView.bs.reloadBlankSlate()
                continuation.resume()
            }
        }
    }

    func delete(at indexPath: IndexPath, animated: Bool = true) async {
        guard let package = itemIdentifier(for: indexPath) else { return }
        await delete(packages: [package], animated: animated)
    }

    func package(for indexPath: IndexPath) -> P? {
        itemIdentifier(for: indexPath)
    }

    func selected() -> [P] {
        tableView.indexPathsForSelectedRows?.compactMap { itemIdentifier(for: $0) } ?? []
    }

    // MARK: - UISearchResultsUpdating

    func updateSearchResults(for searchController: UISearchController) {
        var filter: PackageDataProvider<P>.Filter?
        if let text = searchController.searchBar.text, !text.isEmpty { filter = .name(text) }

        Task(priority: .userInitiated) {
            dataProvider.applyFilter(filter)
            await updateData(animated: false)
        }
    }
}
