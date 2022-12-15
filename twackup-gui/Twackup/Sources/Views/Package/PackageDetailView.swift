//
//  PackageDetailView.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import SDWebImage
import UIKit

protocol PackageDetailViewDelegate: AnyObject {
    func openExternalPackageInfo(_ package: Package)
}

extension PackageVC {
    class PackageDetailedView: UIView {
        private(set) weak var delegate: PackageDetailViewDelegate?

        private(set) var package: Package?

        let identifierLabel = KeyValueLabel(key: Bundle.appLocalize("detailed-view-identifier-lbl"))
        let versionLabel = KeyValueLabel(key: Bundle.appLocalize("detailed-view-version-lbl"))
        let sectionLabel = KeyValueLabel(key: Bundle.appLocalize("detailed-view-section-lbl"))
        let installedSizeLabel = KeyValueLabel(key: Bundle.appLocalize("detailed-view-installedsize-lbl"))

        private(set) lazy var sizesLabel: UILabel = {
            let label = UILabel()
            label.text = Bundle.appLocalize("detailed-view-size-lbl")
            label.font = UIFont.preferredFont(forTextStyle: .headline)
            label.textColor = .secondaryLabel
            return label
        }()

        private(set) lazy var sizesStackView: UIStackView = {
            let stack = UIStackView()
            stack.axis = .vertical
            stack.layoutMargins = UIEdgeInsets(top: 0, left: 16, bottom: 0, right: 0)
            stack.isLayoutMarginsRelativeArrangement = true
            return stack
        }()

        private(set) lazy var logoHeightConstraint = logoView.heightAnchor.constraint(equalToConstant: 0.0)
        private(set) lazy var logoView: UIImageView = {
            let view = UIImageView()

            view.contentMode = .scaleAspectFit
            view.layer.cornerCurve = .continuous
            view.layer.cornerRadius = 14
            view.layer.masksToBounds = true
            return view
        }()

        private(set) lazy var labelsStack: UIStackView = {
            let stack = UIStackView()
            stack.spacing = 8.0
            stack.axis = .vertical
            stack.alignment = .top
            return stack
        }()

        private(set) lazy var learnMoreButton: UIButton = {
            let button = UIButton(type: .system)
            button.setTitle(Bundle.appLocalize("detailed-view-learnmore-btn"), for: .normal)
            button.addTarget(self, action: #selector(learnMoreTapped), for: .touchUpInside)
            return button
        }()

        init(delegate: PackageDetailViewDelegate) {
            self.delegate = delegate
            super.init(frame: .zero)

            addSubview(logoView)
            addSubview(labelsStack)

            labelsStack.addArrangedSubview(identifierLabel)
            labelsStack.addArrangedSubview(versionLabel)
            labelsStack.addArrangedSubview(sectionLabel)
            labelsStack.addArrangedSubview(sizesLabel)
            labelsStack.addArrangedSubview(sizesStackView)
            labelsStack.addArrangedSubview(learnMoreButton)

            sizesStackView.addArrangedSubview(installedSizeLabel)

            logoView.translatesAutoresizingMaskIntoConstraints = false
            labelsStack.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([
                logoView.topAnchor.constraint(equalTo: topAnchor),
                logoHeightConstraint,
                logoView.centerXAnchor.constraint(equalTo: centerXAnchor),

                labelsStack.topAnchor.constraint(equalTo: logoView.bottomAnchor, constant: 8.0),
                labelsStack.leadingAnchor.constraint(equalTo: leadingAnchor),
                labelsStack.trailingAnchor.constraint(equalTo: trailingAnchor)
            ])
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        func updateContents(forPackage package: Package) {
            self.package = package

            identifierLabel.valueLabel.text = package.id
            versionLabel.valueLabel.text = package.version
            sectionLabel.valueLabel.text = package.section.humanName

            if package.installedSize != 0 {
                installedSizeLabel.valueLabel.text = ByteCountFormatter().string(fromByteCount: package.installedSize)
            } else {
                installedSizeLabel.valueLabel.text = Bundle.appLocalize("unknown")
            }

            if let icon = package.icon {
                if icon.isFileURL {
                    logoView.image = UIImage(contentsOfFile: icon.relativePath)
                    logoHeightConstraint.constant = logoView.image != nil ? 60.0 : 0.0
                } else {
                    logoView.sd_setImage(with: icon, placeholderImage: nil) { img, _, _, _ in
                        self.logoHeightConstraint.constant = img != nil ? 60.0 : 0.0
                    }
                }
            } else {
                logoHeightConstraint.constant = 0.0
            }
        }

        @objc
        func learnMoreTapped() {
            guard let package = self.package else { return }
            delegate?.openExternalPackageInfo(package)
        }
    }
}
