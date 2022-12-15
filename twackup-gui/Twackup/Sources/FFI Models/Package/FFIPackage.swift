//
//  Package.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

class FFIPackage: Package {
    let pkg: TwPackage_t

    let id: String

    let name: String

    let version: String

    let section: PackageSection

    let icon: URL?

    let depiction: URL?

    var humanDescription: String? {
        String(ffiSlice: pkg.get_field(pkg.inner_ptr, TwPackageField_t(TW_PACKAGE_FIELD_DESCRIPTION)))
    }

    var architecture: String? {
        String(ffiSlice: pkg.get_field(pkg.inner_ptr, TwPackageField_t(TW_PACKAGE_FIELD_ARCHITECTURE)))
    }

    var installedSize: Int64 {
        let field = TwPackageField_t(TW_PACKAGE_FIELD_INSTALLED_SIZE)
        guard let stringSize = String(ffiSlice: pkg.get_field(pkg.inner_ptr, field)) else { return 0 }
        guard let size = Int64(stringSize) else { return 0 }
        return size * 1_000
    }

    var debSize: Int64 = 0

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
        section = PackageSection(pkg.section)

        let iconField = TwPackageField_t(TW_PACKAGE_FIELD_ICON)
        if let iconString = String(ffiSlice: pkg.get_field(pkg.inner_ptr, iconField)) {
            icon = URL(string: iconString)
        } else {
            icon = nil
        }

        let depictionField = TwPackageField_t(TW_PACKAGE_FIELD_DEPICTION)
        if let depict = String(ffiSlice: pkg.get_field(pkg.inner_ptr, depictionField)) {
            depiction = URL(string: depict)
        } else {
            depiction = nil
        }
    }

    deinit {
        pkg.deallocate(pkg.inner_ptr)
    }

    func isEqualTo(_ other: Package) -> Bool {
        id == other.id && version == other.version
    }
}
