//
//  Compression.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

protocol AsString {
    var asString: String { get }

    var localized: String { get }
}

class Compression: ObservableObject {
    enum Kind: Int, Identifiable, AsString, CaseIterable, Equatable {
        case gzip
        case xzip
        case zst

        var id: RawValue { rawValue }

        var asString: String {
            switch self {
            case .gzip: return "gz"
            case .xzip: return "xz"
            case .zst: return "zst"
            }
        }

        var localized: String { asString.capitalized }
    }

    enum Level: Int, Identifiable, AsString, CaseIterable {
        case none
        case fast
        case normal
        case best

        var id: RawValue { rawValue }

        var asString: String {
            switch self {
            case .none: return "None"
            case .fast: return "Fast"
            case .normal: return "Normal"
            case .best: return "Best"
            }
        }

        var localized: String {
            Bundle.appLocalize("compression-level-\(asString.lowercased())")
        }
    }

    @AppStorage("package-compression-kind")
    var kind: Kind = .gzip

    @AppStorage("package-compression-level")
    var level: Level = .normal
}
