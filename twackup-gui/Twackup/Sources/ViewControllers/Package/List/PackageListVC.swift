//
//  PackageListVC.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class PackageListVC<P: Package>: UIViewController, ScrollableViewController {
    private(set) lazy var dataSource: PackageListDataSource<P> = configureDataSource()

    private(set) lazy var tableHandler: PackageListDelegate<P> = configureTableDelegate()

    let mainModel: MainModel

    class var metadata: (any ViewControllerMetadata)? {
        nil
    }

    private(set) var detail: PackageDetailVC<P>

    override var tabBarItem: UITabBarItem? {
        get { Self.metadata?.tabbarItem }
        set { }
    }

    private(set) lazy var searchController: UISearchController = {
        let controller = UISearchController(searchResultsController: nil)
        controller.obscuresBackgroundDuringPresentation = false
        controller.searchResultsUpdater = dataSource
        controller.searchBar.placeholder = "search-field-lbl".localized
        return controller
    }()

    private(set) lazy var tableView: UITableView = {
        let table = UITableView(frame: .zero, style: .insetGrouped)
        table.backgroundColor = .systemGroupedBackground

        return table
    }()

    init(mainModel: MainModel, detail: PackageDetailVC<P>) {
        self.mainModel = mainModel
        self.detail = detail

        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func loadView() {
        view = tableView
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        tableView.dataSource = dataSource
        tableView.delegate = tableHandler

        let cell = PackageTableViewCell<P>.self
        tableView.register(cell, forCellReuseIdentifier: String(describing: cell))

        navigationItem.title = Self.metadata?.navTitle
        navigationItem.searchController = searchController

        reloadData(animated: false, force: true)
    }

    nonisolated func reloadData(animated: Bool, force: Bool) {
        Task {
            await reloadData(animated: false, force: true)
        }
    }

    func reloadData(animated: Bool, force: Bool) async {
        await dataSource.updateData(animated: animated, force: force)

        tableView.flashScrollIndicators()
    }

    func selectionDidUpdate() {
    }

    func configureDataSource() -> PackageListDataSource<P> {
        fatalError("Must be overritten by child classes")
    }

    func configureTableDelegate() -> PackageListDelegate<P> {
        PackageListDelegate(dataSource: dataSource, listController: self)
    }

    // MARK: - ScrollableViewController

    func scrollToInitialPosition(animated: Bool) {
        if dataSource.dataProvider.packages.isEmpty { return }
        tableView.scrollToRow(at: IndexPath(row: 0, section: 0), at: .top, animated: animated)
    }
}
