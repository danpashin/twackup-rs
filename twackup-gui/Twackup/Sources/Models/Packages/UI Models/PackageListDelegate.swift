//
//  PackageListDelegate.swift
//  Twackup
//
//  Created by Daniil on 17.06.2024.
//

class PackageListDelegate<P: Package>: NSObject, UITableViewDelegate {
    private(set) var dataSource: PackageListDataSource<P>

    private(set) weak var listController: PackageListVC<P>?

    init(dataSource: PackageListDataSource<P>, listController: PackageListVC<P>) {
        self.dataSource = dataSource
        self.listController = listController
    }

    func selectAll(animated: Bool = true) {
        let snapshot = dataSource.snapshot()

        for section in 0..<snapshot.numberOfSections {
            for row in 0..<snapshot.numberOfItems(inSection: section) {
                let indexPath = IndexPath(row: row, section: section)
                dataSource.tableView.selectRow(at: indexPath, animated: animated, scrollPosition: .none)
            }
        }

        listController?.selectionDidUpdate()
    }

    func deselectAll(animated: Bool = true) {
        guard let indexPaths = dataSource.tableView.indexPathsForSelectedRows else { return }
        indexPaths.forEach { indexPath in
            dataSource.tableView.deselectRow(at: indexPath, animated: animated)
        }

        listController?.selectionDidUpdate()
    }

    func didSelect(package: P, at indexPath: IndexPath) {
        if !(listController?.isEditing ?? true) {
            openDetail(for: package, at: indexPath)
        }
    }

    func openDetail(for package: P, at indexPath: IndexPath) {
        guard
            let listController,
            !listController.isEditing,
            let detailNav = listController.detail.navigationController
        else { return }

        if listController.splitViewController?.isCollapsed ?? false {
            dataSource.tableView.deselectRow(at: indexPath, animated: true)
        }

        listController.detail.package = package
        listController.showDetailViewController(detailNav, sender: nil)
    }

    // MARK: - UITableViewDelegate

    func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        if let package = dataSource.package(for: indexPath) {
            didSelect(package: package, at: indexPath)
        }

        listController?.selectionDidUpdate()
    }

    func tableView(_ tableView: UITableView, didDeselectRowAt indexPath: IndexPath) {
        listController?.selectionDidUpdate()
    }
}
