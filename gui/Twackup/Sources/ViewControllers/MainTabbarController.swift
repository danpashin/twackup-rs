//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class MainTabbarController: UITabBarController {

    let dpkgInstance = Dpkg()

    lazy private(set) var leavesPackagesVC: UIViewController = {
        return makePackagesControler(LeavesPackagesDataProvider(dpkgInstance))
    }()

    lazy private(set) var allPackagesVC: UIViewController = {
        return makePackagesControler(AllPackagesDataProvider(dpkgInstance))
    }()

    override func viewDidLoad() {
        super.viewDidLoad()

        view.tintColor = .systemPink

        setViewControllers([leavesPackagesVC, allPackagesVC], animated: false)
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
