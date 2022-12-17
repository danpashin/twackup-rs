//
//  DpkgListModel.swift
//  Twackup
//
//  Created by Daniil on 17.12.2022.
//

class DpkgListModel: PackageListModel {
    let dpkgProvider: DpkgDataProvier

    init(mainModel: MainModel, dataProvider: DpkgDataProvier, metadata: ViewControllerMetadata) {
        dpkgProvider = dataProvider

        super.init(mainModel: mainModel, dataProvider: dataProvider, metadata: metadata)
    }
}
