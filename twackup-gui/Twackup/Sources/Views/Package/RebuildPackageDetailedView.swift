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

        label.numberOfLines = 0
        label.text = "detailed-view-rebuild-btn-footer".localized
        label.font = UIFont.preferredFont(forTextStyle: .caption1)
        label.textAlignment = .center
        label.textColor = .secondaryLabel

        return label
    }()

    private(set) lazy var rebuildFooterView: UIStackView = {
        let stack = UIStackView(arrangedSubviews: [rebuildButton, rebuildWarningLabel])
        stack.axis = .vertical
        stack.alignment = .center
        stack.distribution = .equalSpacing
        stack.spacing = 8.0
        stack.translatesAutoresizingMaskIntoConstraints = false
        stack.backgroundColor = .systemBackground

        stack.layoutMargins = UIEdgeInsets(top: 16, left: 0, bottom: 16, right: 0)
        stack.isLayoutMarginsRelativeArrangement = true

        return stack
    }()

    private(set) var rebuildFooterConstraints: [NSLayoutConstraint]?

    init(delegate: RebuildPackageDetailedViewDelegate) {
        super.init(delegate: delegate)

        addSubview(rebuildFooterView)
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
    }

    override func updateConstraints() {
        super.updateConstraints()

        if rebuildFooterConstraints == nil {
            let safeArea = safeAreaLayoutGuide
            let constraints = [
                rebuildFooterView.leadingAnchor.constraint(equalTo: safeArea.leadingAnchor, constant: 8.0),
                rebuildFooterView.trailingAnchor.constraint(equalTo: safeArea.trailingAnchor, constant: -8.0),
                rebuildFooterView.bottomAnchor.constraint(equalTo: safeArea.bottomAnchor)
            ]

            NSLayoutConstraint.activate(constraints)
            rebuildFooterConstraints = constraints
        }
    }

    @objc
    func rebuild() {
        guard let package = self.package else { return }
        (delegate as? any RebuildPackageDetailedViewDelegate)?.rebuild(package)
    }
}
