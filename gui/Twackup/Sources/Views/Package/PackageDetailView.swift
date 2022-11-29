//
//  PackageDetailView.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit
import SDWebImage

protocol PackageDetailViewDelegate: AnyObject {
    func openExternalPackageInfo(_ package: Package)

}

extension PackageVC {
    class PackageDetailedView: UIView {
        private(set) var delegate: PackageDetailViewDelegate?

        private(set) var package: Package?

        let identifierLabel = KeyValueLabel(key: "Identifier:")
        let versionLabel = KeyValueLabel(key: "Version:")
        let sectionLabel = KeyValueLabel(key: "Section:")

        lazy private(set) var logoHeightConstraint = logoView.heightAnchor.constraint(equalToConstant: 0.0)
        lazy private(set) var logoView: UIImageView = {
            let view = UIImageView()

            view.contentMode = .scaleAspectFit
            view.layer.cornerCurve = .continuous
            view.layer.cornerRadius = 14
            view.layer.masksToBounds = true
            return view
        }()

        lazy private(set) var labelsStack: UIStackView = {
            let stack = UIStackView()
            stack.spacing = 8.0
            stack.axis = .vertical
            stack.alignment = .top
            return stack
        }()

        lazy private(set) var learnMoreButton: UIButton = {
            let button = UIButton(type: .system)
            button.setTitle("Learn More", for: .normal)
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
            labelsStack.addArrangedSubview(learnMoreButton)

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
            sectionLabel.valueLabel.text = package.section.humanName()

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

        @objc func learnMoreTapped() {
            guard let package = self.package else { return }
            delegate?.openExternalPackageInfo(package)
        }
    }
}
