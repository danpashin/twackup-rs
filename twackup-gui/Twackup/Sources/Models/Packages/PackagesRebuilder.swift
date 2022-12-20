//
//  PackagesRebuilder.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import Sentry

// swiftlint:disable legacy_objc_type
class PackagesRebuilder: DpkgProgressDelegate {
    let mainModel: MainModel

    private class State {
        var progress: Progress

        let queue: DispatchQueue

        let updateHandler: ((Progress) -> Void)?

        var donePackages: [BuildedPackage] = []

        init(progress: Progress, updateHandler: ((Progress) -> Void)?) {
            self.progress = progress
            self.updateHandler = updateHandler

            queue = DispatchQueue(label: "twackup.rebuild", qos: .userInitiated)
        }
    }

    private struct PerformanceMeasurer {
        var rootTransaction: Span

        var childTransactions: NSCache<NSString, Span> = NSCache()
    }

    private var buildState: State?

    private var performanceMeasurer: PerformanceMeasurer?

    init(mainModel: MainModel) {
        self.mainModel = mainModel
    }

    /// Performs package rebuilding from fs files to debs
    /// - Parameters:
    ///   - packages: packages that needs to be rebuild
    ///   - updateHandler: Handler that contains current progress of rebuild operation
    ///   - completion: Handler that will be called after rebuild will complete
    func rebuild(packages: [FFIPackage], updateHandler: ((Progress) -> Void)? = nil, completion: (() -> Void)? = nil) {
        if packages.isEmpty {
            completion?()
            return
        }

        let buildState = State(progress: Progress(totalUnitCount: Int64(packages.count)), updateHandler: updateHandler)
        self.buildState = buildState

        mainModel.dpkg.buildProgressDelegate = self

        DispatchQueue.global().async { [self] in
            let measurer = PerformanceMeasurer(
                rootTransaction: SentrySDK.startTransaction(name: "multiple-debs-rebuild", operation: "lib")
            )
            performanceMeasurer = measurer

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

            buildState.queue.async(flags: .barrier) { [self] in
                addPackagesToDatabase { [self] in
                    measurer.rootTransaction.finish()
                    performanceMeasurer = nil

                    buildState.donePackages.removeAll()
                }

                completion?()
            }
        }
    }

    func startProcessing(package: Package) {
        guard let performanceMeasurer else { return }
        let transaction = performanceMeasurer.rootTransaction.startChild(
            operation: "single-deb-rebuild",
            description: package.id
        )
        performanceMeasurer.childTransactions.setObject(transaction, forKey: package.id as NSString)
    }

    func finishedProcessing(package: Package, debPath: URL) {
        guard let buildState else { return }

        if let performanceMeasurer {
            if let transaction = performanceMeasurer.childTransactions.object(forKey: package.id as NSString) {
                transaction.finish()
            }
            performanceMeasurer.childTransactions.removeObject(forKey: package.id as NSString)
        }

        buildState.queue.async {
            buildState.progress.completedUnitCount += 1
            buildState.donePackages.append(BuildedPackage(package: package, debURL: debPath))
            buildState.updateHandler?(buildState.progress)
        }
    }

    func finishedAll() {
    }

    /// Adds builded packages to database
    /// - Parameter completion: completion that will be executed at end of save operation
    private func addPackagesToDatabase(completion: (() -> Void)? = nil) {
        guard let buildState, let performanceMeasurer else { return }

        let databaseTransaction = performanceMeasurer.rootTransaction.startChild(operation: "database-packages-save")
        mainModel.database.addBuildedPackages(buildState.donePackages) {
            databaseTransaction.finish()

            NotificationCenter.default.post(name: DebsListModel.NotificationName, object: nil)
            completion?()
        }
    }
}
