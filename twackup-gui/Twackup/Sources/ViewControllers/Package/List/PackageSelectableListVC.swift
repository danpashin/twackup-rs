//
//  PackageSelectableListVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import UIKit

extension PackageVC {
    class PackageSelectableListVC: PackageListVC {
        var selectedPackages: [Package]? {
            guard let selected = tableView.indexPathsForSelectedRows?.map({ $0.row }) else { return nil }
            let enumerated = model.dataProvider.packages.enumerated()
            return enumerated.filter { selected.contains($0.offset) }.map { $1 }
        }

        private(set) lazy var editBarBtn: UIBarButtonItem = {
            UIBarButtonItem(barButtonSystemItem: .edit, target: self, action: #selector(actionEdit))
        }()

        private(set) lazy var editDoneBarBtn: UIBarButtonItem = {
            UIBarButtonItem(barButtonSystemItem: .done, target: self, action: #selector(actionDoneEdit))
        }()

        private(set) lazy var selectAllBarBtn: UIBarButtonItem = {
            let title = Bundle.appLocalize("debs-selectall-btn")
            return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionSelectAll))
        }()

        override func viewDidLoad() {
            super.viewDidLoad()
            tableView.allowsMultipleSelectionDuringEditing = true

            navigationItem.rightBarButtonItem = editBarBtn
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
            for row in 0..<model.dataProvider.packages.count {
                tableView.selectRow(at: IndexPath(row: row, section: 0), animated: true, scrollPosition: .none)
            }
        }
    }
}
