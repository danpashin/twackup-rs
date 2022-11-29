//
//  PackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

protocol PackageDetailDelegate: AnyObject {
    func didSelectPackage(_ package: Package)
}

protocol PackagesVCModelDelegate: AnyObject {
    func reloadTableView()
}

extension PackageVC {
    class MainModel: NSObject, UISearchResultsUpdating, UITableViewDelegate, UITableViewDataSource {
        private(set) var dataProvider: DataProvider

        var detailDelegate: PackageDetailDelegate?

        var delegate: PackagesVCModelDelegate?

        init(dataProvider: DataProvider) {
            self.dataProvider = dataProvider
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
            detailDelegate?.didSelectPackage(dataProvider.packages[indexPath.row])
        }

        func updateSearchResults(for searchController: UISearchController) {
            var filter: DataProvider.Filter?
            if let text = searchController.searchBar.text, !text.isEmpty { filter = .name(text) }

            dataProvider.applyFilter(filter)
            delegate?.reloadTableView()
        }
    }
}
