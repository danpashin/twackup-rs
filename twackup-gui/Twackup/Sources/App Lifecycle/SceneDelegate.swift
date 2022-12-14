//
//  SceneDelegate.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class SceneDelegate: UIResponder, UIWindowSceneDelegate {
    var window: UIWindow?

    func scene(_ scene: UIScene, willConnectTo session: UISceneSession,
               options connectionOptions: UIScene.ConnectionOptions) {
        guard let windowScene = scene as? UIWindowScene else { return }
        guard let delegate = UIApplication.shared.delegate as? AppDelegate else { return }

        let window = UIWindow(frame: windowScene.coordinateSpace.bounds)
        window.windowScene = windowScene
        window.rootViewController = MainTabbarController(database: delegate.database)
        window.makeKeyAndVisible()

        self.window = window
    }
}
