//
//  PackagesViewController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class PackagesViewController: UITableViewController {
    private(set) var model: PackagesVCModel

    init(_ model: PackagesVCModel) {
        self.model = model

        super.init(style: .plain)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        navigationItem.title = model.navTitle
        navigationController?.navigationBar.prefersLargeTitles = true

        tableView.backgroundColor = .systemBackground
        tableView.register(PackageTableViewCell.self, forCellReuseIdentifier: "PackageCell")
    }

    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return model.packages.count
    }

    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "PackageCell", for: indexPath)
        if let cell = cell as? PackageTableViewCell {
            cell.package = model.packages[indexPath.row]
        }

        return cell
    }

    override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        model.detailDelegate?.didSelectPackage(model.packages[indexPath.row])
    }
}
