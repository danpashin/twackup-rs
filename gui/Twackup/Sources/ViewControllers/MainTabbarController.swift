//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class MainTabbarController: UITabBarController {
    lazy private var dpkgInstance = Dpkg()

    lazy private var database = Database()

    lazy private var buildedPackagesVC: UIViewController = {
        let provider = PackageVC.DatabaseProvider(database)
        let metadata = PackageVC.BuildedPkgsMetadata()

        let detailVC = PackageVC.DatabaseDetailVC()

        let mainVC = PackageVC.ListWithDetailVC(provider, metadata)
        mainVC.model.detailDelegate = detailVC

        let splitVC = UISplitViewController()
        splitVC.tabBarItem = metadata.tabbarItem
        splitVC.viewControllers = [
            UINavigationController(rootViewController: mainVC),
            UINavigationController(rootViewController: detailVC)
        ]
        return splitVC
    }()

    lazy private var leavesPackagesVC: UIViewController = {
        let provider = PackageVC.DpkgProvier(dpkgInstance, leaves: true)
        let metadata = PackageVC.LeavesPkgsMetadata()
        return makePackagesControler(provider, metadata)
    }()

    lazy private var allPackagesVC: UIViewController = {
        let provider = PackageVC.DpkgProvier(dpkgInstance)
        let metadata = PackageVC.AllPkgsMetadata()
        return makePackagesControler(provider, metadata)
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.tintColor = .systemPink

        setViewControllers([buildedPackagesVC, leavesPackagesVC, allPackagesVC], animated: false)
    }

    private func makePackagesControler(_ dataProvider: PackageVC.DataProvider,
                                       _ metadata: PackageVC.Metadata) -> UIViewController {
        let detailVC = PackageVC.DpkgDetailVC(dpkg: dpkgInstance, database: database)

        let mainVC = PackageVC.ListWithDetailVC(dataProvider, metadata)
        mainVC.model.detailDelegate = detailVC

        let splitVC = UISplitViewController()
        splitVC.tabBarItem = metadata.tabbarItem
        splitVC.viewControllers = [
            UINavigationController(rootViewController: mainVC),
            UINavigationController(rootViewController: detailVC)
        ]

        return splitVC
    }
}
