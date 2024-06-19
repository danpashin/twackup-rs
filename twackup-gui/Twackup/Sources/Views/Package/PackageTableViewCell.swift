//
//  PackageTableViewCell.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class PackageTableViewCell<P: Package>: UITableViewCell {
    var package: P?

    override func updateConfiguration(using state: UICellConfigurationState) {
        super.updateConfiguration(using: state)
        accessoryType = .disclosureIndicator

        var config = defaultContentConfiguration().updated(for: state)

        config.directionalLayoutMargins.top = 6.0
        config.directionalLayoutMargins.bottom = 6.0

        config.textProperties.font = UIFont.systemFont(ofSize: UIFont.labelFontSize, weight: .semibold)
        config.textToSecondaryTextVerticalPadding = 2.0

        config.secondaryTextProperties.font = UIFont.systemFont(ofSize: UIFont.smallSystemFontSize, weight: .regular)
        config.secondaryTextProperties.color = .secondaryLabel
        config.secondaryTextProperties.numberOfLines = 2

        if let package {
            setup(config: &config, for: package)
        }

        contentConfiguration = config
    }

    func setup(config: inout UIListContentConfiguration, for package: P) {
        config.image = UIImage(systemName: package.section.systemImageName)
        config.text = package.name
        config.secondaryText = package.humanDescription?.truncate(70)
    }
}
