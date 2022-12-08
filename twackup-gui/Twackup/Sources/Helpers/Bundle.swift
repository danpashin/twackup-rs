//
//  Bundle.swift
//  Twackup
//
//  Created by Daniil on 08.12.2022.
//

import Foundation

extension Bundle {
    class func appLocalize(_ key: String) -> String {
        Bundle.main.localizedString(forKey: key, value: nil, table: nil)
    }
}
