//
//  PackageListVC.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class PackageListVC: UIViewController, PackageListDelegate, ScrollableViewController {
    private(set) var model: PackageListModel

    private(set) var detail: PackageDetailVC

    override var tabBarItem: UITabBarItem? {
        get { model.metadata.tabbarItem }
        set { }
    }

    private(set) lazy var searchController: UISearchController = {
        let controller = UISearchController(searchResultsController: nil)
        controller.obscuresBackgroundDuringPresentation = false
        controller.searchResultsUpdater = model
        controller.searchBar.placeholder = "search-field-lbl".localized
        return controller
    }()

    private(set) lazy var tableView: UITableView = {
        let table = UITableView(frame: .zero, style: .insetGrouped)
        table.delegate = model
        table.dataSource = model
        table.backgroundColor = .systemGroupedBackground

        return table
    }()

    lazy var tableViewLargeReloadingIndicator = UIActivityIndicatorView(style: .large)

    init(model: PackageListModel, detail: PackageDetailVC ) {
        self.model = model
        self.detail = detail
        super.init(nibName: nil, bundle: nil)

        model.delegate = self
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func loadView() {
        view = tableView
        model.tableView = tableView
        tableView.backgroundView = tableViewLargeReloadingIndicator
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        navigationItem.title = model.metadata.navTitle
        navigationItem.searchController = searchController

        reloadData()
    }

    func reloadData() {
        DispatchQueue.main.async { [self] in
            tableView.reloadData()
            endReloadingData()
        }
    }

    func endReloadingData() {
        tableView.flashScrollIndicators()
        tableViewLargeReloadingIndicator.stopAnimating()
    }

    func didSelect(items: [PackageListModel.TableViewItem], inEditState: Bool) {
        guard !inEditState, let item = items.first else { return }
        guard let detailNav = detail.navigationController else { return }

        if splitViewController?.isCollapsed ?? false {
            tableView.deselectRow(at: item.indexPath, animated: true)
        }

        detail.package = item.package

        showDetailViewController(detailNav, sender: nil)
    }

    func scrollToTop(animated: Bool) {
        if model.dataProvider.packages.isEmpty { return }
        tableView.scrollToRow(at: IndexPath(row: 0, section: 0), at: .top, animated: animated)
    }
}
