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

        override var tabBarItem: UITabBarItem? {
            get { model.metadata.tabbarItem }
            set { }
        }

        private(set) lazy var searchController: UISearchController = {
            let controller = UISearchController(searchResultsController: nil)
            controller.obscuresBackgroundDuringPresentation = false
            controller.searchResultsUpdater = self.model
            controller.searchBar.placeholder = "search-field-lbl".localized
            return controller
        }()

        private(set) lazy var tableView: UITableView = {
            let table = UITableView(frame: .zero, style: .insetGrouped)
            table.delegate = model
            table.dataSource = model
            table.backgroundColor = .systemBackground

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
            model.tableView = tableView
        }

        override func viewDidLoad() {
            super.viewDidLoad()

            navigationItem.title = model.metadata.navTitle
            navigationItem.searchController = searchController
        }

        func reloadTableView() {
            tableView.reloadData()
        }

        func didSelectPackage(_ package: Package) {
            guard !tableView.isEditing, let selected = tableView.indexPathForSelectedRow else { return }

            if UIDevice.current.userInterfaceIdiom == .phone {
                tableView.deselectRow(at: selected, animated: true)
                navigationController?.pushViewController(detail, animated: true)
            }

            detail.didSelectPackage(package)
        }
    }
}
