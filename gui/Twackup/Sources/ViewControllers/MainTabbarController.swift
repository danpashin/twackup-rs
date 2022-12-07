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
        let provider = DatabasePackageProvider(database)
        let metadata = PackageVC.BuildedPkgsMetadata()
        let model = PackageVC.DebsListModel(dataProvider: provider, metadata: metadata)

        let detailVC = PackageVC.DatabaseDetailVC(database: database)

        let mainVC = PackageVC.DebsListVC(model: model, detail: detailVC)

        let splitVC = UISplitViewController()
        splitVC.tabBarItem = metadata.tabbarItem
        splitVC.viewControllers = [
            UINavigationController(rootViewController: mainVC),
            UINavigationController(rootViewController: detailVC)
        ]
        return splitVC
    }()

    lazy private var leavesPackagesVC: UIViewController = {
        let provider = DpkgDataProvier(dpkgInstance, leaves: true)
        let metadata = PackageVC.LeavesPkgsMetadata()
        return makePackagesControler(provider, metadata)
    }()

    lazy private var allPackagesVC: UIViewController = {
        let provider = DpkgDataProvier(dpkgInstance)
        let metadata = PackageVC.AllPkgsMetadata()
        return makePackagesControler(provider, metadata)
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.tintColor = .systemPink

        setViewControllers([buildedPackagesVC, leavesPackagesVC, allPackagesVC], animated: false)
    }

    private func makePackagesControler(_ dataProvider: PackageDataProvider,
                                       _ metadata: PackageVC.Metadata) -> UIViewController {
        let model = PackageVC.PackageListModel(dataProvider: dataProvider, metadata: metadata)
        let detailVC = PackageVC.DpkgDetailVC(dpkg: dpkgInstance, database: database)

        let mainVC = PackageVC.PackageListVC(model: model, detail: detailVC)

        let splitVC = UISplitViewController()
        splitVC.tabBarItem = metadata.tabbarItem
        splitVC.viewControllers = [
            UINavigationController(rootViewController: mainVC),
            UINavigationController(rootViewController: detailVC)
        ]

        return splitVC
    }
}
