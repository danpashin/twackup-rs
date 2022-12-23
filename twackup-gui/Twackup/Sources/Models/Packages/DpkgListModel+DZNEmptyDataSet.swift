//
//  DZNEmptyDataSetSource+DZNEmptyDataSetSource.swift
//  Twackup
//
//  Created by Daniil on 23.12.2022.
//

import DZNEmptyDataSet

extension DpkgListModel: DZNEmptyDataSetSource {
    func title(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "dpkg-controller-no-packages-title".localized)
    }

    func description(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "dpkg-controller-no-packages-subtitle".localized)
    }

    func image(forEmptyDataSet scrollView: UIScrollView) -> UIImage? {
        UIImage(
            systemName: "lock.rectangle",
            withConfiguration: UIImage.SymbolConfiguration(pointSize: 120, weight: .light)
        )
    }

    func imageTintColor(forEmptyDataSet scrollView: UIScrollView?) -> UIColor? {
        .tertiaryLabel
    }
}
