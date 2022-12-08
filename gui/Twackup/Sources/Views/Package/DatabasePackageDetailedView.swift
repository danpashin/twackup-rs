//
//  DatabasePackageDetailedView.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

extension PackageVC {
    class DatabasePackageDetailedView: PackageDetailedView {
        let debSizeLabel = KeyValueLabel(key: Bundle.appLocalize("detailed-view-debsize-lbl"))

        override init(delegate: PackageDetailViewDelegate) {
            super.init(delegate: delegate)

            learnMoreButton.isHidden = true

            sizesStackView.addArrangedSubview(debSizeLabel)
        }

        required init?(coder: NSCoder) {
            super.init(coder: coder)
        }

        override func updateContents(forPackage package: Package) {
            super.updateContents(forPackage: package)

            if package.debSize != 0 {
                debSizeLabel.valueLabel.text = ByteCountFormatter().string(fromByteCount: package.debSize)
            } else {
                debSizeLabel.valueLabel.text = Bundle.appLocalize("unknown")
            }
        }
    }
}
