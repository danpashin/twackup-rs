//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class MainTabbarController: UITabBarController {

    var dpkgInstance = Dpkg()

    lazy var leavesPackagesVC: UIViewController = {
        return makePackagesControler(LeavesPackagesVCModel(dpkgInstance))
    }()

    lazy var allPackagesVC: UIViewController = {
        return makePackagesControler(AllPackagesVCModel(dpkgInstance))
    }()

    override func viewDidLoad() {
        super.viewDidLoad()

        view.tintColor = .systemPink

        setViewControllers([leavesPackagesVC, allPackagesVC], animated: false)
    }

    func makePackagesControler(_ model: PackagesVCModel) -> UIViewController {
        var model = model
        let detailVC = PackageDetailViewController()
        model.detailDelegate = detailVC

        let mainVC = PackagesViewController(model)

        let splitVC = UISplitViewController()
        splitVC.tabBarItem = mainVC.model.tabbarItem
        splitVC.viewControllers = [
            UINavigationController(rootViewController: mainVC),
            UINavigationController(rootViewController: detailVC),
        ]

        return splitVC
    }
}
