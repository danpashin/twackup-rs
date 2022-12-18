//
//  PackagesRebuilder.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import Sentry

// swiftlint:disable legacy_objc_type
class PackagesRebuilder: DpkgBuildDelegate {
    let mainModel: MainModel

    private var rebuildedPackages: [BuildedPackage] = []

    private(set) lazy var dbSaveQueue: OperationQueue = {
        let queue = OperationQueue()
        queue.maxConcurrentOperationCount = 1
        queue.qualityOfService = .userInitiated

        return queue
    }()

    private var updateHandler: ((Progress) -> Void)?

    private var pfmcRootTransaction: Span?

    private var performanceTransactions: NSCache<NSString, Span> = NSCache()

    init(mainModel: MainModel) {
        self.mainModel = mainModel
    }

    func rebuild(packages: [Package], updateHandler: ((Progress) -> Void)? = nil, completion: (() -> Void)? = nil) {
        mainModel.dpkg.buildDelegate = self
        self.updateHandler = updateHandler

        dbSaveQueue.progress.totalUnitCount = Int64(packages.count)

        DispatchQueue.global().async { [self] in
            pfmcRootTransaction = SentrySDK.startTransaction(name: "multiple-debs-rebuild", operation: "lib")

            do {
                let results = try mainModel.dpkg.rebuild(packages: packages)
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

        dbSaveQueue.addOperation { [self] in
            rebuildedPackages.append(BuildedPackage(package: package, debURL: debPath))

            dbSaveQueue.progress.completedUnitCount += 1
            updateHandler?(dbSaveQueue.progress)
        }
    }

    func finishedAll() {
        dbSaveQueue.addBarrierBlock { [self] in
            let databaseTransaction = pfmcRootTransaction?.startChild(operation: "database-packages-save")
            mainModel.database.addBuildedPackages(rebuildedPackages) { [self] in
                databaseTransaction?.finish()
                pfmcRootTransaction?.finish()
                pfmcRootTransaction = nil

                NotificationCenter.default.post(name: DebsListModel.NotificationName, object: nil)
            }
        }
    }
}
