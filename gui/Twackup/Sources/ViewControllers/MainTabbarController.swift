//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class MainTabbarController: UITabBarController {

    let dpkgInstance = Dpkg()

    let database = Database()

    lazy private(set) var buildedPackagesVC: UIViewController = {
        return makePackagesControler(BuildedPackagesDataProvider(database))
    }()

    lazy private(set) var leavesPackagesVC: UIViewController = {
        return makePackagesControler(LeavesPackagesDataProvider(dpkgInstance))
    }()

    lazy private(set) var allPackagesVC: UIViewController = {
        return makePackagesControler(AllPackagesDataProvider(dpkgInstance))
    }()

    override func viewDidLoad() {
        super.viewDidLoad()

        let package = database.createBuildedPackage();
        package.name = "Test"
        package.id = "ru.danpashin.test"
        package.architecture = "iphoneos-arm"
        package.buildDate = Date()
        package.debRelativePath = "/"
        package.version = "1.0"
        database.addBuildedPackage(package)

        view.tintColor = .systemPink

        setViewControllers([buildedPackagesVC, leavesPackagesVC, allPackagesVC], animated: false)
    }

    func makePackagesControler(_ dataProvider: PackagesDataProvider) -> UIViewController {
        let detailVC = PackageDetailViewController()

        let mainVC = PackagesViewController(dataProvider)
        mainVC.model.detailDelegate = detailVC

        let splitVC = UISplitViewController()
        splitVC.tabBarItem = dataProvider.tabbarItem
        splitVC.viewControllers = [
            UINavigationController(rootViewController: mainVC),
            UINavigationController(rootViewController: detailVC),
        ]

        return splitVC
    }
}
