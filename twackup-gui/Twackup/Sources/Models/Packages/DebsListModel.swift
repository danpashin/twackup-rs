//
//  DebsListModel.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

import DZNEmptyDataSet

protocol DebsListModelDelegate: PackageListDelegate {
    func debsModel(
        _ debsModel: DebsListModel,
        didRecieveDebRemoveChallenge package: DebPackage,
        completion: @escaping  (_ allow: Bool) -> Void
    )
}

class DebsListModel: PackageListModel {
    static let NotificationName = Notification.Name("twackup/reloadDEBS")

    private(set) var debsProvider: DatabasePackageProvider
    override var dataProvider: PackageDataProvider {
        get { return debsProvider }
        set { }
    }

    weak var debsModelDelegate: DebsListModelDelegate?

    override var tableView: UITableView? {
        didSet {
            guard let tableView else { return }
            let cellID = String(describing: DebTableViewCell.self)
            tableView.register(DebTableViewCell.self, forCellReuseIdentifier: cellID)

            tableView.emptyDataSetSource = self
        }
    }

    // MARK: - Public functions

    init(mainModel: MainModel) {
        debsProvider = mainModel.databasePackageProvider
        let metadata = BuildedPkgsMetadata()

        super.init(mainModel: mainModel, dataProvider: debsProvider, metadata: metadata)
    }

    func removePackage(at indexPath: IndexPath) {
        if debsProvider.deletePackage(at: indexPath.row) {
            tableView?.deleteRows(at: [indexPath], with: .automatic)
            delegate?.endReloadingData()
        }
    }

    // MARK: - UITableViewDelegate

    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cellID = String(describing: DebTableViewCell.self)
        let cell = tableView.dequeueReusableCell(withIdentifier: cellID, for: indexPath)
        if let cell = cell as? DebTableViewCell {
            cell.package = dataProvider.packages[indexPath.row]
        }

        return cell
    }

    func tableView(
        _ tableView: UITableView,
        trailingSwipeActionsConfigurationForRowAt indexPath: IndexPath
    ) -> UISwipeActionsConfiguration? {
        let delete = UIContextualAction(style: .destructive, title: nil) { _, _, completionHandler in
            self.removePackage(at: indexPath)
            completionHandler(true)
        }
        delete.image = UIImage(systemName: "trash.fill")
        delete.title = "remove-btn".localized

        return UISwipeActionsConfiguration(actions: [delete])
    }

    func tableView(
        _ tableView: UITableView,
        contextMenuConfigurationForRowAt indexPath: IndexPath,
        point: CGPoint
    ) -> UIContextMenuConfiguration? {
        // swiftlint:disable trailing_closure
        let configurator = UIContextMenuConfiguration(actionProvider: { _ in
            guard let package = self.dataProvider.packages[indexPath.row].asDEB else { return nil }
            let copyID = UIAction(title: "copy-id".localized, image: UIImage(systemName: "paperclip")) { _ in
                UIPasteboard.general.string = package.id
            }

            let remove = UIAction(
                title: "remove-btn".localized,
                image: UIImage(systemName: "trash"),
                attributes: [.destructive],
                handler: { [self] _ in
                    debsModelDelegate?.debsModel(self, didRecieveDebRemoveChallenge: package) { [self] allow in
                        if allow {
                            removePackage(at: indexPath)
                        }
                    }
                }
            )

            var children = [copyID, remove]

            // valid url so it's safe to unwrap
            let filzaURL = URL(string: "filza://\(package.fileURL.path)")!
            if UIApplication.shared.canOpenURL(filzaURL) {
                children.insert(
                    UIAction(
                        title: "view-in-filza".localized,
                        image: UIImage(systemName: "arrowshape.turn.up.right"),
                        handler: { _ in
                            UIApplication.shared.open(filzaURL)
                        }
                    ),
                    at: 0
                )
            }

            return UIMenu(children: children)
        })
        // swiftlint:enable trailing_closure

        return configurator
    }
}
