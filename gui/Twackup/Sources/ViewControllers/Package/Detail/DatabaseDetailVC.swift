//
//  DatabaseDetailVC.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

extension PackageVC {
    class DatabaseDetailVC: DetailVC {
        private lazy var _container = DatabasePackageDetailedView(delegate: self)
        override var containerView: PackageDetailedView { _container }

        private var debURL: URL?

        private lazy var sharingBtn: UIBarButtonItem = {
            let shareImage = UIImage(systemName: "square.and.arrow.up")
            return UIBarButtonItem(image: shareImage, style: .plain, target: self, action: #selector(share))
        }()

        override func didSelectPackage(_ package: Package) {
            super.didSelectPackage(package)

            navigationItem.rightBarButtonItem = sharingBtn
        }

        @objc func share() {
            guard let package = currentPackage, let package = database.fetch(package: package) else { return }
            let items = [package.fileURL()]

            let activityViewController = UIActivityViewController(activityItems: items, applicationActivities: nil)
            activityViewController.popoverPresentationController?.sourceView = view

            present(activityViewController, animated: true, completion: nil)
        }
    }
}
