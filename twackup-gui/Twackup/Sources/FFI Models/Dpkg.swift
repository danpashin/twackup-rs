//
//  PackageParser.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

import Sentry

protocol DpkgProgress: Actor {
    /// Being called when package is ready to start it's rebuilding operation
    func startProcessing(package: Package)

    /// Being called when package just finished it's rebuilding operation
    func finishedProcessing(package: Package, debURL: URL)

    /// Being called when all packages are processed
    func finishedAll()
}

actor Dpkg {
    enum MessageLevel: UInt8 {
        case debug
        case info
        case warning
        case error
    }

    nonisolated static let defaultSaveDirectory: URL = {
        FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
    }()

    /// Delegate which will be called on build state update
    weak var buildProgressDelegate: DpkgProgress?

    private let innerDpkg: UnsafeMutablePointer<TwDpkg_t>

    private let preferences: Preferences

    private var buildParameters = TwBuildParameters_t()

    init(path: String, preferences: Preferences, lock: Bool = false) {
        innerDpkg = tw_init(path, lock)
        self.preferences = preferences
    }

    deinit {
        tw_free(innerDpkg)
    }

    /// Parses packages from dpkg database
    /// - Parameter onlyLeaves: True if only leaves packages should be returned. Otherwise, false
    /// - Returns: Array of parsed packages. Improper packages will be skipped
    func parsePackages(onlyLeaves: Bool) throws -> [FFIPackage] {
        var rawPkgs = slice_boxed_TwPackage_t()
        let result = tw_get_packages(innerDpkg, onlyLeaves, TW_PACKAGES_SORT_NAME, &rawPkgs)
        if result != TW_RESULT_OK || rawPkgs.ptr == nil {
            throw NSError(domain: "ru.danpashin.twackup", code: 0, userInfo: [
                NSLocalizedDescriptionKey: "FFI returned \(result) code. Critical bug?"
            ])
        }

        let packages = UnsafeBufferPointer(start: rawPkgs.ptr, count: rawPkgs.len)
            .compactMap { package in
                let model = FFIPackage(package)
                if model == nil {
                    package.deallocate(package.inner_ptr)
                }

                return model
            }

        rawPkgs.ptr.deallocate()

        return packages
    }

    /// Rebuilds packages and saves them to specified directory
    /// - Parameters:
    ///   - packages: Packages that should be rebuilt
    ///   - outDir: Directory that will contain debs of packages
    /// - Returns: Array with results.
    /// Every result contains full deb path if rebuild is success or error if not
    func rebuild(packages: [FFIPackage], outDir: URL = defaultSaveDirectory) async throws -> [Result<URL, Error>] {
        var ffiResults = slice_boxed_TwPackagesRebuildResult()
        withUnsafeMutablePointer(to: &ffiResults) { buildParameters.results = $0 }

        buildParameters.functions = createProgressFuncs()

        // Since Swift enums have values equal to FFI ones, it is safe to just pass them by without any checks
        buildParameters.preferences.compression_level = await .init(UInt32(preferences.compression.level.rawValue))
        buildParameters.preferences.compression_type = await .init(UInt32(preferences.compression.kind.rawValue))
        buildParameters.preferences.follow_symlinks = await preferences.followSymlinks

        outDir.path.utf8CString.withUnsafeBufferPointer { buffer in
            buildParameters.out_dir = UnsafePointer(strdup(buffer.baseAddress!))
        }
        defer { buildParameters.out_dir.deallocate() }

        let status = outDir.path.utf8CString.withUnsafeBufferPointer { pointer in
            // safe to unwrap?
            buildParameters.out_dir = pointer.baseAddress!

            return packages.map { $0.pkg }.withUnsafeBufferPointer { pointer in
                // safe to unwrap?
                buildParameters.packages = slice_ref_TwPackage_t(ptr: pointer.baseAddress!, len: pointer.count)

                return tw_rebuild_packages(innerDpkg, buildParameters)
            }
        }

        if status != TW_RESULT_OK {
            tw_free_rebuild_results(ffiResults)

            throw NSError(domain: "ru.danpashin.twackup", code: 0, userInfo: [
                NSLocalizedDescriptionKey: "FFI returned \(status) code. Critical bug?"
            ])
        }

        let results: [Result<URL, Error>] = UnsafeBufferPointer(start: ffiResults.ptr, count: ffiResults.len)
            .map { result in
                if !result.success {
                    return .failure(NSError(domain: "ru.danpashin.twackup", code: 0, userInfo: [
                        NSLocalizedDescriptionKey: "\(String(ffiSlice: result.error) ?? "")"
                    ]))
                }

                // safe to unwrap here 'cause Rust string is UTF-8 encoded string
                let path = String(ffiSlice: result.deb_path)!
                return .success(URL(fileURLWithPath: path))
            }

        tw_free_rebuild_results(ffiResults)

        return results
    }

    private func createProgressFuncs() -> TwProgressFunctions {
        var funcs = TwProgressFunctions()

        // not a memory leak actually. It lives as long as self does
        funcs.context = Unmanaged<Dpkg>.passUnretained(self).toOpaque()
        funcs.started_processing = { context, package in
            // Package is a stack pointer so it doesn't need to be released
            guard let context, let package, let ffiPackage = FFIPackage(package.pointee) else { return }

            let dpkg = Unmanaged<Dpkg>.fromOpaque(context).takeUnretainedValue()
            Task(priority: .utility) {
                await dpkg.buildProgressDelegate?.startProcessing(package: ffiPackage)
            }
        }
        funcs.finished_processing = { context, package, debPath in
            // Package is a stack pointer so it doesn't need to be released
            guard let context,
                  let package,
                  let ffiPackage = FFIPackage(package.pointee),
                  let debPath = String(ffiSlice: debPath)
            else { return }

            let dpkg = Unmanaged<Dpkg>.fromOpaque(context).takeUnretainedValue()
            Task(priority: .utility) {
                let debURL = URL(fileURLWithPath: debPath)
                await dpkg.buildProgressDelegate?.finishedProcessing(package: ffiPackage, debURL: debURL)
            }
        }
        funcs.finished_all = { context in
            guard let context else { return }
            let dpkg = Unmanaged<Dpkg>.fromOpaque(context).takeUnretainedValue()
            Task(priority: .utility) {
                await dpkg.buildProgressDelegate?.finishedAll()
            }
        }

        return funcs
    }
}

#if swift(>=6.0)
extension UnsafeMutablePointer<TwDpkg_t>: @unchecked @retroactive Sendable {}
#endif
