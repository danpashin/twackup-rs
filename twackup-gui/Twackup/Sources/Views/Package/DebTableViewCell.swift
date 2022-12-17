//
//  DebTableViewCell.swift
//  Twackup
//
//  Created by Daniil on 16.12.2022.
//

import UIKit

class DebTableViewCell: PackageTableViewCell {
    override func updateUI() {
        guard let package else { return }

        config.image = UIImage(systemName: package.section.systemImageName)
        config.text = package.name
        config.secondaryText = package.version

        contentConfiguration = config
    }
}
