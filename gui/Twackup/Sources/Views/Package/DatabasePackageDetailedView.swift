//
//  DatabasePackageDetailedView.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import Foundation

extension PackageVC {
    class DatabasePackageDetailedView: PackageDetailedView {
        override init(delegate: PackageDetailViewDelegate) {
            super.init(delegate: delegate)

            learnMoreButton.isHidden = true
        }

        required init?(coder: NSCoder) {
            super.init(coder: coder)
        }
    }
}
