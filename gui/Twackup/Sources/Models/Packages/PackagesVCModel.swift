//
//  PackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

protocol PackageDetailDelegate {
    func didSelectPackage(_ package: Package)
}

protocol PackagesVCModelDelegate {
    func reloadTableView()
}

class PackagesVCModel: NSObject, UISearchResultsUpdating, UISearchControllerDelegate, UITableViewDelegate, UITableViewDataSource {
    private(set) var dataProvider: PackagesDataProvider

    var detailDelegate: PackageDetailDelegate? = nil

    var delegate: PackagesVCModelDelegate? = nil

    init(dataProvider: PackagesDataProvider) {
        self.dataProvider = dataProvider
    }

    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return dataProvider.packages.count
    }

    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "PackageCell", for: indexPath)
        if let cell = cell as? PackageTableViewCell {
            cell.package = dataProvider.packages[indexPath.row]
        }

        return cell
    }

    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        detailDelegate?.didSelectPackage(dataProvider.packages[indexPath.row])
    }

    func updateSearchResults(for searchController: UISearchController) {
        var filter: PackageFilter?
        if let text = searchController.searchBar.text, !text.isEmpty { filter = .Name(text) }

        dataProvider.filter(filter)
        delegate?.reloadTableView()
    }
}