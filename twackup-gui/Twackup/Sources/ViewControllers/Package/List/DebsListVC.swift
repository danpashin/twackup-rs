//
//  DebsListVC.swift
//  Twackup
//
//  Created by Daniil on 01.12.2022.
//

import BlankSlate

final class DebsListVC: SelectablePackageListVC<DebPackage> {
    override class var metadata: (any ViewControllerMetadata)? {
        BuildedPkgsMetadata()
    }

    var debsDataSource: DebsListDataSource {
        dataSource as! DebsListDataSource // swiftlint:disable:this force_cast
    }

    private let databaseProvider: DatabasePackageProvider

    private(set) lazy var removeAllBarBtn: UIBarButtonItem = {
        UIBarButtonItem(title: "debs-remove-all-btn".localized, primaryAction: UIAction { [self] _ in
            setEditing(false, animated: true)
            askAndDelete(packages: databaseProvider.allPackages)
        })
    }()

    private(set) lazy var removeSelectedBarBtn: UIBarButtonItem = {
        UIBarButtonItem(title: "debs-remove-selected-btn".localized, primaryAction: UIAction { [self] _ in
            askAndDelete(packages: dataSource.selected())
        })
    }()

    private(set) lazy var shareSelectedBarBtn: UIBarButtonItem = {
        let title = "debs-share-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionShareSelected))
    }()

    private var reloadObserver: NSObjectProtocol?

    override init(mainModel: MainModel, detail: PackageDetailVC<DebPackage>) {
        databaseProvider = DatabasePackageProvider(mainModel.database)

        super.init(mainModel: mainModel, detail: detail)

        let center = NotificationCenter.default
        reloadObserver = center.addObserver(forName: .DebsReload, object: nil, queue: .main) { [weak self] _ in
            self?.reloadData(animated: true, force: true)
        }
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    deinit {
        Task { @MainActor in
            NotificationCenter.default.removeObserver(reloadObserver as Any)
        }
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        tableView.bs.dataSource = self
    }

    override func selectionDidUpdate() {
        super.selectionDidUpdate()

        if isEditing {
            let isAnySelected = dataSource.isAnySelected

            shareSelectedBarBtn.isEnabled = isAnySelected
            guard var buttons = toolbarItems, !buttons.isEmpty else { return }
            buttons[0] = isAnySelected ? removeSelectedBarBtn : removeAllBarBtn
            setToolbarItems(buttons, animated: false)
        }
    }

    override func configureDataSource() -> PackageListDataSource<DebPackage> {
        let cell = DebTableViewCell.self
        let cellID = String(describing: cell)
        tableView.register(cell, forCellReuseIdentifier: cellID)

        return DebsListDataSource(tableView: tableView, dataProvider: databaseProvider) { table, indexPath, package in
            let cell = table.dequeueReusableCell(withIdentifier: cellID, for: indexPath)
            if let cell = cell as? DebTableViewCell {
                cell.package = package
            }

            return cell
        }
    }

    override func configureTableDelegate() -> PackageListDelegate<DebPackage> {
        DebsListDelegate(dataSource: debsDataSource, listController: self)
    }

    override func setEditing(_ editing: Bool, animated: Bool) {
        super.setEditing(editing, animated: animated)

        if editing {
            shareSelectedBarBtn.isEnabled = false

            let spacer = UIBarButtonItem(barButtonSystemItem: .flexibleSpace, target: nil, action: nil)
            setToolbarItems([removeAllBarBtn, spacer, shareSelectedBarBtn], animated: false)
        }

        navigationController?.setToolbarHidden(!editing, animated: animated)
    }

    // MARK: - Actions

    @objc
    func actionShareSelected(_ button: UIBarButtonItem) {
        let debURLS = dataSource.selected().map { $0.fileURL }
        if debURLS.isEmpty { return }

        let activityVC = UIActivityViewController(activityItems: debURLS, applicationActivities: nil)
        activityVC.popoverPresentationController?.barButtonItem = button

        present(activityVC, animated: true, completion: nil)
    }

    func askAndDelete(packages: [DebPackage]) {
        let alert = UIAlertController(
            title: "deb-remove-alert-title".localized,
            message: "deb-remove-alert-subtitle".localized,
            preferredStyle: .alert
        )

        alert.addAction(UIAlertAction(title: "deb-remove-alert-ok".localized, style: .destructive) { [self] _ in
            Task {
                await debsDataSource.delete(packages: packages)
            }
        })

        alert.addAction(UIAlertAction(title: "cancel".localized, style: .cancel))

        present(alert, animated: true)
    }
}
