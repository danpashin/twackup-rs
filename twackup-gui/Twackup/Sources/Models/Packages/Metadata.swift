//
//  AllPackagesVCModel.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class ViewControllerMetadata {
    var navTitle: String? { .none }

    var tabbarItem: UITabBarItem? { .none }
}

extension PackageVC {
    class AllPkgsMetadata: ViewControllerMetadata {
        override var navTitle: String { Bundle.appLocalize("all-pkgs-full-title") }

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: Bundle.appLocalize("all-pkgs-short-title"),
                         image: UIImage(systemName: "note.text"), tag: 0)
        }
    }

    class LeavesPkgsMetadata: ViewControllerMetadata {
        override var navTitle: String { Bundle.appLocalize("leaves-pkgs-full-title") }

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: Bundle.appLocalize("leaves-pkgs-short-title"),
                         image: UIImage(systemName: "heart.text.square"), tag: 0)
        }
    }

    class BuildedPkgsMetadata: ViewControllerMetadata {
        override var navTitle: String { Bundle.appLocalize("debs-full-title") }

        override var tabbarItem: UITabBarItem {
            UITabBarItem(title: Bundle.appLocalize("debs-short-title"),
                         image: UIImage(systemName: "cube"), tag: 0)
        }
    }
}

class LogVCMetadata: ViewControllerMetadata {
    override var navTitle: String { Bundle.appLocalize("log-full-title") }

    override var tabbarItem: UITabBarItem {
        UITabBarItem(title: Bundle.appLocalize("log-short-title"),
                     image: UIImage(systemName: "scroll"), tag: 0)
    }
}

class PreferencesVCMetadata: ViewControllerMetadata {
    override var navTitle: String { Bundle.appLocalize("preferences-full-title") }

    override var tabbarItem: UITabBarItem {
        UITabBarItem(title: Bundle.appLocalize("preferences-short-title"),
                     image: UIImage(systemName: "gearshape"), tag: 0)
    }
}
