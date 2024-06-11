//
//  ScrollableViewController.swift
//  Twackup
//
//  Created by Daniil on 21.12.2022.
//

@MainActor
protocol ScrollableViewController: UIViewController {
    /// Scrolls controller to it's initial position - top or bottom
    ///
    /// - Parameter animated: Pass true if all actions should be animated
    func scrollToInitialPosition(animated: Bool)
}
