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

    init() {
        _compression.setPublisher(objectWillChange)
    }
}
