//
//  PackageSection.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

@objc
enum PackageSection: UInt16 {
    case archiving
    case development
    case networking
    case packaging
    case system
    case terminalSupport
    case textEditors
    case themes
    case tweaks
    case utilities
    case other

    var systemImageName: String {
        switch self {
        case .archiving: return "doc.zipper"
        case .development: return "cpu"
        case .networking: return "network"
        case .packaging: return "archivebox"
        case .system: return "command"
        case .terminalSupport: return "terminal"
        case .textEditors: return "doc.text"
        case .themes: return "lasso.sparkles"
        case .tweaks: return "gearshape"
        case .utilities: return "keyboard"
        case .other: return "cube"
        }
    }

    var humanName: String {
        switch self {
        case .archiving: return "Archiving"
        case .development: return "Development"
        case .networking: return "Networking"
        case .packaging: return "Packaging"
        case .system: return "System"
        case .terminalSupport: return "Terminal support"
        case .textEditors: return "Text editors"
        case .themes: return "Themes"
        case .tweaks: return "Tweaks"
        case .utilities: return "Utilities"
        case .other: return "Other"
        }
    }
}

extension TwPackageSection_t {
    var swiftSection: PackageSection {
        switch self {
        case TW_PACKAGE_SECTION_ARCHIVING.clampedToU8: return .archiving
        case TW_PACKAGE_SECTION_DEVELOPMENT.clampedToU8: return .development
        case TW_PACKAGE_SECTION_NETWORKING.clampedToU8: return .networking
        case TW_PACKAGE_SECTION_PACKAGING.clampedToU8: return .packaging
        case TW_PACKAGE_SECTION_SYSTEM.clampedToU8: return .system
        case TW_PACKAGE_SECTION_TERMINAL_SUPPORT.clampedToU8: return .terminalSupport
        case TW_PACKAGE_SECTION_TEXT_EDITORS.clampedToU8: return .textEditors
        case TW_PACKAGE_SECTION_THEMES.clampedToU8: return .themes
        case TW_PACKAGE_SECTION_TWEAKS.clampedToU8: return .tweaks
        case TW_PACKAGE_SECTION_UTILITIES.clampedToU8: return .utilities

        default: return .other
        }
    }
}
