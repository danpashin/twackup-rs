//
//  PackageParser.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

protocol DpkgBuildDelegate: AnyObject {
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

    weak var buildDelegate: DpkgBuildDelegate?

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

            packages.append(pModel as Package)
        }

        return packages
    }

    func rebuild(packages: [Package], outDir: URL = defaultSaveDirectory) throws -> [Result<URL, NSError>] {
        let preferences = Preferences()

        let innerPkgs = packages.compactMap { ($0 as? FFIPackage)?.pkg }
        let ffiPackages = innerPkgs.withUnsafeBufferPointer { pointer in
            // safe to unwrap?
            slice_ref_TwPackage_t(ptr: pointer.baseAddress!, len: pointer.count)
        }

        var ffiResults = slice_boxed_TwPackagesRebuildResult()

        var buildParameters = TwBuildParameters_t()
        buildParameters.packages = ffiPackages
        buildParameters.functions = createProgressFuncs()
        buildParameters.preferences.compression_level = TwCompressionLevel_t(preferences.compression.level.rawValue)
        buildParameters.preferences.compression_type = TwCompressionType_t(preferences.compression.kind.rawValue)

        withUnsafeMutablePointer(to: &ffiResults) { buildParameters.results = $0 }

        let status = outDir.path.utf8CString.withUnsafeBufferPointer { pointer in
            // safe to unwrap?
            buildParameters.out_dir = pointer.baseAddress!
            return tw_rebuild_packages(innerDpkg, buildParameters)
        }

        if status != TwResult_t(TW_RESULT_OK) {
            fatalError()
        }

        var results: [Result<URL, NSError>] = []
        results.reserveCapacity(ffiResults.len)

        for result in UnsafeBufferPointer(start: ffiResults.ptr, count: ffiResults.len) {
            if result.success {
                // safe to unwrap here 'cause Rust string is UTF-8 encoded string
                let path = String(ffiSlice: result.deb_path)!
                results.append(.success(URL(fileURLWithPath: path)))
            } else {
                results.append(.failure(NSError(domain: "ru.danpashin.twackup", code: 0, userInfo: [
                    NSLocalizedDescriptionKey: "\(String(ffiSlice: result.error) ?? "")"
                ])))
            }
        }

        tw_free_rebuild_results(ffiResults)

        return results
    }

    private func createProgressFuncs() -> TwProgressFunctions {
        var funcs = TwProgressFunctions()
        funcs.context = Unmanaged<Dpkg>.passUnretained(self).toOpaque()
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
