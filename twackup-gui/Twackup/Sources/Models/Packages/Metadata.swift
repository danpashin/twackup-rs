//
//  AllPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

protocol ViewControllerMetadata {
    var navTitle: String { get }

    var tabbarItem: UITabBarItem? { get }
}

struct AllPkgsMetadata: ViewControllerMetadata {
    var navTitle: String { "all-pkgs-full-title".localized }

    var tabbarItem: UITabBarItem? {
        UITabBarItem(
            title: "all-pkgs-short-title".localized,
            image: UIImage(systemName: "note.text"),
            tag: 0
        )
    }
}

struct LeavesPkgsMetadata: ViewControllerMetadata {
    var navTitle: String { "leaves-pkgs-full-title".localized }

    var tabbarItem: UITabBarItem? {
        UITabBarItem(
            title: "leaves-pkgs-short-title".localized,
            image: UIImage(systemName: "heart.text.square"),
            tag: 0
        )
    }
}

struct BuildedPkgsMetadata: ViewControllerMetadata {
    var navTitle: String { "debs-full-title".localized }

    var tabbarItem: UITabBarItem? {
        UITabBarItem(
            title: "debs-short-title".localized,
            image: UIImage(systemName: "cube"),
            tag: 0
        )
    }
}

struct LogVCMetadata: ViewControllerMetadata {
    var navTitle: String { "log-full-title".localized }

    var tabbarItem: UITabBarItem? {
        UITabBarItem(
            title: "log-short-title".localized,
            image: UIImage(systemName: "scroll"),
            tag: 0
        )
    }
}

struct PreferencesVCMetadata: ViewControllerMetadata {
    var navTitle: String { "preferences-full-title".localized }

    var tabbarItem: UITabBarItem? {
        UITabBarItem(
            title: "preferences-short-title".localized,
            image: UIImage(systemName: "gearshape"),
            tag: 0
        )
    }
}
