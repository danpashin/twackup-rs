//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import SwiftUI
import UIKit

class MainTabbarController: UITabBarController {
    let mainModel: MainModel

    private(set) lazy var buildedPackagesVC: UIViewController = {
        let model = PackageVC.DebsListModel(mainModel: mainModel)
        let detailVC = PackageVC.DatabaseDetailVC(mainModel: mainModel)
        let mainVC = PackageVC.DebsListVC(model: model, detail: detailVC)

        return TwoColumnsVC(first: mainVC, second: detailVC)
    }()

    private(set) lazy var leavesPackagesVC: UIViewController = {
        let model = PackageVC.LeavesPackagesModel(mainModel: mainModel)
        let detailVC = PackageVC.DpkgDetailVC(mainModel: mainModel)
        let mainVC = PackageVC.LeavesListVC(model: model, detail: detailVC)

        return TwoColumnsVC(first: mainVC, second: detailVC)
    }()

    private(set) lazy var allPackagesVC: UIViewController = {
        let model = PackageVC.InstalledPackagesModel(mainModel: mainModel)
        let detailVC = PackageVC.DpkgDetailVC(mainModel: mainModel)
        let mainVC = PackageVC.DpkgListVC(model: model, detail: detailVC)

        return TwoColumnsVC(first: mainVC, second: detailVC)
    }()

    private(set) lazy var logVC: UIViewController = {
        let logMetadata = LogVCMetadata()
        let logVC = LoggerViewController(mainModel: mainModel, metadata: logMetadata)
        let logController = SimpleNavController(rootViewController: logVC)
        logController.tabBarItem = logMetadata.tabbarItem

        return logController
    }()

    private(set) lazy var settingsVC: UIViewController = {
        let prefsMetadata = PreferencesVCMetadata()
        let rootView = SettingsViewController(mainModel: mainModel, metadata: prefsMetadata)
        let settingsController = UIHostingController(rootView: rootView)
        settingsController.tabBarItem = prefsMetadata.tabbarItem

        return settingsController
    }()

    init(mainModel: MainModel) {
        self.mainModel = mainModel

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
