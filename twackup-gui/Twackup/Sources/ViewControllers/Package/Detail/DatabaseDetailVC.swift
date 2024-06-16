//
//  DatabaseDetailVC.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

class DatabaseDetailVC: PackageDetailVC {
    private lazy var _container = DatabasePackageDetailedView(delegate: self)
    override var detailView: PackageDetailedView { _container }

    private lazy var shareDebButton: UIBarButtonItem = {
        UIBarButtonItem(barButtonSystemItem: .action, target: self, action: #selector(shareDeb))
    }()

    override var package: Package? {
        didSet {
            navigationItem.rightBarButtonItem = package != nil ? shareDebButton : nil
        }
    }

    @objc
    func shareDeb(_ button: UIBarButtonItem) {
        guard let package = package?.asDEB else { return }

        let activityVC = UIActivityViewController(activityItems: [package.fileURL], applicationActivities: nil)
        activityVC.popoverPresentationController?.barButtonItem = button
        present(activityVC, animated: true, completion: nil)
    }
}
