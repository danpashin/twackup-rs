//
//  PackageTableViewCell.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class PackageTableViewCell: UITableViewCell {
    var package: Package? {
        didSet {
            updateUI()
        }
    }

    lazy var config: UIListContentConfiguration = {
        var cfg = defaultContentConfiguration()

        cfg.directionalLayoutMargins.top = 6.0
        cfg.directionalLayoutMargins.bottom = 6.0

        cfg.textProperties.font = UIFont.systemFont(ofSize: UIFont.labelFontSize, weight: .semibold)
        cfg.textToSecondaryTextVerticalPadding = 2.0

        cfg.secondaryTextProperties.font = UIFont.systemFont(ofSize: UIFont.smallSystemFontSize, weight: .regular)
        cfg.secondaryTextProperties.color = .secondaryLabel
        cfg.secondaryTextProperties.numberOfLines = 2

        return cfg
    }()

    override init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: .subtitle, reuseIdentifier: reuseIdentifier)

        accessoryType = .disclosureIndicator
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    func updateUI() {
        guard let package else { return }

        config.image = UIImage(systemName: package.section.systemImageName)
        config.text = package.name
        config.secondaryText = package.humanDescription?.truncate(70)

        contentConfiguration = config
    }
}
