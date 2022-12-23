//
//  Package.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

/// An abstract instance, describing Debian package format
protocol Package: AnyObject {
    /// Unique package identifier
    var id: String { get }

    /// Name of package that displays in every package manager
    var name: String { get }

    /// Version of package
    var version: String { get }

    /// This field specifies an application area into which
    /// the package has been classified
    var section: PackageSection { get }

    /// The architecture specifies which type of hardware this
    /// package was compiled for.
    var architecture: String? { get }

    /// Package icon. Can be file or http URL
    var icon: URL? { get }

    /// The upstream project home page url
    var depiction: URL? { get }

    /// The format for the package description is a short brief
    /// summary on the first line (after the Description field).
    /// The following lines should be used as a longer, more
    /// detailed description. Each line of the long description
    /// must be preceded by a space, and blank lines in the long
    /// description must contain a single ‘.’ following the
    /// preceding space
    var humanDescription: String? { get }

    /// The approximate total size of the package's installed files
    var installedSize: Int64 { get }

    func isEqualTo(_ other: Package) -> Bool
}

extension Package {
    var asFFI: FFIPackage? { self as? FFIPackage }

    var asDEB: DebPackage? { self as? DebPackage }
}
