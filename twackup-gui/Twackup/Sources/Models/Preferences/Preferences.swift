//
//  Preferences.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

@MainActor
class Preferences: ObservableObject {
    @ProxiedObservedObject private(set) var compression = Compression()

    @AppStorage("should-follow-symlinks")
    private var pFollowSymlinks: Bool?

    var followSymlinks: Bool {
        get {
            pFollowSymlinks ?? FileManager.default.fileExists(atPath: "/var/jb/var/lib/dpkg")
        }
        set {
            pFollowSymlinks = newValue
        }
    }

    @AppStorage("saveDirectory")
    var saveDirectory: URL = defaultSaveDirectory

    var saveDiskNode: Int64 {
        get throws {
            let attrs = try FileManager.default.attributesOfItem(atPath: saveDirectory.path)
            return attrs[.systemNumber] as! Int64
        }
    }

    init() throws {
        _compression.setPublisher(objectWillChange)

        try createSaveDirectoryIfNeeded()
    }

    func createSaveDirectoryIfNeeded() throws {
        var isDir: ObjCBool = true
        var exists = FileManager.default.fileExists(atPath: saveDirectory.path, isDirectory: &isDir)

        // Application is not usable without directories created
        if exists && !isDir.boolValue {
            try FileManager.default.removeItem(at: saveDirectory)
            exists = false
        }

        if !exists {
            try FileManager.default.createDirectory(at: saveDirectory, withIntermediateDirectories: true)
        }
    }
}

extension Preferences {
    static let defaultSaveDirectory: URL = {
        let proxy = LSApplicationProxy.forCurrentProcess()
        let unsandboxed = (proxy?.entitlements["com.apple.private.security.no-sandbox"] as? Bool) ?? false
        if unsandboxed {
            return URL(fileURLWithPath: "/var/mobile/Documents/Twackup")
        }

        return FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
    }()
}
