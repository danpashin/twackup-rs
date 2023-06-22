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

    var section: PackageSection {
        return pkg.section.swiftSection
    }

    private(set) lazy var icon: URL? = {
        guard let icon = String(ffiSlice: pkg.get_field(pkg.inner_ptr, TW_PACKAGE_FIELD_ICON)) else { return nil }

        return URL(string: icon)
    }()

    private(set) lazy var depiction: URL? = {
        var depiction = String(ffiSlice: pkg.get_field(pkg.inner_ptr, TW_PACKAGE_FIELD_DEPICTION))
        if depiction == nil {
            depiction = String(ffiSlice: pkg.get_field(pkg.inner_ptr, TW_PACKAGE_FIELD_HOMEPAGE))
        }

        guard let depiction else { return nil }
        return URL(string: depiction)
    }()

    private(set) lazy var humanDescription: String? = {
        String(ffiSlice: pkg.get_field(pkg.inner_ptr, TW_PACKAGE_FIELD_DESCRIPTION))
    }()

    private(set) lazy var architecture: String? = {
        String(ffiSlice: pkg.get_field(pkg.inner_ptr, TW_PACKAGE_FIELD_ARCHITECTURE))
    }()

    private(set) lazy var installedSize: Int64 = {
        let field = TW_PACKAGE_FIELD_INSTALLED_SIZE
        guard let stringSize = String(ffiSlice: pkg.get_field(pkg.inner_ptr, field)) else { return 0 }
        guard let size = Int64(stringSize) else { return 0 }
        return size * 1_000
    }()

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
        pkg.deallocate(pkg.inner_ptr)
    }

    func isEqualTo(_ other: Package) -> Bool {
        id == other.id && version == other.version
    }
}
