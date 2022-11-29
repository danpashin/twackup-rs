//
//  RebuildPackageDetailedView.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

protocol RebuildPackageDetailedViewDelegate: PackageDetailViewDelegate {
    func rebuild(_ package: Package)
}

extension PackageVC {
    class RebuildPackageDetailedView: PackageDetailedView {

        lazy private(set) var rebuildButton: UIButton = {
            let button = UIButton(type: .system)
            button.addTarget(self, action: #selector(rebuild), for: .touchUpInside)

            button.setTitle("REBUILD", for: .normal)
            button.titleLabel?.font = UIFont.systemFont(ofSize: UIFont.buttonFontSize, weight: .semibold)

            button.setImage(UIImage(systemName: "shippingbox"), for: .normal)
            button.imageEdgeInsets = UIEdgeInsets(top: 0, left: -20, bottom: 0, right: 0)

            button.backgroundColor = .quaternarySystemFill
            button.contentEdgeInsets = UIEdgeInsets(top: 20.0, left: 64.0, bottom: 20.0, right: 64.0)

            button.layer.cornerCurve = .continuous
            button.layer.cornerRadius = 20.0

            return button
        }()

        init(delegate: RebuildPackageDetailedViewDelegate) {
            super.init(delegate: delegate)

            addSubview(rebuildButton)

            rebuildButton.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([
                rebuildButton.centerXAnchor.constraint(equalTo: centerXAnchor),
                rebuildButton.bottomAnchor.constraint(equalTo: bottomAnchor, constant: -8.0)
            ])
        }

        override init(delegate: PackageDetailViewDelegate) {
            fatalError("Wrong delegate")
        }

        required init?(coder: NSCoder) {
            super.init(coder: coder)
        }

        @objc func rebuild() {
            guard let package = self.package else { return }
            (delegate as? any RebuildPackageDetailedViewDelegate)?.rebuild(package)
        }
    }
}
