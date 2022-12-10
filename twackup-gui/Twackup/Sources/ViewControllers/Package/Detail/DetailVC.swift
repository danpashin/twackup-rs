//
//  PackagesDetailVC.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

extension PackageVC {
    class DetailVC: UIViewController, PackageDetailViewDelegate {
        lazy private(set) var containerView: PackageDetailedView = PackageDetailedView(delegate: self)

        let database: Database

        private(set) var currentPackage: Package?

        init(database: Database) {
            self.database = database
            super.init(nibName: nil, bundle: nil)
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        override func viewDidLoad() {
            super.viewDidLoad()
            navigationController?.navigationBar.prefersLargeTitles = true

            view.addSubview(containerView)
            view.backgroundColor = .systemBackground

            let safeArea = view.safeAreaLayoutGuide

            containerView.translatesAutoresizingMaskIntoConstraints = false
            containerView.isHidden = true
            NSLayoutConstraint.activate([
                containerView.leadingAnchor.constraint(equalTo: safeArea.leadingAnchor, constant: 8.0),
                containerView.trailingAnchor.constraint(equalTo: safeArea.trailingAnchor, constant: -8.0),
                containerView.topAnchor.constraint(equalTo: safeArea.topAnchor, constant: 8.0),
                containerView.bottomAnchor.constraint(equalTo: safeArea.bottomAnchor, constant: -8.0)
            ])

            if let currentPackage {
                didSelectPackage(currentPackage)
            }
        }

        func didSelectPackage(_ package: Package) {
            navigationItem.title = package.name

            currentPackage = package

            containerView.isHidden = false
            containerView.updateContents(forPackage: package)
        }

        func openExternalPackageInfo(_ package: Package) {
            guard let depiction = package.depiction else { return }
            UIApplication.shared.open(depiction)
        }
    }
}
