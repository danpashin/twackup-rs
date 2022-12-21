//
//  TwoColumnsVC.swift
//  Twackup
//
//  Created by Daniil on 07.12.2022.
//

import UIKit

class TwoColumnsVC: UISplitViewController, UISplitViewControllerDelegate {
    let primaryVC: ScrollableViewController

    let primaryNavigationVC: SimpleNavController

    let secondaryVC: UIViewController

    init(primaryVC: ScrollableViewController, secondaryVC: UIViewController) {
        self.primaryVC = primaryVC
        self.secondaryVC = secondaryVC
        primaryNavigationVC = SimpleNavController(rootViewController: primaryVC)

        super.init(nibName: nil, bundle: nil)

        self.tabBarItem = primaryVC.tabBarItem
        viewControllers = [
            primaryNavigationVC,
            SimpleNavController(rootViewController: secondaryVC)
        ]

        delegate = self
        preferredDisplayMode = .oneBesideSecondary
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    func resetNavigation(animated: Bool) {
        if primaryNavigationVC.topViewController == primaryVC {
            primaryVC.scrollToTop(animated: animated)
        } else {
            primaryNavigationVC.popToRootViewController(animated: animated)
        }
    }

    func splitViewController(
        _ splitViewController: UISplitViewController,
        collapseSecondary secondaryViewController: UIViewController,
        onto primaryViewController: UIViewController
    ) -> Bool {
        splitViewController.traitCollection.horizontalSizeClass == .compact
    }
}
