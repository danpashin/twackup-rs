//
//  Jailbreaks.swift
//  Twackup
//
//  Created by Daniil on 12.06.2024.
//

func jbRootPath(_ cPath: UnsafePointer<CChar>?) -> String {
    guard let resolved = libroot_dyn_jbrootpath(cPath, nil) else { return "" }
    let result = String(cString: resolved)
    free(resolved)

    return result
}

func jbRootPath<S: StringProtocol>(_ path: S) -> String {
    path.withCString { jbRootPath($0) }
}
