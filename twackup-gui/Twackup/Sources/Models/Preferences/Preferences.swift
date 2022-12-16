//
//  Preferences.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

class Preferences: ObservableObject {
    static let `default` = Preferences()

    @ObservedObjectProxy
    private(set) var compression: Compression = Compression()

    init() {
        _compression.setPublisher(objectWillChange)
    }
}
