//
//  DatabaseDetailVC.swift
//  Twackup
//
//  Created by Daniil on 29.11.2022.
//

import UIKit

extension PackageVC {
    class DatabaseDetailVC: DetailVC {
        private lazy var _container = DatabasePackageDetailedView(delegate: self)
        override var containerView: PackageDetailedView { _container }
    }
}
