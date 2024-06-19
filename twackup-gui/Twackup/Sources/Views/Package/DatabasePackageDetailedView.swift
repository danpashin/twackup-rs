//
//  DatabasePackageDetailedView.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

class DatabasePackageDetailedView: PackageDetailedView<DebPackage> {
    let debSizeLabel = KeyValueLabel(key: "detailed-view-debsize-lbl".localized)

    override init(delegate: PackageDetailViewDelegate) {
        super.init(delegate: delegate)

        sizesStackView.addArrangedSubview(debSizeLabel)
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
    }

    override func updateContents(forPackage package: DebPackage) {
        super.updateContents(forPackage: package)
        learnMoreButton.isHidden = true

        // float value comparement logic
        if package.debSize.value > 1 {
            debSizeLabel.valueLabel.text = ByteCountFormatter().string(from: package.debSize)
        } else {
            debSizeLabel.valueLabel.text = "unknown".localized
        }
    }
}
