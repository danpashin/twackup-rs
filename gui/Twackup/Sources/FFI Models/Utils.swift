//
//  Utils.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import Foundation

extension String {
    init?(ffiSlice slice: slice_raw_uint8_t) {
        if let string = String(bytesNoCopy: slice.ptr, length: slice.len, encoding: .utf8, freeWhenDone: false) {
            self = string
        } else {
            return nil
        }
    }
}
