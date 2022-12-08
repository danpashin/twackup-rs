//
//  PackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

protocol PackageListDelegate: AnyObject {
    func reloadTableView()

    func didSelectPackage(_ package: Package)
}

extension PackageVC {
    class PackageListModel: NSObject, UISearchResultsUpdating, UITableViewDelegate, UITableViewDataSource {
        private(set) var dataProvider: PackageDataProvider

        private(set) var metadata: ViewControllerMetadata

        var delegate: PackageListDelegate?

        init(dataProvider: PackageDataProvider, metadata: ViewControllerMetadata) {
            self.dataProvider = dataProvider
            self.metadata = metadata
        }

        func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
            return dataProvider.packages.count
        }

        func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
            let cell = tableView.dequeueReusableCell(withIdentifier: "PackageCell", for: indexPath)
            if let cell = cell as? SimpleTableViewCell {
                cell.package = dataProvider.packages[indexPath.row]
            }

            return cell
        }

        func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
            delegate?.didSelectPackage(dataProvider.packages[indexPath.row])
        }

        func updateSearchResults(for searchController: UISearchController) {
            var filter: PackageDataProvider.Filter?
            if let text = searchController.searchBar.text, !text.isEmpty { filter = .name(text) }

            dataProvider.applyFilter(filter)
            delegate?.reloadTableView()
        }
    }
}
