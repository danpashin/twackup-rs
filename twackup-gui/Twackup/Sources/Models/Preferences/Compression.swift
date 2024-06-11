//
//  Compression.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

protocol Localized {
    var localized: String { get }
}

@MainActor
class Compression: ObservableObject {
    enum Kind: Int, Identifiable, Localized, CaseIterable {
        case gzip
        case xzip
        case zst
        case bzip2

        var id: RawValue { rawValue }

        var localized: String {
            var stringRepr: String
            switch self {
            case .gzip: stringRepr = "gzip"
            case .xzip: stringRepr = "xzip"
            case .zst: stringRepr = "zstd"
            case .bzip2: stringRepr = "bzip2"
            }

            return stringRepr.capitalized
        }
    }

    enum Level: Int, Identifiable, Localized, CaseIterable {
        case none
        case fast
        case normal
        case best

        var id: RawValue { rawValue }

        var localized: String {
            var suffix: String
            switch self {
            case .none: suffix = "none"
            case .fast: suffix = "fast"
            case .normal: suffix = "normal"
            case .best: suffix = "best"
            }

            return "compression-level-\(suffix)".localized
        }
    }

    @AppStorage("package-compression-kind")
    var kind: Kind = .gzip

    @AppStorage("package-compression-level")
    var level: Level = .normal
}
