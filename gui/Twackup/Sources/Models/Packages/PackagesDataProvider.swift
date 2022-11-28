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

protocol Package {
    var id: String { get }

    var name: String  { get }

    var version: String  { get }

    var section: PackageSection  { get }

    var architecture: String { get }

    var icon: URL? { get }

    var depiction: URL? { get }

    var description: String { get }
}

protocol PackagesDataProvider {
    var navTitle: String { get }

    var tabbarItem: UITabBarItem { get }

    var packages: [Package] { get }

    mutating func filter(_ filter: PackageFilter?)
}
