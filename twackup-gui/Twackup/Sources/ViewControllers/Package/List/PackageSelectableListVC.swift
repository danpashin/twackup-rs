//
//  PackageSelectableListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import UIKit
import DZNEmptyDataSet

class PackageSelectableListVC: PackageListVC {
    private(set) lazy var editBarBtn: UIBarButtonItem = {
        UIBarButtonItem(barButtonSystemItem: .edit, target: self, action: #selector(actionEdit))
    }()

    private(set) lazy var editDoneBarBtn: UIBarButtonItem = {
        UIBarButtonItem(barButtonSystemItem: .done, target: self, action: #selector(actionDoneEdit))
    }()

    private(set) lazy var selectAllBarBtn: UIBarButtonItem = {
        let title = "debs-selectall-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionSelectAll))
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        tableView.allowsMultipleSelectionDuringEditing = true

        navigationItem.rightBarButtonItem = editBarBtn
    }

    override func endReloadingData() {
        super.endReloadingData()

        actionDoneEdit()

        tableView.reloadEmptyDataSet()
        editBarBtn.isEnabled = !model.dataProvider.packages.isEmpty
    }

    @objc
    func actionEdit() {
        tableView.setEditing(true, animated: true)

        navigationItem.leftBarButtonItem = selectAllBarBtn
        navigationItem.rightBarButtonItem = editDoneBarBtn
    }

    @objc
    func actionDoneEdit() {
        tableView.setEditing(false, animated: true)

        navigationItem.leftBarButtonItem = nil
        navigationItem.rightBarButtonItem = editBarBtn
    }

    @objc
    func actionSelectAll() {
        model.selectAll()
    }
}
