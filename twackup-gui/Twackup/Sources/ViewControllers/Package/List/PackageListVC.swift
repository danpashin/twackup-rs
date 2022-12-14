//
//  PackageListVC.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

extension PackageVC {
    class PackageListVC: UIViewController, PackageListDelegate {
        private(set) var model: PackageListModel

        private(set) var detail: DetailVC

        lazy private(set) var searchController: UISearchController = {
            let controller = UISearchController(searchResultsController: nil)
            controller.obscuresBackgroundDuringPresentation = false
            controller.searchResultsUpdater = self.model
            controller.searchBar.placeholder = Bundle.appLocalize("search-field-lbl")
            return controller
        }()

        lazy private(set) var tableView: UITableView = {
            let table = UITableView(frame: .zero, style: .insetGrouped)
            table.delegate = model
            table.dataSource = model

            return table
        }()

        init(model: PackageListModel, detail: DetailVC ) {
            self.model = model
            self.detail = detail
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

            navigationItem.title = model.metadata.navTitle
            navigationItem.searchController = searchController
            navigationController?.navigationBar.prefersLargeTitles = true

            tableView.backgroundColor = .systemBackground
            let cellClass = PackageTableViewCell.self
            tableView.register(cellClass, forCellReuseIdentifier: String(describing: cellClass))
        }

        func reloadTableView() {
            tableView.reloadData()
        }

        func didSelectPackage(_ package: Package) {
            guard !tableView.isEditing else { return }

            if UIDevice.current.userInterfaceIdiom == .phone {
                tableView.deselectRow(at: tableView.indexPathForSelectedRow!, animated: true)
                navigationController?.pushViewController(detail, animated: true)
            }

            detail.didSelectPackage(package)
        }
    }
}
