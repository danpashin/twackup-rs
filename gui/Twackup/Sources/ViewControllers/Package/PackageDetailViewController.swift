//
//  PackagesDetailVC.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class PackageDetailViewController: UIViewController, PackageDetailDelegate, PackageDetailViewDelegate {

    lazy var containerView = PackageDetailView(delegate: self)

    override func viewDidLoad() {
        super.viewDidLoad()
        navigationController?.navigationBar.prefersLargeTitles = true

        view.addSubview(containerView)
        view.backgroundColor = .systemBackground

        containerView.translatesAutoresizingMaskIntoConstraints = false
        containerView.isHidden = true
        NSLayoutConstraint.activate([
            containerView.leadingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.leadingAnchor, constant: 8.0),
            containerView.trailingAnchor.constraint(equalTo: view.safeAreaLayoutGuide.trailingAnchor, constant: -8.0),
            containerView.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 8.0),
            containerView.bottomAnchor.constraint(equalTo: view.safeAreaLayoutGuide.bottomAnchor, constant: -8.0),
        ])
    }

    func didSelectPackage(_ package: Package) {
        navigationItem.title = package.name

        containerView.isHidden = false
        containerView.updateContents(forPackage: package)
    }

    func openExternalPackageInfo(_ package: Package) {
        guard let depiction = package.depiction else { return }
        UIApplication.shared.open(depiction)
    }

    func rebuild(_ package: Package) {

    }
}
