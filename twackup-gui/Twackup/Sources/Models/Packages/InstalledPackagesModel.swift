//
//  InstalledPackagesModel.swift
//  Twackup
//
//  Created by Daniil on 17.12.2022.
//

extension PackageVC {
    class InstalledPackagesModel: DpkgListModel {
        init(mainModel: MainModel) {
            let provider = DpkgDataProvier(mainModel.dpkg, leaves: false)
            let metadata = AllPkgsMetadata()

            super.init(mainModel: mainModel, dataProvider: provider, metadata: metadata)
        }
    }
}
