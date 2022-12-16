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

    private var pfmcRootTransaction: Span?

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
            pfmcRootTransaction = SentrySDK.startTransaction(name: "multiple-debs-rebuild", operation: "lib")

            do {
                let results = try self.dpkg.rebuild(packages: packages)
                for result in results {
                    switch result {
                    case .success: continue

                    case .failure(let error):
                        FFILogger.shared.log("\(error)", level: .error)
                        SentrySDK.capture(error: error)
                    }
                }
            } catch {
                FFILogger.shared.log("\(error)", level: .error)
                SentrySDK.capture(error: error)
            }

            pfmcRootTransaction?.finish()
            pfmcRootTransaction = nil

            completion?()
        }
    }

    func startProcessing(package: Package) {
        guard let pfmcRootTransaction else { return }
        let transaction = pfmcRootTransaction.startChild(operation: "single-deb-rebuild", description: package.id)
        performanceTransactions.setObject(transaction, forKey: package.id as NSString)
    }

    func finishedProcessing(package: Package, debPath: URL) {
        if let transaction = performanceTransactions.object(forKey: package.id as NSString) {
            transaction.finish()
        }
        performanceTransactions.removeObject(forKey: package.id as NSString)

        dbSaveQueue.async { [self] in
            if let progress {
                progress.completedUnitCount += 1
                updateHandler?(progress)
            }

            rebuildedPackages.append(BuildedPackage(package: package, debURL: debPath))
        }
    }

    func finishedAll() {
        dbSaveQueue.async { [self] in
            let databaseTransaction = pfmcRootTransaction?.startChild(operation: "database-packages-save")
            database.addBuildedPackages(rebuildedPackages) { [self] in
                NotificationCenter.default.post(name: PackageVC.DebsListModel.NotificationName, object: nil)

                databaseTransaction?.finish()
                pfmcRootTransaction?.finish()
                pfmcRootTransaction = nil
            }
        }
    }
}
