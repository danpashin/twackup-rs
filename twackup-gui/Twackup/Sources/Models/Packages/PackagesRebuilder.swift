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

    private var rebuildedPackages: [DebPackage] = []

    private lazy var rebuildedLock: UnsafeMutablePointer<os_unfair_lock> = {
        let lock = UnsafeMutablePointer<os_unfair_lock>.allocate(capacity: 1)
        lock.initialize(to: .init())
        return lock
    }()

    init(dpkg: Dpkg, database: Database) {
        self.dpkg = dpkg
        self.database = database
    }

    deinit {
        rebuildedLock.deinitialize(count: 1)
        rebuildedLock.deallocate()
    }

    func rebuild(packages: [Package], completion: (() -> Void)? = nil) {
        dpkg.buildDelegate = self
        guard let appDelegate = UIApplication.shared.delegate as? AppDelegate else {
            completion?()
            return
        }

        DispatchQueue.global().async {
            do {
                let results = try self.dpkg.rebuild(packages: packages)
                for result in results {
                    switch result {
                    case .success: continue
                    case .failure(let error):
                        appDelegate.logger.log(message: FFILogger.Message(text: "\(error)"), level: .error)
                    }
                }
            } catch {
                appDelegate.logger.log(message: FFILogger.Message(text: "\(error)"), level: .error)
            }

            completion?()
        }
    }

    func startProcessing(package: Package) {

    }

    func finishedProcessing(package: Package, debPath: URL) {
        let model = database.createBuildedPackage()
        model.setProperties(package: package)
        model.setProperties(file: debPath, pathRelativeTo: Dpkg.defaultSaveDirectory)

        os_unfair_lock_lock(rebuildedLock)
        rebuildedPackages.append(model)
        os_unfair_lock_unlock(rebuildedLock)
    }

    func finishedAll() {
        database.addBuildedPackages(rebuildedPackages)
        NotificationCenter.default.post(name: PackageVC.DebsListModel.NotificationName, object: nil)
    }
}
