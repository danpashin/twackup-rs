//
//  DpkgListModel.swift
//  Twackup
//
//  Created by Daniil on 17.12.2022.
//

import DZNEmptyDataSet

class DpkgListModel: PackageListModel {
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
}
