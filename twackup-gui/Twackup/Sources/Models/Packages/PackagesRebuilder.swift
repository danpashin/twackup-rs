//
//  PackagesRebuilder.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import Foundation

class PackagesRebuilder: DpkgBuildDelegate {
    let dpkg: Dpkg

    let database: Database

    private var rebuildedPackages: [BuildedPackage] = []

    private let dbSaveQueue: DispatchQueue = DispatchQueue(label: "database-save", qos: .default)

    init(dpkg: Dpkg, database: Database) {
        self.dpkg = dpkg
        self.database = database
    }

    func rebuild(packages: [Package], completion: (() -> Void)? = nil) {
        dpkg.buildDelegate = self

        DispatchQueue.global().async {
            do {
                let results = try self.dpkg.rebuild(packages: packages)
                for result in results {
                    switch result {
                    case .success: continue
                    case .failure(let error):
                        FFILogger.shared.log("\(error)", level: .error)
                    }
                }
            } catch {
                FFILogger.shared.log("\(error)", level: .error)
            }

            completion?()
        }
    }

    func startProcessing(package: Package) {

    }

    func finishedProcessing(package: Package, debPath: URL) {
        dbSaveQueue.async { [self] in
            rebuildedPackages.append(BuildedPackage(package: package, debURL: debPath))
        }
    }

    func finishedAll() {
        dbSaveQueue.async { [self] in
            database.addBuildedPackages(rebuildedPackages) {
                NotificationCenter.default.post(name: PackageVC.DebsListModel.NotificationName, object: nil)
            }
        }
    }
}
