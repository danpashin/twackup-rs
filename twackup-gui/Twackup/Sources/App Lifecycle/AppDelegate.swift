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
    let logger = FFILogger.shared

    private(set) lazy var database = Database()

    func application(
        _ application: UIApplication, didFinishLaunchingWithOptions
        launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        SentrySDK.start { options in
            options.dsn = "https://fe79331fe69841ddabaf9b5161d50e00@o4504339145555968.ingest.sentry.io/4504339146670080"
            options.tracesSampleRate = 1.0
            options.enableFileIOTracking = true
            options.enableCoreDataTracking = true
        }

        return true
    }
}
