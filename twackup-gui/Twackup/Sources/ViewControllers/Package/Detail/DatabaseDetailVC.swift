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

        private lazy var shareDebButton: UIBarButtonItem = {
            UIBarButtonItem(barButtonSystemItem: .action, target: self, action: #selector(shareDeb))
        }()

        override func didSelectPackage(_ package: Package) {
            super.didSelectPackage(package)

            navigationItem.rightBarButtonItem = shareDebButton
        }

        @objc
        func shareDeb() {
            guard let package = currentPackage as? DebPackage else { return }

            let activityVC = UIActivityViewController(activityItems: [package.fileURL()], applicationActivities: nil)
            activityVC.popoverPresentationController?.barButtonItem = shareDebButton
            present(activityVC, animated: true, completion: nil)
        }
    }
}
