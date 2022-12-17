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

class RebuildPackageDetailedView: PackageDetailedView {
    private(set) lazy var rebuildButton: UIButton = {
        let button = UIButton(type: .system)
        button.addTarget(self, action: #selector(rebuild), for: .touchUpInside)
        button.translatesAutoresizingMaskIntoConstraints = false

        button.setTitle("detailed-view-rebuild-btn".localized, for: .normal)
        button.titleLabel?.font = UIFont.systemFont(ofSize: UIFont.buttonFontSize, weight: .semibold)

        button.setImage(UIImage(systemName: "shippingbox"), for: .normal)
        button.imageEdgeInsets = UIEdgeInsets(top: 0, left: -20, bottom: 0, right: 0)

        button.backgroundColor = .quaternarySystemFill
        button.contentEdgeInsets = UIEdgeInsets(top: 20.0, left: 64.0, bottom: 20.0, right: 64.0)

        button.layer.cornerCurve = .continuous
        button.layer.cornerRadius = 20.0

        return button
    }()

    private(set) lazy var rebuildWarningLabel: UILabel = {
        let label = UILabel()
        label.translatesAutoresizingMaskIntoConstraints = false

        label.numberOfLines = 0
        label.text = "detailed-view-rebuild-btn-footer".localized
        label.font = UIFont.preferredFont(forTextStyle: .caption1)
        label.textAlignment = .center
        label.textColor = .secondaryLabel

        return label
    }()

    private(set) var rebuildButtonConstraints: [NSLayoutConstraint]?

    init(delegate: RebuildPackageDetailedViewDelegate) {
        super.init(delegate: delegate)

        addSubview(rebuildButton)
        addSubview(rebuildWarningLabel)
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
    }

    override func updateConstraints() {
        super.updateConstraints()

        if rebuildButtonConstraints == nil {
            let constraints = [
                rebuildWarningLabel.leadingAnchor.constraint(equalTo: leadingAnchor),
                rebuildWarningLabel.trailingAnchor.constraint(equalTo: trailingAnchor),
                rebuildWarningLabel.bottomAnchor.constraint(equalTo: bottomAnchor, constant: -8.0),

                rebuildButton.centerXAnchor.constraint(equalTo: centerXAnchor),
                rebuildButton.bottomAnchor.constraint(equalTo: rebuildWarningLabel.topAnchor, constant: -8.0)
            ]

            NSLayoutConstraint.activate(constraints)
            rebuildButtonConstraints = constraints
        }
    }

    @objc
    func rebuild() {
        guard let package = self.package else { return }
        (delegate as? any RebuildPackageDetailedViewDelegate)?.rebuild(package)
    }
}
