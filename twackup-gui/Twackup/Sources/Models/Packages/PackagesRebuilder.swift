//
//  PackagesRebuilder.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import Sentry

// swiftlint:disable legacy_objc_type
actor PackagesRebuilder: DpkgProgressSubscriber, Equatable, Hashable {
    let mainModel: MainModel

    nonisolated private let uuid = UUID()

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

        mainModel.dpkg.progressNotifier.addSubscriber(self)
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

        func log(error: Error) async {
            await FFILogger.shared.log("\(error)", level: .error)
            SentrySDK.capture(error: error)
        }

        do {
            let saveDir = await mainModel.preferences.saveDirectory
            let results = try await mainModel.dpkg.rebuild(packages: packages, outDir: saveDir)
            for result in results {
                switch result {
                case .success: continue
                case .failure(let error): await log(error: error)
                }
            }
        } catch {
            await log(error: error)
        }

        await addPackagesToDatabase()
        performanceMeasurer?.rootTransaction.finish()
        performanceMeasurer = nil
    }

    // MARK: - Private methods

    /// Adds builded packages to database
    /// - Parameter completion: completion that will be executed at end of save operation
    private func addPackagesToDatabase() async {
        guard let performanceMeasurer else { return }

        let databaseTransaction = performanceMeasurer.rootTransaction.startChild(operation: "database-packages-save")

        await mainModel.database.add(packages: donePackages)
        databaseTransaction.finish()

        NotificationCenter.default.post(name: .DebsReload, object: nil)
    }

    // MARK: - DpkgProgressSubscriber

    func startProcessing(package: FFIPackage) {
        guard let performanceMeasurer else { return }
        let transaction = performanceMeasurer.rootTransaction.startChild(
            operation: "single-deb-rebuild",
            description: package.id
        )
        performanceMeasurer.childTransactions.setObject(transaction, forKey: package.id as NSString)
    }

    func finishedProcessing(package: FFIPackage, debURL: URL) {
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

    // MARK: - Equatable

    static func == (lhs: PackagesRebuilder, rhs: PackagesRebuilder) -> Bool {
        lhs.uuid == rhs.uuid
    }

    // MARK: - Hashable

    nonisolated func hash(into hasher: inout Hasher) {
        hasher.combine(uuid)
    }
}

// swiftlint:enable legacy_objc_type
