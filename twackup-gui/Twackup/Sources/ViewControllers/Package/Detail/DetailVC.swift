//
//  PackagesDetailVC.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class DetailVC: UIViewController, PackageDetailViewDelegate {
    private(set) lazy var containerView: PackageDetailedView = PackageDetailedView(delegate: self)

    let mainModel: MainModel

    var package: Package? {
        didSet {
            navigationItem.title = package?.name

            if let package {
                containerView.isHidden = false
                containerView.updateContents(forPackage: package)
            } else {
                containerView.isHidden = true
            }
        }
    }

    private(set) var generalConstraints: [NSLayoutConstraint]?

    init(mainModel: MainModel) {
        self.mainModel = mainModel
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        view.addSubview(containerView)
        view.backgroundColor = .systemBackground

        containerView.translatesAutoresizingMaskIntoConstraints = false
        containerView.isHidden = true
    }

    override func updateViewConstraints() {
        super.updateViewConstraints()

        if generalConstraints == nil {
            let safeArea = view.safeAreaLayoutGuide
            let constraints = [
                containerView.leadingAnchor.constraint(equalTo: safeArea.leadingAnchor, constant: 8.0),
                containerView.trailingAnchor.constraint(equalTo: safeArea.trailingAnchor, constant: -8.0),
                containerView.topAnchor.constraint(equalTo: safeArea.topAnchor, constant: 8.0),
                containerView.bottomAnchor.constraint(equalTo: safeArea.bottomAnchor, constant: -8.0)
            ]

            NSLayoutConstraint.activate(constraints)
            generalConstraints = constraints
        }
    }

    func openExternalPackageInfo(_ package: Package) {
        guard let depiction = package.depiction else { return }
        UIApplication.shared.open(depiction)
    }
}
