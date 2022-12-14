//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit
import SwiftUI

class MainTabbarController: UITabBarController {
    lazy private var dpkgInstance: Dpkg = {
        var path = "/var/lib/dpkg"
        if !FileManager.default.fileExists(atPath: path) {
            path = "/var/jb/dpkg"
        }

        return Dpkg(path: path)
    }()

    let database: Database

    init(database: Database) {
        self.database = database

        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    lazy private var buildedPackagesVC: UIViewController = {
        let provider = DatabasePackageProvider(database)
        let metadata = PackageVC.BuildedPkgsMetadata()
        let model = PackageVC.DebsListModel(dataProvider: provider, metadata: metadata)

        let detailVC = PackageVC.DatabaseDetailVC(database: database)

        let mainVC = PackageVC.DebsListVC(model: model, detail: detailVC)

        let splitVC = TwoColumnsVC()
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

        let logMetadata = LogVCMetadata()
        let logController = UINavigationController(rootViewController: LoggerViewController(metadata: logMetadata))
        logController.tabBarItem = logMetadata.tabbarItem

        let settingsController = UIHostingController(rootView: SettingsViewController())
        settingsController.navigationItem.title = "Host"
        settingsController.tabBarItem = PreferencesVCMetadata().tabbarItem

        setViewControllers([
            buildedPackagesVC, leavesPackagesVC, allPackagesVC, logController, settingsController
        ], animated: false)
    }

    private func makePackagesControler(_ dataProvider: PackageDataProvider,
                                       _ metadata: ViewControllerMetadata) -> UIViewController {
        let model = PackageVC.PackageListModel(dataProvider: dataProvider, metadata: metadata)
        let detailVC = PackageVC.DpkgDetailVC(dpkg: dpkgInstance, database: database)

        let mainVC = PackageVC.DpkgListVC(dpkg: dpkgInstance, database: database, model: model, detail: detailVC)

        let splitVC = TwoColumnsVC()
        splitVC.tabBarItem = metadata.tabbarItem
        splitVC.viewControllers = [
            UINavigationController(rootViewController: mainVC),
            UINavigationController(rootViewController: detailVC)
        ]

        return splitVC
    }
}
