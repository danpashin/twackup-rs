//
//  DebsListModel+DZNEmptyDataSet.swift
//  Twackup
//
//  Created by Daniil on 23.12.2022.
//

import DZNEmptyDataSet

extension DebsListModel: DZNEmptyDataSetSource {
    nonisolated func title(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "database-controller-no-packages-title".localized)
    }

    nonisolated func description(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "database-controller-no-packages-subtitle".localized)
    }

    nonisolated func image(forEmptyDataSet scrollView: UIScrollView) -> UIImage? {
        UIImage(
            systemName: "shippingbox",
            withConfiguration: UIImage.SymbolConfiguration(pointSize: 120, weight: .light)
        )
    }

    nonisolated func imageTintColor(forEmptyDataSet scrollView: UIScrollView?) -> UIColor? {
        .tertiaryLabel
    }
}
