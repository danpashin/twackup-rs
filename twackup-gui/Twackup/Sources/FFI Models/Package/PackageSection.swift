//
//  PackageSection.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import Foundation

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

    init(_ section: TwPackageSection_t) {
        switch section {
        case TwPackageSection_t(TW_PACKAGE_SECTION_ARCHIVING): self = .archiving
        case TwPackageSection_t(TW_PACKAGE_SECTION_DEVELOPMENT): self = .development
        case TwPackageSection_t(TW_PACKAGE_SECTION_NETWORKING): self = .networking
        case TwPackageSection_t(TW_PACKAGE_SECTION_PACKAGING): self = .packaging
        case TwPackageSection_t(TW_PACKAGE_SECTION_SYSTEM): self = .system
        case TwPackageSection_t(TW_PACKAGE_SECTION_TERMINAL_SUPPORT): self = .terminalSupport
        case TwPackageSection_t(TW_PACKAGE_SECTION_TEXT_EDITORS): self = .textEditors
        case TwPackageSection_t(TW_PACKAGE_SECTION_THEMES): self = .themes
        case TwPackageSection_t(TW_PACKAGE_SECTION_TWEAKS): self = .tweaks
        case TwPackageSection_t(TW_PACKAGE_SECTION_UTILITIES): self = .utilities

        default: self = .other
        }
    }
}
