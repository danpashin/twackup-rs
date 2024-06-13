//
//  SwiftConversions.swift
//  Twackup
//
//  Created by Daniil on 13.06.2024.
//

extension TwPackagesSort {
    var clampedToU8: TwPackagesSort_t {
        return TwPackagesSort_t(self.rawValue)
    }
}

extension TwPackageSection {
    var clampedToU8: TwPackageSection_t {
        return TwPackageSection_t(self.rawValue)
    }
}

extension TwPackageField {
    var clampedToU8: TwPackageField_t {
        return TwPackageField_t(self.rawValue)
    }
}

extension TwResult {
    var clampedToU8: TwResult_t {
        return TwResult_t(self.rawValue)
    }
}
