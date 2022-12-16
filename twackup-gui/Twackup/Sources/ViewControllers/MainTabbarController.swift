//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import SwiftUI
import UIKit

class MainTabbarController: UITabBarController {
    let database: Database

    private(set) lazy var dpkgInstance: Dpkg = {
        var path = "/var/lib/dpkg"
        if !FileManager.default.fileExists(atPath: path) {
            path = "/var/jb/dpkg"
        }

        return Dpkg(path: path)
    }()

    private(set) lazy var buildedPackagesVC: UIViewController = {
        let provider = DatabasePackageProvider(database)
        let metadata = PackageVC.BuildedPkgsMetadata()
        let model = PackageVC.DebsListModel(dataProvider: provider, metadata: metadata)

        let detailVC = PackageVC.DatabaseDetailVC(database: database)
        let mainVC = PackageVC.DebsListVC(model: model, detail: detailVC)

        return TwoColumnsVC(first: mainVC, second: detailVC, tabBarItem: metadata.tabbarItem)
    }()

    private(set) lazy var leavesPackagesVC: UIViewController = {
        let provider = DpkgDataProvier(dpkgInstance, leaves: true)
        let metadata = PackageVC.LeavesPkgsMetadata()

        let model = PackageVC.PackageListModel(dataProvider: provider, metadata: metadata)
        let detailVC = PackageVC.DpkgDetailVC(dpkg: dpkgInstance, database: database)
        let mainVC = PackageVC.LeavesListVC(dpkg: dpkgInstance, database: database, model: model, detail: detailVC)

        return TwoColumnsVC(first: mainVC, second: detailVC, tabBarItem: metadata.tabbarItem)
    }()

    private(set) lazy var allPackagesVC: UIViewController = {
        let provider = DpkgDataProvier(dpkgInstance)
        let metadata = PackageVC.AllPkgsMetadata()

        let model = PackageVC.PackageListModel(dataProvider: provider, metadata: metadata)
        let detailVC = PackageVC.DpkgDetailVC(dpkg: dpkgInstance, database: database)
        let mainVC = PackageVC.DpkgListVC(dpkg: dpkgInstance, database: database, model: model, detail: detailVC)

        return TwoColumnsVC(first: mainVC, second: detailVC, tabBarItem: metadata.tabbarItem)
    }()

    private(set) lazy var logVC: UIViewController = {
        let logMetadata = LogVCMetadata()
        let logController = UINavigationController(rootViewController: LoggerViewController(metadata: logMetadata))
        logController.tabBarItem = logMetadata.tabbarItem

        return logController
    }()

    private(set) lazy var settingsVC: UIViewController = {
        let prefsMetadata = PreferencesVCMetadata()
        let settingsController = UIHostingController(rootView: SettingsViewController(metadata: prefsMetadata))
        settingsController.tabBarItem = prefsMetadata.tabbarItem

        return settingsController
    }()

    init(database: Database) {
        self.database = database

        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        setViewControllers([
            buildedPackagesVC, leavesPackagesVC, allPackagesVC, logVC, settingsVC
        ], animated: false)
    }
}
