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

        override func didSelectPackage(_ package: Package) {
            super.didSelectPackage(package)

            debURL = package.debDefaultURL()

            addSharingMenu()
        }

        func addSharingMenu() {
            let shareImage = UIImage(systemName: "square.and.arrow.up")
            let btn = UIBarButtonItem(image: shareImage, style: .plain, target: self, action: #selector(share))
            navigationItem.rightBarButtonItem = btn
        }

        @objc func share() {
            let items = [debURL!]
            let activityViewController = UIActivityViewController(activityItems: items, applicationActivities: nil)
            activityViewController.popoverPresentationController?.sourceView = self.view

            self.present(activityViewController, animated: true, completion: nil)
        }
    }
}
