//
//  PackageSection.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import Foundation

enum PackageSection: String {

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


    init(_ section: TwPackageSection_t) {
        switch section {
        case TwPackageSection_t(TW_PACKAGE_SECTION_ARCHIVING):
            self = .archiving
        case TwPackageSection_t(TW_PACKAGE_SECTION_DEVELOPMENT):
            self = .development
        case TwPackageSection_t(TW_PACKAGE_SECTION_NETWORKING):
            self = .networking
        case TwPackageSection_t(TW_PACKAGE_SECTION_PACKAGING):
            self = .packaging
        case TwPackageSection_t(TW_PACKAGE_SECTION_SYSTEM):
            self = .system
        case TwPackageSection_t(TW_PACKAGE_SECTION_TERMINAL_SUPPORT):
            self = .terminalSupport
        case TwPackageSection_t(TW_PACKAGE_SECTION_TEXT_EDITORS):
            self = .textEditors
        case TwPackageSection_t(TW_PACKAGE_SECTION_THEMES):
            self = .themes
        case TwPackageSection_t(TW_PACKAGE_SECTION_TWEAKS):
            self = .tweaks
        case TwPackageSection_t(TW_PACKAGE_SECTION_UTILITIES):
            self = .utilities
        default:
            self = .other
        }
    }

    func systemImageName() -> String {
        switch self {
        case .archiving:
            return "doc.zipper"
        case .development:
            return "cpu"
        case .networking:
            return "network"
        case .packaging:
            return "archivebox.circle.fill"
        case .system:
            return "command"
        case .terminalSupport:
            return "terminal"
        case .textEditors:
            return "doc.text"
        case .themes:
            return "lasso.sparkles"
        case .tweaks:
            return "rectangle.expand.vertical"
        case .utilities:
            return "keyboard"
        case .other:
            return "person.fill.turn.down"
        }
    }
}
