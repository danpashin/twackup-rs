//
//  PackageTableViewCell.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

extension String {
    func truncate(_ length: Int, trailing: String = "...") -> String {
        return (count > length) ? prefix(length) + trailing : self
    }
}


class PackageTableViewCell: UITableViewCell {

    var package: Package? {
        didSet {
            updateUI()
        }
    }

    lazy private var config: UIListContentConfiguration = {
        var config = defaultContentConfiguration()

        config.directionalLayoutMargins = NSDirectionalEdgeInsets(top: 6.0, leading: 0.0, bottom: 6.0, trailing: 0.0)

        config.textProperties.font = UIFont.systemFont(ofSize: UIFont.labelFontSize, weight: .semibold)
        config.textToSecondaryTextVerticalPadding = 0.0

        config.secondaryTextProperties.font = UIFont.systemFont(ofSize: UIFont.smallSystemFontSize, weight: .regular)
        config.secondaryTextProperties.color = .secondaryLabel
        config.secondaryTextProperties.numberOfLines = 2

        return config
    }()

    override init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: .subtitle, reuseIdentifier: reuseIdentifier)

        accessoryType = .disclosureIndicator
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    private func updateUI() {
        guard let package else { return }

        config.image = UIImage(systemName: package.section.systemImageName())
        config.text = package.name
        config.secondaryText = package.description.truncate(70)

        contentConfiguration = config
    }
}
