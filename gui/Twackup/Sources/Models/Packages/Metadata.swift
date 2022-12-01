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
        override var navTitle: String { "All packages"}

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: "All", image: UIImage(systemName: "list.bullet.rectangle"), tag: 0)
        }
    }

    class LeavesPkgsMetadata: Metadata {
        override var navTitle: String { "Leaves" }

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: "Leaves", image: UIImage(systemName: "cube"), tag: 0)
        }
    }

    class BuildedPkgsMetadata: Metadata {
        override var navTitle: String { "DEBs" }

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: "DEBs", image: UIImage(systemName: "list.bullet.rectangle"), tag: 0)
        }
    }
}
