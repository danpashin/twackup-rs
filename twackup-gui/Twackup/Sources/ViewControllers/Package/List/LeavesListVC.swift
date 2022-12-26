//
//  LeavesListVC.swift
//  Twackup
//
//  Created by Daniil on 16.12.2022.
//

import UIKit

class LeavesListVC: DpkgListVC {
    private lazy var shareListButton: UIBarButtonItem = {
        UIBarButtonItem(barButtonSystemItem: .action, target: self, action: #selector(shareList))
    }()

    override func viewDidLoad() {
        super.viewDidLoad()

        navigationItem.leftBarButtonItem = shareListButton
    }

    override func endReloadingData() {
        super.endReloadingData()
        shareListButton.isEnabled = !model.dataProvider.packages.isEmpty
    }

    override func setEditing(_ editing: Bool, animated: Bool) {
        super.setEditing(editing, animated: animated)

        if !editing {
            navigationItem.leftBarButtonItem = shareListButton
        }
    }

    @objc
    func shareList(_ button: UIBarButtonItem) {
        let packagesText = model.dataProvider.packages
            .enumerated()
            .map { index, package in
                String(format: "%3d. %@ - %@", index + 1, package.id, package.version)
            }
            .joined(separator: "\n")

        let activityVC = UIActivityViewController(activityItems: [packagesText], applicationActivities: nil)
        activityVC.popoverPresentationController?.barButtonItem = button
        present(activityVC, animated: true, completion: nil)
    }
}
