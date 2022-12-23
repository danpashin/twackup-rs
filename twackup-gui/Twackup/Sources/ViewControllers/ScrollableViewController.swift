//
//  ScrollableViewController.swift
//  Twackup
//
//  Created by Daniil on 21.12.2022.
//

protocol ScrollableViewController: UIViewController {
    /// Scrolls controller to the top
    /// 
    /// - Parameter animated: Pass true if all actions should be animated
    func scrollToTop(animated: Bool)
}
