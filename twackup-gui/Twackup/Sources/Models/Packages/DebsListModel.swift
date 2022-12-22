//
//  DebsListModel.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

import DZNEmptyDataSet

class DebsListModel: PackageListModel, DZNEmptyDataSetSource {
    static let NotificationName = Notification.Name("twackup/reloadDEBS")

    private(set) var debsProvider: DatabasePackageProvider
    override var dataProvider: PackageDataProvider {
        get { return debsProvider }
        set { }
    }

    override var tableView: UITableView? {
        didSet {
            guard let tableView else { return }
            let cellID = String(describing: DebTableViewCell.self)
            tableView.register(DebTableViewCell.self, forCellReuseIdentifier: cellID)

            tableView.emptyDataSetSource = self
        }
    }

    init(mainModel: MainModel) {
        debsProvider = mainModel.databasePackageProvider
        let metadata = BuildedPkgsMetadata()

        super.init(mainModel: mainModel, dataProvider: debsProvider, metadata: metadata)
    }

    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cellID = String(describing: DebTableViewCell.self)
        let cell = tableView.dequeueReusableCell(withIdentifier: cellID, for: indexPath)
        if let cell = cell as? DebTableViewCell {
            cell.package = dataProvider.packages[indexPath.row]
        }

        return cell
    }

    func removePackage(at indexPath: IndexPath) {
        if debsProvider.deletePackage(at: indexPath.row) {
            tableView?.deleteRows(at: [indexPath], with: .automatic)
            delegate?.endReloadingData()
        }
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

    func title(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "database-controller-no-packages-title".localized)
    }

    func description(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "database-controller-no-packages-subtitle".localized)
    }

    func image(forEmptyDataSet scrollView: UIScrollView) -> UIImage? {
        UIImage(
            systemName: "shippingbox",
            withConfiguration: UIImage.SymbolConfiguration(pointSize: 120, weight: .light)
        )
    }

    func imageTintColor(forEmptyDataSet scrollView: UIScrollView?) -> UIColor? {
        .tertiaryLabel
    }

    func tableView(
        _ tableView: UITableView,
        contextMenuConfigurationForRowAt indexPath: IndexPath,
        point: CGPoint
    ) -> UIContextMenuConfiguration? {
        // swiftlint:disable trailing_closure
        let configurator = UIContextMenuConfiguration(actionProvider: { _ in
            let copyID = UIAction(title: "copy-id".localized, image: UIImage(systemName: "paperclip")) { _ in
                let package = self.dataProvider.packages[indexPath.row]
                UIPasteboard.general.string = package.id
            }

            let remove = UIAction(
                title: "remove-btn".localized,
                image: UIImage(systemName: "trash"),
                attributes: [.destructive],
                handler: { _ in
                    self.removePackage(at: indexPath)
                }
            )

            return UIMenu(children: [copyID, remove])
        })

        return configurator
    }
}
