//
//  SelectablePackageListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import DZNEmptyDataSet
import UIKit

class SelectablePackageListVC: PackageListVC {
    private(set) lazy var selectAllBarBtn: UIBarButtonItem = {
        let title = "debs-selectall-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionSelectAll))
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        tableView.allowsMultipleSelectionDuringEditing = true

        navigationItem.rightBarButtonItem = editButtonItem
    }

    override func endReloadingData() {
        super.endReloadingData()

        setEditing(false, animated: true)

        tableView.reloadEmptyDataSet()
        editButtonItem.isEnabled = !model.dataProvider.packages.isEmpty
    }

    override func setEditing(_ editing: Bool, animated: Bool) {
        super.setEditing(editing, animated: animated)

        tableView.setEditing(editing, animated: animated)
        navigationItem.leftBarButtonItem = editing ? selectAllBarBtn : nil
    }

    // MARK: - Actions

    @objc
    func actionSelectAll() {
        model.selectAll()
    }
}
