//
//  TwoColumnsVC.swift
//  Twackup
//
//  Created by Daniil on 07.12.2022.
//

import UIKit

class TwoColumnsVC: UISplitViewController, UISplitViewControllerDelegate {
    init(first: UIViewController, second: UIViewController) {
        super.init(nibName: nil, bundle: nil)

        self.tabBarItem = first.tabBarItem
        viewControllers = [
            SimpleNavController(rootViewController: first),
            SimpleNavController(rootViewController: second)
        ]
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        delegate = self
        preferredDisplayMode = .oneBesideSecondary
    }

    func splitViewController(
        _ splitViewController: UISplitViewController,
        collapseSecondary secondaryViewController: UIViewController,
        onto primaryViewController: UIViewController
    ) -> Bool {
        return true
    }
}
