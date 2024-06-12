//
//  SceneDelegate.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class SceneDelegate: UIResponder, UIWindowSceneDelegate {
    var window: UIWindow?

    func scene(
        _ scene: UIScene,
        willConnectTo session: UISceneSession,
        options connectionOptions: UIScene.ConnectionOptions
    ) {
        guard let windowScene = scene as? UIWindowScene else { return }
        guard let delegate = UIApplication.shared.delegate as? AppDelegate else { return }

        let window = UIWindow(windowScene: windowScene)
        window.makeKeyAndVisible()
        window.backgroundColor = .systemBackground
        self.window = window

        Task {
            // Since actors are executing on `RunLoop.main`
            // we can't call logger initialization in `AppDelegate.init` (runloop not run yet)
            // Another option is to make FFILogger to be not an actor but this will bring libdispatch back
            //
            // So, there's a hack. Init mainModel (since it is a lazy var) just after logger
            // Screen will blink (that's why `window.backgroundColor` is set) but that's ok for us
            await delegate.setupConsoleLogger()

            window.rootViewController = MainTabbarController(mainModel: delegate.mainModel)
        }
    }
}
