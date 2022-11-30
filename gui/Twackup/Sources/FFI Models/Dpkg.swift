//
//  PackageParser.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

import Foundation

protocol DpkgBuildDelegate: AnyObject {
    func printMessage(_ message: String, level: Dpkg.MessageLevel)
    func startProcessing(package: Package)
    func finishedProcessing(package: Package, debPath: String)
    func finishedAll()
}

class Dpkg {

    enum MessageLevel: UInt8 {
        case debug
        case info
        case warning
        case error
    }

    var buildDelegate: DpkgBuildDelegate?

    private let innerDpkg: UnsafeMutablePointer<TwDpkg_t>

    init(path: String = "/var/lib/dpkg", lock: Bool = false) {
        innerDpkg = tw_init(path, false)
    }

    deinit {
        tw_free(innerDpkg)
    }

    func parsePackages(leaves: Bool) -> [Package] {
        let ffiPackages = tw_get_packages(innerDpkg, leaves, TwPackagesSort_t(TW_PACKAGES_SORT_NAME))

        let rawPackages = UnsafeBufferPointer(start: ffiPackages.ptr, count: ffiPackages.len)

        var packages: [Package] = []
        packages.reserveCapacity(rawPackages.count)

        for package in rawPackages {
            if let packageModel = FFIPackage(package) {
                packages.append(packageModel as any Package)
            }
        }

        rawPackages.deallocate()

        return packages
    }

    func rebuild(packages: [Package], outDir: URL) {

        let innerPkgs = packages.compactMap({ ($0 as? FFIPackage)?.pkg })
        let ffiPackages = innerPkgs.withUnsafeBufferPointer {
            // safe to unwrap?
            slice_ref_TwPackage_t(ptr: $0.baseAddress!, len: $0.count)
        }

        var results = slice_boxed_TwPackagesRebuildResult()
        let status = outDir.path.utf8CString.withUnsafeBufferPointer {
            // safe to unwrap?
            tw_rebuild_packages(innerDpkg, ffiPackages, createProgressFuncs(), $0.baseAddress!, &results)
        }

        print("status = \(status)")
        assert(status == TwResult_t(TW_RESULT_OK))

        let resultsBuf = UnsafeBufferPointer(start: results.ptr, count: results.len)
        for result in resultsBuf {
            let error = String(ffiSlice: result.error)
            let path = String(ffiSlice: result.deb_path)
            
            print("success = \(result.success)")
            print("path = \"\(String(describing: path))\"; error = \(String(describing: error))")
        }

        tw_free_rebuild_results(results)
    }

    private func createProgressFuncs() -> TwProgressFunctions {
        var funcs = TwProgressFunctions()
        funcs.context = Unmanaged<Dpkg>.passUnretained(self).toOpaque()
        funcs.print_message = { context, message, msgLevel in
            guard let context,
                  let message = String(ffiSlice: message),
                  let msgLevel = MessageLevel(rawValue: msgLevel)
            else { return }

            let dpkg = Unmanaged<Dpkg>.fromOpaque(context).takeUnretainedValue()
            dpkg.buildDelegate?.printMessage(message, level: msgLevel)
        }
        funcs.started_processing = { context, package in
            guard let context, let package, let ffiPackage = FFIPackage(package.pointee) else { return }

            let dpkg = Unmanaged<Dpkg>.fromOpaque(context).takeUnretainedValue()
            dpkg.buildDelegate?.startProcessing(package: ffiPackage)
        }
        funcs.finished_processing = { context, package, debPath in
            guard let context,
                  let package,
                  let ffiPackage = FFIPackage(package.pointee),
                  let debPath = String(ffiSlice: debPath)
            else { return }

            let dpkg = Unmanaged<Dpkg>.fromOpaque(context).takeUnretainedValue()
            dpkg.buildDelegate?.finishedProcessing(package: ffiPackage, debPath: debPath)
        }
        funcs.finished_all = { context in
            guard let context else { return }
            let dpkg = Unmanaged<Dpkg>.fromOpaque(context).takeUnretainedValue()
            dpkg.buildDelegate?.finishedAll()
        }

        return funcs
    }
}
