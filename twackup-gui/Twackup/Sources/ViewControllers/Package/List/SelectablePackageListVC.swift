//
//  SelectablePackageListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import UIKit

class SelectablePackageListVC<P: Package>: PackageListVC<P> {
    private(set) lazy var selectAllBarBtn: UIBarButtonItem = {
        UIBarButtonItem(title: "Select All".localized, primaryAction: UIAction { [self] _ in
            tableHandler.selectAll(animated: false)
        })
    }()

    private(set) lazy var deselectAllBarBtn: UIBarButtonItem = {
        UIBarButtonItem(title: "Deselect All".localized, primaryAction: UIAction { [self] _ in
            tableHandler.deselectAll(animated: false)
        })
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        tableView.allowsMultipleSelectionDuringEditing = true

        navigationItem.rightBarButtonItem = editButtonItem
    }

    override func reloadData(animated: Bool, force: Bool) async {
        await super.reloadData(animated: animated, force: force)

        setEditing(false, animated: animated)

        editButtonItem.isEnabled = !dataSource.dataProvider.packages.isEmpty
    }

    override func selectionDidUpdate() {
        super.selectionDidUpdate()

        navigationItem.leftBarButtonItem = dataSource.isAllSelected ? deselectAllBarBtn : selectAllBarBtn
    }

    override func setEditing(_ editing: Bool, animated: Bool) {
        super.setEditing(editing, animated: animated)

        tableView.setEditing(editing, animated: animated)
        navigationItem.leftBarButtonItem = editing ? selectAllBarBtn : nil
    }
}
