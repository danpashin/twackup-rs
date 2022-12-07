//
//  PackageParser.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

protocol DpkgBuildDelegate: AnyObject {
    func printMessage(_ message: String, level: Dpkg.MessageLevel)
    func startProcessing(package: Package)
    func finishedProcessing(package: Package, debPath: URL)
    func finishedAll()
}

class Dpkg {

    enum MessageLevel: UInt8 {
        case debug
        case info
        case warning
        case error
    }

    static let defaultSaveDirectory: URL = {
        FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
    }()

    var buildDelegate: DpkgBuildDelegate?

    private let innerDpkg: UnsafeMutablePointer<TwDpkg_t>

    init(path: String = "/var/lib/dpkg", lock: Bool = false) {
        innerDpkg = tw_init(path, false)
    }

    deinit {
        tw_free(innerDpkg)
    }

    func parsePackages(leaves: Bool) -> [Package] {
        var rawPkgs = slice_boxed_TwPackage_t()
        let result = tw_get_packages(innerDpkg, leaves, TwPackagesSort_t(TW_PACKAGES_SORT_NAME), &rawPkgs)
        if result != TwResult_t(TW_RESULT_OK) { return [] }

        let ffiPackages = UnsafeConsumingCollection(raw: rawPkgs.ptr, length: rawPkgs.len)

        var packages: [Package] = []
        packages.reserveCapacity(ffiPackages.count)

        for package in ffiPackages {
            guard let pModel = FFIPackage(package) else {
                package.deallocate(package.inner_ptr)
                continue
            }
            
            packages.append(pModel as any Package)
        }

        return packages
    }

    func rebuild(packages: [Package], outDir: URL = defaultSaveDirectory) -> Bool {
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

        let resultsBuf = UnsafeConsumingCollection(raw: results.ptr, length: results.len)
        for result in resultsBuf {
            let error = String(ffiSlice: result.error, deallocate: true)
            let path = String(ffiSlice: result.deb_path, deallocate: true)

            print("success = \(result.success)")
            print("path = \"\(String(describing: path))\"; error = \(String(describing: error))")
        }

        return status == TwResult_t(TW_RESULT_OK)
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
            dpkg.buildDelegate?.finishedProcessing(package: ffiPackage, debPath: URL(fileURLWithPath: debPath))
        }
        funcs.finished_all = { context in
            guard let context else { return }
            let dpkg = Unmanaged<Dpkg>.fromOpaque(context).takeUnretainedValue()
            dpkg.buildDelegate?.finishedAll()
        }

        return funcs
    }
}
