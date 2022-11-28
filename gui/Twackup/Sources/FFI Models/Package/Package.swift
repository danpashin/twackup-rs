//
//  Package.swift
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

import Foundation

class FFIPackage: Package {

    let pkg: TwPackage_t

    let id: String

    let name: String

    let version: String

    let section: PackageSection

    let icon: URL?

    let depiction: URL?

    let description: String

    let installedSize: Int64

    let architecture: String

    init?(_ pkg: TwPackage_t) {
        self.pkg = pkg
        
        id = String(ffiSlice: pkg.identifier) ?? "FFI ERROR"
        if (id.hasPrefix("gsc") || id.hasPrefix("cy+")) {
            pkg.inner_ptr.deallocate()
            return nil
        }

        name = String(ffiSlice: pkg.name) ?? "FFI ERROR"
        version = String(ffiSlice: pkg.version) ?? "FFI ERROR"
        section = PackageSection(pkg.section)

        let iconField = TwPackageField_t(TW_PACKAGE_FIELD_ICON)
        if let iconString = String(ffiSlice: pkg.get_field(pkg.inner_ptr, iconField)) {
            icon = URL(string: iconString)
        } else {
            icon = nil
        }

        let depictionField = TwPackageField_t(TW_PACKAGE_FIELD_DEPICTION)
        if let _depiction = String(ffiSlice: pkg.get_field(pkg.inner_ptr, depictionField)) {
            depiction = URL(string: _depiction)
        } else {
            depiction = nil
        }

        let installedSizeField = TwPackageField_t(TW_PACKAGE_FIELD_INSTALLED_SIZE)
        installedSize = Int64(String(ffiSlice: pkg.get_field(pkg.inner_ptr, installedSizeField)) ?? "") ?? 0

        let descriptionField = TwPackageField_t(TW_PACKAGE_FIELD_DESCRIPTION)
        description = String(ffiSlice: pkg.get_field(pkg.inner_ptr, descriptionField)) ?? ""

        let archField = TwPackageField_t(TW_PACKAGE_FIELD_ARCHITECTURE)
        architecture = String(ffiSlice: pkg.get_field(pkg.inner_ptr, archField)) ?? ""
    }

    deinit {
        pkg.inner_ptr.deallocate()
    }

    func humanInstalledSize() -> String {
        return ByteCountFormatter.string(fromByteCount: installedSize * 1000, countStyle: .decimal)
    }
}
