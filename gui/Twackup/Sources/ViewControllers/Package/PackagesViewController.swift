//
//  PackagesViewController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

extension PackageVC {
    class ListWithDetailVC: UIViewController, PackagesVCModelDelegate {

        private(set) var model: MainModel

        private(set) var metadata: PackageVC.Metadata

        lazy private(set) var searchController: UISearchController = {
            let controller = UISearchController(searchResultsController: nil)
            controller.obscuresBackgroundDuringPresentation = false
            controller.searchResultsUpdater = self.model
            controller.searchBar.placeholder = "Search"
            return controller
        }()

        lazy private(set) var tableView: UITableView = {
            let table = UITableView(frame: .zero, style: .plain)
            table.delegate = model
            table.dataSource = model

            return table
        }()

        init(_ dataProvider: PackageVC.DataProvider, _ metadata: PackageVC.Metadata) {
            self.model = MainModel(dataProvider: dataProvider)
            self.metadata = metadata
            super.init(nibName: nil, bundle: nil)

            model.delegate = self
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        override func loadView() {
            self.view = tableView
        }

        override func viewDidLoad() {
            super.viewDidLoad()

            navigationItem.title = metadata.navTitle
            navigationItem.searchController = searchController
            navigationController?.navigationBar.prefersLargeTitles = true

            tableView.backgroundColor = .systemBackground
            tableView.register(SimpleTableViewCell.self, forCellReuseIdentifier: "PackageCell")
        }

        func reloadTableView() {
            self.tableView.reloadData()
        }
    }
}
