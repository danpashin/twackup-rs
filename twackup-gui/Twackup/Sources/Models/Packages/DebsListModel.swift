//
//  DebsListModel.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

extension PackageVC {
    class DebsListModel: PackageListModel {
        static let NotificationName = Notification.Name("twackup/reloadDEBS")

        private(set) var debsProvider: DatabasePackageProvider
        override var dataProvider: PackageDataProvider {
            get { return debsProvider }
            set { }
        }

        init(dataProvider: DatabasePackageProvider, metadata: ViewControllerMetadata) {
            debsProvider = dataProvider
            super.init(dataProvider: dataProvider, metadata: metadata)
        }

        func tableView(
            _ tableView: UITableView,
            trailingSwipeActionsConfigurationForRowAt indexPath: IndexPath
        ) -> UISwipeActionsConfiguration? {
            let delete = UIContextualAction(style: .destructive, title: nil) { _, _, completionHandler in
                if self.debsProvider.deletePackage(at: indexPath.row) {
                    tableView.deleteRows(at: [indexPath], with: .automatic)
                }
                completionHandler(true)
            }
            delete.image = UIImage(systemName: "trash.fill")
            delete.title = Bundle.appLocalize("remove-btn")

            return UISwipeActionsConfiguration(actions: [delete])
        }
    }
}
