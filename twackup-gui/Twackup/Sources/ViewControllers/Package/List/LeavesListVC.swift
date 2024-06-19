//
//  LeavesListVC.swift
//  Twackup
//
//  Created by Daniil on 16.12.2022.
//

import UIKit

class LeavesListVC: DpkgListVC {
    override class var metadata: (any ViewControllerMetadata)? {
        LeavesPkgsMetadata()
    }

    private lazy var shareListButton: UIBarButtonItem = {
        UIBarButtonItem(barButtonSystemItem: .action, target: self, action: #selector(shareList))
    }()

    override func viewDidLoad() {
        super.viewDidLoad()

        navigationItem.leftBarButtonItem = shareListButton
    }

    override func reloadData(animated: Bool, force: Bool) async {
        await super.reloadData(animated: animated, force: force)

        shareListButton.isEnabled = !dataSource.dataProvider.packages.isEmpty
    }

    override func setEditing(_ editing: Bool, animated: Bool) {
        super.setEditing(editing, animated: animated)

        if !editing {
            navigationItem.leftBarButtonItem = shareListButton
        }
    }

    init(mainModel: MainModel, detail: PackageDetailVC<FFIPackage>) {
        super.init(mainModel: mainModel, detail: detail, leaves: true)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    @objc
    func shareList(_ button: UIBarButtonItem) {
        let packagesText = dataSource.dataProvider.packages
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
