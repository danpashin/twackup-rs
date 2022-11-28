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

    func rebuild(_ package: Package)
}

extension Views.Package {
    class DetailView: UIView {
        private(set) var delegate: PackageDetailViewDelegate?

        private(set) var package: Package?

        let identifierLabel = Views.KeyValueLabel(key: "Identifier:")
        let versionLabel = Views.KeyValueLabel(key: "Version:")
        let sectionLabel = Views.KeyValueLabel(key: "Section:")

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

        init(delegate: PackageDetailViewDelegate) {
            self.delegate = delegate
            super.init(frame: .zero)

            addSubview(logoView)
            addSubview(labelsStack)
            addSubview(rebuildButton)

            labelsStack.addArrangedSubview(identifierLabel)
            labelsStack.addArrangedSubview(versionLabel)
            labelsStack.addArrangedSubview(sectionLabel)
            labelsStack.addArrangedSubview(learnMoreButton)

            logoView.translatesAutoresizingMaskIntoConstraints = false
            labelsStack.translatesAutoresizingMaskIntoConstraints = false
            rebuildButton.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([
                logoView.topAnchor.constraint(equalTo: topAnchor),
                logoHeightConstraint,
                logoView.centerXAnchor.constraint(equalTo: centerXAnchor),

                labelsStack.topAnchor.constraint(equalTo: logoView.bottomAnchor, constant: 8.0),
                labelsStack.leadingAnchor.constraint(equalTo: leadingAnchor),
                labelsStack.trailingAnchor.constraint(equalTo: trailingAnchor),

                rebuildButton.centerXAnchor.constraint(equalTo: centerXAnchor),
                rebuildButton.bottomAnchor.constraint(equalTo: bottomAnchor, constant: -8.0)
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

        @objc func rebuild() {
            guard let package = self.package else { return }
            delegate?.rebuild(package)
        }
    }
}
