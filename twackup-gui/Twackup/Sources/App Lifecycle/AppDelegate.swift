//
//  AppDelegate.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    let logger = FFILogger.shared

    private(set) lazy var database = Database()
}