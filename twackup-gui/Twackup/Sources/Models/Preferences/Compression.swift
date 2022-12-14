//
//  Compression.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

struct Compression {
    enum Kind: String, Identifiable, AsString {
        case gzip = "GZip"
        case xzip = "XZip"
        case zst = "ZStd"

        static let allValues: [Kind] = [.gzip, .xzip, .zst]

        var id: RawValue { rawValue }

        var asString: String { rawValue }
    }

    enum Level: Int, Identifiable, AsString {
        case none = 0
        case fast = 1
        case normal = 6
        case best = 9

        static let allValues: [Level] = [.none, .fast, .normal, .best]

        var id: RawValue { rawValue }

        var asString: String {
            switch self {
            case .none: return "None"
            case .fast: return "Fast"
            case .normal: return "Normal"
            case .best: return "Best"
            }
        }
    }

    var kind: Kind = .gzip
    var level: Level = .normal
}
