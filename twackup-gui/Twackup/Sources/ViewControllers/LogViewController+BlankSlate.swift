//
//  LogViewController+BlankSlate.swift
//  Twackup
//
//  Created by Daniil on 19.06.2024.
//

import BlankSlate

extension LogViewController: @preconcurrency BlankSlate.DataSource, @preconcurrency BlankSlate.Delegate {
    nonisolated func title(forBlankSlate view: UIView) -> NSAttributedString? {
        NSAttributedString(string: "log-controller-empty-title".localized)
    }

    nonisolated func detail(forBlankSlate view: UIView) -> NSAttributedString? {
        NSAttributedString(string: "log-controller-empty-subtitle".localized)
    }

    func image(forBlankSlate view: UIView) -> UIImage? {
        UIImage(
            systemName: "text.alignleft",
            withConfiguration: UIImage.SymbolConfiguration(pointSize: 120, weight: .light)
        )
    }

    func imageTintColor(forBlankSlate view: UIView) -> UIColor? {
        .tertiaryLabel
    }
    
    func blankSlateShouldDisplay(_ view: UIView) -> Bool {
        MainActor.assumeIsolated { currentText.length == 0 }
    }
}
