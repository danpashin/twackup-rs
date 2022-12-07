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

    let humanDescription: String?

    let installedSize: Int64?

    let architecture: String?

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

        let installedSizeField = TwPackageField_t(TW_PACKAGE_FIELD_INSTALLED_SIZE)
        if let size = String(ffiSlice: pkg.get_field(pkg.inner_ptr, installedSizeField)) {
            installedSize = Int64(size)
        } else {
            installedSize = nil
        }

        let descriptionField = TwPackageField_t(TW_PACKAGE_FIELD_DESCRIPTION)
        humanDescription = String(ffiSlice: pkg.get_field(pkg.inner_ptr, descriptionField))

        let archField = TwPackageField_t(TW_PACKAGE_FIELD_ARCHITECTURE)
        architecture = String(ffiSlice: pkg.get_field(pkg.inner_ptr, archField))
    }

    deinit {
        pkg.deallocate(pkg.inner_ptr)
    }

    func humanInstalledSize() -> String {
        return ByteCountFormatter.string(fromByteCount: installedSize ?? 0 * 1000, countStyle: .decimal)
    }
}
