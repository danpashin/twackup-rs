//
//  Package.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

final class FFIPackage: Package, Sendable {
    static func == (lhs: FFIPackage, rhs: FFIPackage) -> Bool {
        lhs.id == rhs.id && lhs.version == rhs.version
    }

    private let pkg: TwPackage_t

    let id: String

    let name: String

    let version: String

    var section: PackageSection {
        pkg.section.swiftSection
    }

    var inner: TwPackageRef_t {
        pkg.inner
    }

    var icon: URL? {
        let fieldString = tw_package_field_str(pkg.inner, TW_PACKAGE_FIELD_ICON.clampedToU8)
        guard let iconURL = String(ffiSlice: fieldString, deallocate: true) else {
            return nil
        }

        return URL(string: iconURL)
    }

    var depiction: URL? {
        var depiction = String(ffiSlice: tw_package_field_str(pkg.inner, TW_PACKAGE_FIELD_DEPICTION.clampedToU8))
        if depiction == nil {
            depiction = String(ffiSlice: tw_package_field_str(pkg.inner, TW_PACKAGE_FIELD_HOMEPAGE.clampedToU8))
        }

        guard let depiction else { return nil }
        return URL(string: depiction)
    }

    var humanDescription: String? {
        String(ffiSlice: tw_package_field_str(pkg.inner, TW_PACKAGE_FIELD_DESCRIPTION.clampedToU8))
    }

    var architecture: String? {
        String(ffiSlice: tw_package_field_str(pkg.inner, TW_PACKAGE_FIELD_ARCHITECTURE.clampedToU8))
    }

    var installedSize: Int64 {
        let field = TW_PACKAGE_FIELD_INSTALLED_SIZE.clampedToU8
        guard let stringSize = String(ffiSlice: tw_package_field_str(pkg.inner, field)) else { return 0 }
        guard let size = Int64(stringSize) else { return 0 }
        return size * 1_000
    }

    init?(_ pkg: TwPackage_t) {
        self.pkg = pkg

        // safe to unwrap here 'cause Rust string is UTF-8 encoded
        // and never will be nullable
        id = String(ffiSlice: pkg.identifier)!
        if id.hasPrefix("gsc") || id.hasPrefix("cy+") {
            return nil
        }

        // Here unwrap is safe too
        name = String(ffiSlice: pkg.name)!
        version = String(ffiSlice: pkg.version)!
    }

    deinit {
        tw_package_release(pkg.inner)
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(id)
        hasher.combine(version)
    }
}

extension TwPackage_t: @unchecked Sendable {}
