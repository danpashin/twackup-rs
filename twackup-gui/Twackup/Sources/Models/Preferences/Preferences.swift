//
//  Preferences.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

class Preferences: ObservableObject {
    static let `default` = Preferences()

    @ProxiedObservedObject private(set) var compression = Compression()

    init() {
        _compression.setPublisher(objectWillChange)
    }
}
