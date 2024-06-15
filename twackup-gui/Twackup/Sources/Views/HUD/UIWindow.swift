//
//  UIWindow.swift
//  iAppsDRM
//
//  Created by Daniil on 08.01.2024.
//  Copyright © 2024 Даниил. All rights reserved.
//

import UIKit

extension UIWindow {
    static var focusedScene: UIWindowScene? {
        UIApplication.shared.connectedScenes.first { scene in
            let state = scene.activationState
            return (state == .foregroundActive || state == .foregroundInactive) && scene is UIWindowScene
        } as? UIWindowScene
    }
}
