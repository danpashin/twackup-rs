//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class MainTabbarController: UITabBarController {
    private typealias Provider = ViewControllers.Package.DataProvider
    private typealias Metadata = ViewControllers.Package.Metadata

    let dpkgInstance = Dpkg()

    let database = Database()

    lazy private(set) var buildedPackagesVC: UIViewController = {
        return makePackagesControler(Provider.BuildedPkgsVC(database), Metadata.BuildedPkgsVC())
    }()

    lazy private(set) var leavesPackagesVC: UIViewController = {
        return makePackagesControler(Provider.LeavesPkgsVC(dpkgInstance), Metadata.LeavesPkgsVC())
    }()

    lazy private(set) var allPackagesVC: UIViewController = {
        return makePackagesControler(Provider.AllPkgsVC(dpkgInstance), Metadata.AllPkgsVC())
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.tintColor = .systemPink

        setViewControllers([buildedPackagesVC, leavesPackagesVC, allPackagesVC], animated: false)
    }

    private func makePackagesControler(_ dataProvider: Provider.BasicProvider,
                                       _ metadata: PackagesControllerMetadata) -> UIViewController {
        let detailVC = ViewControllers.Package.Detail()

        let mainVC = ViewControllers.Package.Main(dataProvider, metadata)
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
