//
//  LeavesPackagesModel.swift
//  Twackup
//
//  Created by Daniil on 17.12.2022.
//

class LeavesPackagesModel: DpkgListModel {
    init(mainModel: MainModel) {
        let provider = DpkgDataProvier(mainModel.dpkg, leaves: true)
        let metadata = LeavesPkgsMetadata()

        super.init(mainModel: mainModel, dataProvider: provider, metadata: metadata)
    }
}
