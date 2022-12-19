//
//  PackagesDetailVC.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class DetailVC: UIViewController, PackageDetailViewDelegate {
    private(set) lazy var detailView = PackageDetailedView(delegate: self)

    let mainModel: MainModel

    var package: Package? {
        didSet {
            navigationItem.title = package?.name

            if let package {
                detailView.updateContents(forPackage: package)
            }

            detailView.isHidden = package == nil
        }
    }

    private(set) var detailViewConstraints: [NSLayoutConstraint]?

    init(mainModel: MainModel) {
        self.mainModel = mainModel
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        view.addSubview(detailView)
        view.backgroundColor = .systemBackground

        detailView.translatesAutoresizingMaskIntoConstraints = false
        detailView.isHidden = package == nil
    }

    override func updateViewConstraints() {
        super.updateViewConstraints()

        if detailViewConstraints == nil {
            let constraints = [
                detailView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
                detailView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
                detailView.topAnchor.constraint(equalTo: view.topAnchor),
                detailView.bottomAnchor.constraint(equalTo: view.bottomAnchor)
            ]

            NSLayoutConstraint.activate(constraints)
            detailViewConstraints = constraints
        }
    }

    func openExternalPackageInfo(_ package: Package) {
        guard let depiction = package.depiction else { return }
        UIApplication.shared.open(depiction)
    }
}
