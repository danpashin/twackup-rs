//
//  PackagesRebuilder.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import Sentry

// swiftlint:disable legacy_objc_type
actor PackagesRebuilder: DpkgProgress {
    let mainModel: MainModel

    private final class PerformanceMeasurer: @unchecked Sendable {
        let rootTransaction: Span

        let childTransactions: NSCache<NSString, Span> = NSCache()

        init(rootTransaction: Span) {
            self.rootTransaction = rootTransaction
        }
    }

    private(set) var progress: Progress = Progress()

    let updateHandler: (@Sendable (Progress) -> Void)?

    private(set) var donePackages: [BuildedPackage] = []

    private var performanceMeasurer: PerformanceMeasurer?

    init(mainModel: MainModel, updateHandler: (@Sendable (Progress) -> Void)? = nil) {
        self.mainModel = mainModel
        self.updateHandler = updateHandler
//        mainModel.dpkg.buildProgressDelegate = self
    }

    /// Performs package rebuilding from fs files to debs
    /// - Parameters:
    ///   - packages: packages that needs to be rebuild
    ///   - updateHandler: Handler that contains current progress of rebuild operation
    ///   - completion: Handler that will be called after rebuild will complete
    func rebuild(packages: [FFIPackage]) async {
        if packages.isEmpty {
            return
        }

        progress = Progress(totalUnitCount: Int64(packages.count))
        performanceMeasurer = PerformanceMeasurer(
            rootTransaction: SentrySDK.startTransaction(name: "multiple-debs-rebuild", operation: "lib")
        )

        do {
            let results = try await mainModel.dpkg.rebuild(packages: packages)
            for result in results {
                switch result {
                case .success: continue

                case .failure(let error):
                    await FFILogger.shared.log("\(error)", level: .error)
                    SentrySDK.capture(error: error)
                }
            }
        } catch {
            await FFILogger.shared.log("\(error)", level: .error)
            SentrySDK.capture(error: error)
        }

        await addPackagesToDatabase()
        performanceMeasurer?.rootTransaction.finish()
        performanceMeasurer = nil
    }

    func startProcessing(package: Package) {
        guard let performanceMeasurer else { return }
        let transaction = performanceMeasurer.rootTransaction.startChild(
            operation: "single-deb-rebuild",
            description: package.id
        )
        performanceMeasurer.childTransactions.setObject(transaction, forKey: package.id as NSString)
    }

    func finishedProcessing(package: Package, debURL: URL) {
        if let performanceMeasurer {
            if let transaction = performanceMeasurer.childTransactions.object(forKey: package.id as NSString) {
                transaction.finish()
            }
            performanceMeasurer.childTransactions.removeObject(forKey: package.id as NSString)
        }

        progress.completedUnitCount += 1
        donePackages.append(BuildedPackage(package: package, debURL: debURL))
        updateHandler?(progress)
    }

    func finishedAll() {
    }

    /// Adds builded packages to database
    /// - Parameter completion: completion that will be executed at end of save operation
    private func addPackagesToDatabase() async {
        guard let performanceMeasurer else { return }

        let databaseTransaction = performanceMeasurer.rootTransaction.startChild(operation: "database-packages-save")

        await mainModel.database.add(packages: donePackages)
        databaseTransaction.finish()
        await NotificationCenter.default.post(name: DebsListModel.NotificationName, object: nil)
    }
}
// swiftlint:enable legacy_objc_type
