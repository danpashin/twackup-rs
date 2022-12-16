//
//  PackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

@objc
protocol PackageListDelegate {
    func reloadTableView()

    func didSelectPackage(_ package: Package)

    @objc
    optional func tableView(_ tableView: UITableView, didUpdateSelection selected: [IndexPath]?)
}

extension PackageVC {
    class PackageListModel: NSObject, UISearchResultsUpdating, UITableViewDelegate, UITableViewDataSource {
        private(set) var dataProvider: PackageDataProvider

        private(set) var metadata: ViewControllerMetadata

        weak var delegate: PackageListDelegate?

        var tableView: UITableView? {
            didSet {
                let cellID = String(describing: PackageTableViewCell.self)
                tableView?.register(PackageTableViewCell.self, forCellReuseIdentifier: cellID)
            }
        }

        init(dataProvider: PackageDataProvider, metadata: ViewControllerMetadata) {
            self.dataProvider = dataProvider
            self.metadata = metadata
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
            delegate?.didSelectPackage(dataProvider.packages[indexPath.row])

            delegate?.tableView?(tableView, didUpdateSelection: tableView.indexPathsForSelectedRows)
        }

        func tableView(_ tableView: UITableView, didDeselectRowAt indexPath: IndexPath) {
            delegate?.tableView?(tableView, didUpdateSelection: tableView.indexPathsForSelectedRows)
        }

        func updateSearchResults(for searchController: UISearchController) {
            var filter: PackageDataProvider.Filter?
            if let text = searchController.searchBar.text, !text.isEmpty { filter = .name(text) }

            dataProvider.applyFilter(filter)
            delegate?.reloadTableView()
        }
    }
}
