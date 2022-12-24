//
//  LogViewController+DZNEmptyDataSet.swift
//  Twackup
//
//  Created by Daniil on 24.12.2022.
//

import DZNEmptyDataSet

extension LogViewController: DZNEmptyDataSetSource, DZNEmptyDataSetDelegate {
    func title(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "log-controller-empty-title".localized)
    }

    func description(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "log-controller-empty-subtitle".localized)
    }

    func image(forEmptyDataSet scrollView: UIScrollView) -> UIImage? {
        UIImage(
            systemName: "text.alignleft",
            withConfiguration: UIImage.SymbolConfiguration(pointSize: 120, weight: .light)
        )
    }

    func imageTintColor(forEmptyDataSet scrollView: UIScrollView?) -> UIColor? {
        .tertiaryLabel
    }

    func emptyDataSetShouldDisplay(_ scrollView: UIScrollView?) -> Bool {
        currentText.length == 0
    }
}
