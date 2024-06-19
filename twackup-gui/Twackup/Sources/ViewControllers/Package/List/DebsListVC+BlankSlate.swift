//
//  DebsListVC+BlankSlate.swift
//  Twackup
//
//  Created by Daniil on 19.06.2024.
//

import BlankSlate

extension DebsListVC: @preconcurrency BlankSlate.DataSource {
    nonisolated func title(forBlankSlate view: UIView) -> NSAttributedString? {
        NSAttributedString(string: "database-controller-no-packages-title".localized)
    }

    nonisolated func detail(forBlankSlate view: UIView) -> NSAttributedString? {
        NSAttributedString(string: "database-controller-no-packages-subtitle".localized)
    }

    func image(forBlankSlate view: UIView) -> UIImage? {
        UIImage(
            systemName: "shippingbox",
            withConfiguration: UIImage.SymbolConfiguration(pointSize: 120, weight: .light)
        )
    }

    func imageTintColor(forBlankSlate view: UIView) -> UIColor? {
        .tertiaryLabel
    }
}
