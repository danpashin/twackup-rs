//
//  PackagesDataProvider.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import UIKit

enum PackageFilter {
    case Name(String)
}

protocol PackagesDataProvider {
    var navTitle: String { get }

    var tabbarItem: UITabBarItem { get }

    var packages: [Package] { mutating get }

    init(_ dpkg: Dpkg)

    mutating func filter(_ filter: PackageFilter?)
}
