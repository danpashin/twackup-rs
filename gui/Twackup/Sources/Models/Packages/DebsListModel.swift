//
//  DebsListModel.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

extension PackageVC {
    class DebsListModel: PackageListModel {
        static let NotificationName = NSNotification.Name("twackup/reloadDEBS")

        private(set) var debsProvider: DatabasePackageProvider
        override var dataProvider: PackageDataProvider {
            get { return debsProvider }
            set { }
        }

        init(dataProvider: DatabasePackageProvider, metadata: Metadata) {
            debsProvider = dataProvider
            super.init(dataProvider: dataProvider, metadata: metadata)
        }

        func tableView(_ tableView: UITableView,
                       trailingSwipeActionsConfigurationForRowAt indexPath: IndexPath) -> UISwipeActionsConfiguration? {
            let delete = UIContextualAction(style: .destructive, title: "Delete") { _, _, completionHandler in
                self.debsProvider.deletePackage(at: indexPath.row)
                tableView.deleteRows(at: [indexPath], with: .automatic)
                completionHandler(true)
            }

            return UISwipeActionsConfiguration(actions: [delete])
        }
    }
}
