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
    let preferences = Preferences()

    private var consoleLoggerIsSet = false

    private(set) lazy var mainModel: MainModel = {
        let dpkg = Dpkg(path: jbRootPath("/var/lib/dpkg"), preferences: preferences)
        return MainModel(database: Database(), dpkg: dpkg, preferences: preferences)
    }()

    override init() {
        super.init()

        libSandy_applyProfile("TwackupGUI")
    }

    func application(
        _ application: UIApplication, didFinishLaunchingWithOptions
        launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        SentrySDK.start { options in
            options.dsn = "https://fe79331fe69841ddabaf9b5161d50e00@o4504339145555968.ingest.sentry.io/4504339146670080"
            options.tracesSampleRate = 1.0
            options.enableUIViewControllerTracing = false
            options.enableNetworkBreadcrumbs = false
        }

        return true
    }

    func setupConsoleLogger() async {
        if !consoleLoggerIsSet {
            consoleLoggerIsSet = true
            await FFILogger.shared.addSubscriber(ConsoleLogger())
        }
    }
}
