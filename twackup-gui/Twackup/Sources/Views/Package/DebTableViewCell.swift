//
//  DebTableViewCell.swift
//  Twackup
//
//  Created by Daniil on 16.12.2022.
//

import UIKit

class DebTableViewCell: PackageTableViewCell<DebPackage> {
    override func setup(config: inout UIListContentConfiguration, for package: DebPackage) {
        config.image = UIImage(systemName: package.section.systemImageName)
        config.text = package.name
        config.secondaryText = package.version
    }
}
