//
//  DpkgListModel.swift
//  Twackup
//
//  Created by Daniil on 17.12.2022.
//

import DZNEmptyDataSet

class DpkgListModel: PackageListModel, DZNEmptyDataSetSource {
    let dpkgProvider: DpkgDataProvier

    override var tableView: UITableView? {
        didSet {
            tableView?.emptyDataSetSource = self
        }
    }

    init(mainModel: MainModel, dataProvider: DpkgDataProvier, metadata: ViewControllerMetadata) {
        dpkgProvider = dataProvider

        super.init(mainModel: mainModel, dataProvider: dataProvider, metadata: metadata)
    }

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
