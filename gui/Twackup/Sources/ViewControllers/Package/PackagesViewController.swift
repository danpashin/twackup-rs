//
//  PackagesViewController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class PackagesViewController: UIViewController, PackagesVCModelDelegate {

    private(set) var model: PackagesVCModel

    lazy private(set) var searchController: UISearchController = {
        let controller = UISearchController(searchResultsController: nil)
        controller.obscuresBackgroundDuringPresentation = false
        controller.searchResultsUpdater = self.model
        controller.delegate = self.model
        
        controller.searchBar.placeholder = "Search"
        return controller
    }()

    lazy private(set) var tableView: UITableView =  {
        let table = UITableView(frame: CGRectZero, style: .plain)
        table.delegate = model
        table.dataSource = model

        return table
    }()

    init(_ dataProvider: PackagesDataProvider) {
        self.model = PackagesVCModel(dataProvider: dataProvider)
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

        navigationItem.title = model.dataProvider.navTitle
        navigationItem.searchController = searchController
        navigationController?.navigationBar.prefersLargeTitles = true

        tableView.backgroundColor = .systemBackground
        tableView.register(PackageTableViewCell.self, forCellReuseIdentifier: "PackageCell")
    }

    func reloadTableView() {
        self.tableView.reloadData()
    }
}
