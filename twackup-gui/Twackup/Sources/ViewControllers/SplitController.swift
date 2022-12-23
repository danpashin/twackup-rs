//
//  SplitController.swift
//  Twackup
//
//  Created by Daniil on 07.12.2022.
//

import UIKit

class SplitController: UISplitViewController, UISplitViewControllerDelegate {
    let primaryVC: ScrollableViewController

    let primaryNavigationVC: NavigationController

    let secondaryVC: UIViewController

    init(primaryVC: ScrollableViewController, secondaryVC: UIViewController) {
        self.primaryVC = primaryVC
        self.secondaryVC = secondaryVC
        primaryNavigationVC = NavigationController(rootViewController: primaryVC)

        super.init(nibName: nil, bundle: nil)

        tabBarItem = primaryVC.tabBarItem
        viewControllers = [
            primaryNavigationVC,
            NavigationController(rootViewController: secondaryVC)
        ]

        delegate = self
        preferredDisplayMode = .oneBesideSecondary
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    /// Resets current navigation stack.
    /// If there's controller in navigation - it pops it to root.
    /// If current nav controller is root - it scrolls it to the top.
    ///
    /// - Parameter animated: Pass true if all actions should be animated
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
