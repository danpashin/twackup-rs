//
//  DpkgProgressNotifier.swift
//  Twackup
//
//  Created by Daniil on 14.06.2024.
//
protocol DpkgProgressSubscriber: Hashable, Sendable {
    /// Being called when package is ready to start it's rebuilding operation
    func startProcessing(package: FFIPackage) async

    /// Being called when package just finished it's rebuilding operation
    func finishedProcessing(package: FFIPackage, debURL: URL) async

    /// Being called when all packages are processed
    func finishedAll() async
}

final class DpkgProgressNotifier: @unchecked Sendable {
    private(set) var ffiFunctions = TwProgressFunctions()

    private var untypedSubscribers: Set<AnyHashable> = []

    private var subscribers: [any DpkgProgressSubscriber] {
        // Since subscriber can only be added via addSubscriber() it must conform to DpkgProgressSubscriber
        // So force casting is safe here
        untypedSubscribers.map { $0 as! any DpkgProgressSubscriber }  // swiftlint:disable:this force_cast
    }

    init() {
        initFuncs()
    }

    func addSubscriber(_ subscriber: any DpkgProgressSubscriber) {
        untypedSubscribers.insert(AnyHashable(subscriber))
    }

    func removeSubscriber(_ subscriber: any DpkgProgressSubscriber) {
        untypedSubscribers.remove(subscriber)
    }

    // MARK: - Private methods

    private func initFuncs() {
        ffiFunctions.context = Unmanaged.passUnretained(self).toOpaque()
        ffiFunctions.started_processing = { context, package in
            guard let context, let ffiPackage = FFIPackage(package) else {
                tw_package_release(package.inner)
                return
            }

            let dpkg = Unmanaged<DpkgProgressNotifier>.fromOpaque(context).takeUnretainedValue()
            dpkg.startProcessing(ffiPackage)
        }
        ffiFunctions.finished_processing = { context, package, debPath in
            guard let context, let ffiPackage = FFIPackage(package),
                  let debPath = String(ffiSlice: debPath)
            else {
                tw_package_release(package.inner)
                return
            }

            let dpkg = Unmanaged<DpkgProgressNotifier>.fromOpaque(context).takeUnretainedValue()
            let debURL = URL(fileURLWithPath: debPath)
            dpkg.finishedProcessing(ffiPackage, debURL: debURL)
        }
        ffiFunctions.finished_all = { context in
            guard let context else { return }

            let dpkg = Unmanaged<DpkgProgressNotifier>.fromOpaque(context).takeUnretainedValue()
            dpkg.finishedAll()
        }
    }

    private func startProcessing(_ package: FFIPackage) {
        subscribers.forEach { subscriber in
            Task(priority: .utility) {
                await subscriber.startProcessing(package: package)
            }
        }
    }

    private func finishedProcessing(_ package: FFIPackage, debURL: URL) {
        subscribers.forEach { subscriber in
            Task(priority: .utility) {
                await subscriber.finishedProcessing(package: package, debURL: debURL)
            }
        }
    }

    private func finishedAll() {
        subscribers.forEach { subscriber in
            Task(priority: .utility) {
                await subscriber.finishedAll()
            }
        }
    }
}
