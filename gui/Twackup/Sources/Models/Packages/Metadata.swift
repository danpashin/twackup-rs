//
//  AllPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

extension PackageVC {
    class Metadata {
        var navTitle: String? { .none }

        var tabbarItem: UITabBarItem? { .none }
    }

    class AllPkgsMetadata: Metadata {
        override var navTitle: String { Bundle.appLocalize("all-pkgs-full-title") }

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: Bundle.appLocalize("all-pkgs-short-title"),
                         image: UIImage(systemName: "note.text"), tag: 0)
        }
    }

    class LeavesPkgsMetadata: Metadata {
        override var navTitle: String { Bundle.appLocalize("leaves-pkgs-full-title") }

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: Bundle.appLocalize("leaves-pkgs-short-title"),
                         image: UIImage(systemName: "heart.text.square"), tag: 0)
        }
    }

    class BuildedPkgsMetadata: Metadata {
        override var navTitle: String { Bundle.appLocalize("debs-full-title") }

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: Bundle.appLocalize("debs-short-title"),
                         image: UIImage(systemName: "cube"), tag: 0)
        }
    }
}
