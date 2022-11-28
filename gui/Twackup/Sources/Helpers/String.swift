//
//  String.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

extension String {
    init?(ffiSlice slice: slice_raw_uint8_t) {
        if slice.ptr == nil || slice.len == 0 {
            return nil
        }

        if let string = String(bytesNoCopy: slice.ptr, length: slice.len, encoding: .utf8, freeWhenDone: false) {
            self = string
        } else {
            return nil
        }
    }

    func truncate(_ length: Int, trailing: String = "...") -> String {
        return (count > length) ? prefix(length) + trailing : self
    }
}
