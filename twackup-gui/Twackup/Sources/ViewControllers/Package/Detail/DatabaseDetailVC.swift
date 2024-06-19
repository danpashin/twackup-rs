//
//  DatabaseDetailVC.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

class DatabaseDetailVC: PackageDetailVC<DebPackage> {
    private lazy var _container = DatabasePackageDetailedView(delegate: self)
    override var detailView: PackageDetailedView<DebPackage> { _container }

    private lazy var shareDebButton: UIBarButtonItem = {
        UIBarButtonItem(barButtonSystemItem: .action, target: self, action: #selector(shareDeb))
    }()

    override var package: DebPackage? {
        didSet {
            navigationItem.rightBarButtonItem = package != nil ? shareDebButton : nil
        }
    }

    @objc
    func shareDeb(_ button: UIBarButtonItem) {
        guard let package else { return }

        let activityVC = UIActivityViewController(activityItems: [package.fileURL], applicationActivities: nil)
        activityVC.popoverPresentationController?.barButtonItem = button
        present(activityVC, animated: true, completion: nil)
    }
}
