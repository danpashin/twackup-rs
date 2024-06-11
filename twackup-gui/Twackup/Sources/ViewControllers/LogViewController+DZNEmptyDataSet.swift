//
//  LogViewController+DZNEmptyDataSet.swift
//  Twackup
//
//  Created by Daniil on 24.12.2022.
//

import DZNEmptyDataSet

extension LogViewController: DZNEmptyDataSetSource {
    nonisolated func title(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "log-controller-empty-title".localized)
    }

    nonisolated func description(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "log-controller-empty-subtitle".localized)
    }

    nonisolated func image(forEmptyDataSet scrollView: UIScrollView) -> UIImage? {
        UIImage(
            systemName: "text.alignleft",
            withConfiguration: UIImage.SymbolConfiguration(pointSize: 120, weight: .light)
        )
    }

    nonisolated func imageTintColor(forEmptyDataSet scrollView: UIScrollView?) -> UIColor? {
        .tertiaryLabel
    }
}

#if swift(>=6.0)
extension LogViewController: @preconcurrency DZNEmptyDataSetDelegate {
    func emptyDataSetShouldDisplay(_ scrollView: UIScrollView?) -> Bool {
        currentText.length == 0
    }
}
#else
extension LogViewController: DZNEmptyDataSetDelegate {
    func emptyDataSetShouldDisplay(_ scrollView: UIScrollView?) -> Bool {
        currentText.length == 0
    }
}
#endif
