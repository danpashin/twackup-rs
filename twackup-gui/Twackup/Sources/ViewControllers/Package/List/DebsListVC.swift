//
//  DebsListVC.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

extension PackageVC {
    class DebsListVC: PackageListVC {
        private var debsModel: DebsListModel
        override var model: PackageListModel {
            get { return debsModel }
            set { }
        }

        init(model: DebsListModel, detail: PackageVC.DetailVC) {
            debsModel = model
            super.init(model: model, detail: detail)
        }

        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        override func viewDidLoad() {
            super.viewDidLoad()

            let center = NotificationCenter.default
            center.addObserver(forName: DebsListModel.NotificationName, object: nil, queue: .current) { _ in
                self.reload()
            }
        }

        func reload() {
            DispatchQueue.global().async {
                self.debsModel.debsProvider.reload()
                DispatchQueue.main.async {
                    self.reloadTableView()
                }
            }
        }
    }
}
