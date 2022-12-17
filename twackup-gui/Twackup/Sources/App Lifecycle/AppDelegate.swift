//
//  AppDelegate.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import Sentry
import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    private(set) lazy var mainModel: MainModel = {
        var path = "/var/lib/dpkg"
        if !FileManager.default.fileExists(atPath: path) {
            path = "/var/jb/dpkg"
        }

        return MainModel(database: Database(), dpkg: Dpkg(path: path))
    }()

    func application(
        _ application: UIApplication, didFinishLaunchingWithOptions
        launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        SentrySDK.start { options in
            options.dsn = "https://fe79331fe69841ddabaf9b5161d50e00@o4504339145555968.ingest.sentry.io/4504339146670080"
            options.tracesSampleRate = 0.7
            options.enableCoreDataTracking = true
            options.enableUIViewControllerTracking = false
            options.enableNetworkBreadcrumbs = false
        }

        return true
    }
}
