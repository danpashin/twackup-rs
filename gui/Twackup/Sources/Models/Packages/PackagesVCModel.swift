//
//  PackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

protocol PackageDetailDelegate {
    func didSelectPackage(_ package: Package)
}

protocol PackagesVCModel {
    var navTitle: String { get }

    var tabbarItem: UITabBarItem { get }

    var packages: [Package] { mutating get }

    var detailDelegate: PackageDetailDelegate? { get set }

    init(_ dpkg: Dpkg)
}
