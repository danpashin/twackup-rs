//
//  Package.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

import Foundation

protocol Package: AnyObject {
    var id: String { get }

    var name: String { get }

    var version: String { get }

    var section: PackageSection { get }

    var architecture: String? { get }

    var icon: URL? { get }

    var depiction: URL? { get }

    var humanDescription: String? { get }
}
