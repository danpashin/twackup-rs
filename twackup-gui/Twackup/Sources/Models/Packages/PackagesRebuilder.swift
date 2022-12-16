//
//  PackagesRebuilder.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import Sentry

// swiftlint:disable legacy_objc_type
class PackagesRebuilder: DpkgBuildDelegate {
    let dpkg: Dpkg

    let database: Database

    private var rebuildedPackages: [BuildedPackage] = []

    private let dbSaveQueue: DispatchQueue = DispatchQueue(label: "database-save", qos: .default)

    private var updateHandler: ((Progress) -> Void)?

    private var progress: Progress?

    private var performanceRootTransaction: Span?

    private var performanceTransactions: NSCache<NSString, Span> = NSCache()

    init(dpkg: Dpkg, database: Database) {
        self.dpkg = dpkg
        self.database = database
    }

    func rebuild(packages: [Package], updateHandler: ((Progress) -> Void)? = nil, completion: (() -> Void)? = nil) {
        dpkg.buildDelegate = self
        self.updateHandler = updateHandler

        progress = Progress(totalUnitCount: Int64(packages.count))

        DispatchQueue.global().async { [self] in
            performanceRootTransaction = SentrySDK.startTransaction(name: "Rebuild debs", operation: "lib")

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

            performanceRootTransaction?.finish()

            completion?()
        }
    }

    func startProcessing(package: Package) {
        let transaction = performanceRootTransaction?.startChild(operation: package.id)
        performanceTransactions.setObject(transaction!, forKey: package.id as NSString)
    }

    func finishedProcessing(package: Package, debPath: URL) {
        if let transaction = performanceTransactions.object(forKey: package.id as NSString) {
            transaction.finish()
        }
        performanceTransactions.removeObject(forKey: package.id as NSString)

        dbSaveQueue.async { [self] in
            if let progress {
                progress.completedUnitCount += 1

                if let updateHandler {
                    updateHandler(progress)
                }
            }

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
