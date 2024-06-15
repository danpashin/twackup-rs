//
//  PackageParser.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

import Sentry

enum DpkgError: Error, LocalizedError {
    case `internal`

    var errorDescription: String? {
        switch self {
        case .internal: "Internal error. Needs more details"
        }
    }
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

    private let innerDpkg: UnsafeMutablePointer<TwDpkg_t>

    private let preferences: Preferences

    private var buildParameters = TwBuildParameters_t()

    nonisolated let progressNotifier = DpkgProgressNotifier()

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
        var packagesPtr: UnsafeMutablePointer<TwPackage>?
        let count = tw_get_packages(innerDpkg, onlyLeaves, TW_PACKAGES_SORT_NAME.clampedToU8, &packagesPtr)
        if count == -1 {
            throw DpkgError.internal
        }

        let buffer = UnsafeBufferPointer(start: packagesPtr, count: Int(count))
        defer { tw_free_packages(packagesPtr, count) }

        return buffer.compactMap { package in
            let model = FFIPackage(package)
            if model == nil {
                tw_package_release(package.inner)
            }

            return model
        }
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

        buildParameters.functions = progressNotifier.ffiFunctions

        // Since Swift enums have values equal to FFI ones, it is safe to just pass them by without any checks
        buildParameters.preferences.compression_level = await .init(UInt32(preferences.compression.level.rawValue))
        buildParameters.preferences.compression_type = await .init(UInt32(preferences.compression.kind.rawValue))
        buildParameters.preferences.follow_symlinks = await preferences.followSymlinks

        outDir.path.utf8CString.withUnsafeBufferPointer { buffer in
            buildParameters.out_dir = UnsafePointer(strdup(buffer.baseAddress!))
        }
        defer { buildParameters.out_dir.deallocate() }

        let status = packages
            .map { $0.inner }
            .map { Optional($0) } // Apple moment
            .withUnsafeBufferPointer { pointer in
                // safe to unwrap?
                buildParameters.packages = slice_ref_TwPackageRef_t(ptr: pointer.baseAddress!, len: pointer.count)

                return tw_rebuild_packages(innerDpkg, buildParameters)
            }

        if status != TW_RESULT_OK.clampedToU8 {
            tw_free_rebuild_results(ffiResults)

            throw NSError(domain: "ru.danpashin.twackup", code: 0, userInfo: [
                NSLocalizedDescriptionKey: "FFI returned \(status) code. Critical bug?"
            ])
        }

        let results: [Result<URL, Error>] = UnsafeBufferPointer(start: ffiResults.ptr, count: ffiResults.len)
            .map { result in
                // package is not used yet. When it will be - should call `tw_package_retain` on it
                if !result.success {
                    return .failure(NSError(domain: "ru.danpashin.twackup", code: 0, userInfo: [
                        NSLocalizedDescriptionKey: "\(String(ffiSlice: result.error) ?? "")"
                    ]))
                }

                // safe to unwrap here 'cause Rust string is UTF-8 encoded string
                let path = String(ffiSlice: result.deb_path)!

                // URL will copy string
                return .success(URL(fileURLWithPath: path))
            }

        tw_free_rebuild_results(ffiResults)

        return results
    }
}

#if swift(>=6.0)
extension UnsafeMutablePointer<TwDpkg_t>: @unchecked @retroactive Sendable {}
#endif
