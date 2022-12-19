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

class PackageDetailedView: UIView {
    private(set) weak var delegate: PackageDetailViewDelegate?

    private(set) var package: Package?

    let identifierLabel = KeyValueLabel(key: "detailed-view-identifier-lbl".localized)
    let versionLabel = KeyValueLabel(key: "detailed-view-version-lbl".localized)
    let sectionLabel = KeyValueLabel(key: "detailed-view-section-lbl".localized)
    let installedSizeLabel = KeyValueLabel(key: "detailed-view-installedsize-lbl".localized)

    private(set) lazy var scrollView: UIScrollView = {
        let view = UIScrollView()
        view.alwaysBounceVertical = true
        view.translatesAutoresizingMaskIntoConstraints = false
        view.automaticallyAdjustsScrollIndicatorInsets = false

        return view
    }()

    private(set) lazy var sizesLabel: UILabel = {
        let label = UILabel()
        label.text = "detailed-view-size-lbl".localized
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
        view.translatesAutoresizingMaskIntoConstraints = false
        view.contentMode = .scaleAspectFit
        view.layer.cornerCurve = .continuous
        view.layer.cornerRadius = 14
        view.layer.masksToBounds = true

        return view
    }()

    private(set) lazy var labelsStack: UIStackView = {
        let stack = UIStackView()
        stack.translatesAutoresizingMaskIntoConstraints = false
        stack.spacing = 8.0
        stack.axis = .vertical
        stack.alignment = .top

        stack.layoutMargins = UIEdgeInsets(top: 8, left: 8, bottom: 0, right: 8)
        stack.isLayoutMarginsRelativeArrangement = true

        return stack
    }()

    private(set) lazy var learnMoreButton: UIButton = {
        let button = UIButton(type: .system)
        button.setTitle("detailed-view-learnmore-btn".localized, for: .normal)
        button.addTarget(self, action: #selector(learnMoreTapped), for: .touchUpInside)
        return button
    }()

    private(set) var generalConstraints: [NSLayoutConstraint]?

    init(delegate: PackageDetailViewDelegate) {
        self.delegate = delegate
        super.init(frame: .zero)

        addSubview(scrollView)

        scrollView.addSubview(logoView)
        scrollView.addSubview(labelsStack)

        labelsStack.addArrangedSubview(identifierLabel)
        labelsStack.addArrangedSubview(versionLabel)
        labelsStack.addArrangedSubview(sectionLabel)
        labelsStack.addArrangedSubview(sizesLabel)
        labelsStack.addArrangedSubview(sizesStackView)
        labelsStack.addArrangedSubview(learnMoreButton)

        sizesStackView.addArrangedSubview(installedSizeLabel)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func updateConstraints() {
        super.updateConstraints()

        if generalConstraints == nil {
            let constraints = [
                logoView.topAnchor.constraint(equalTo: scrollView.topAnchor),
                logoHeightConstraint,
                logoView.centerXAnchor.constraint(equalTo: scrollView.centerXAnchor),

                labelsStack.topAnchor.constraint(equalTo: logoView.bottomAnchor),
                labelsStack.leadingAnchor.constraint(equalTo: scrollView.leadingAnchor),
                labelsStack.trailingAnchor.constraint(equalTo: scrollView.trailingAnchor),

                scrollView.topAnchor.constraint(equalTo: topAnchor),
                scrollView.bottomAnchor.constraint(equalTo: bottomAnchor),
                scrollView.leadingAnchor.constraint(equalTo: leadingAnchor),
                scrollView.trailingAnchor.constraint(equalTo: trailingAnchor),
                scrollView.contentLayoutGuide.widthAnchor.constraint(equalTo: widthAnchor)
            ]

            NSLayoutConstraint.activate(constraints)
            generalConstraints = constraints
        }
    }

    func updateContents(forPackage package: Package) {
        self.package = package

        identifierLabel.valueLabel.text = package.id
        versionLabel.valueLabel.text = package.version
        sectionLabel.valueLabel.text = package.section.humanName

        learnMoreButton.isHidden = package.depiction == nil

        if package.installedSize != 0 {
            installedSizeLabel.valueLabel.text = ByteCountFormatter().string(fromByteCount: package.installedSize)
        } else {
            installedSizeLabel.valueLabel.text = "unknown".localized
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
