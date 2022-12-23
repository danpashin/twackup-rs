//
//  NavigationController.swift
//  Twackup
//
//  Created by Daniil on 17.12.2022.
//

class NavigationController: UINavigationController {
    override var tabBarItem: UITabBarItem? {
        get { viewControllers.first?.tabBarItem }
        set { }
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        navigationBar.prefersLargeTitles = true
    }
}
