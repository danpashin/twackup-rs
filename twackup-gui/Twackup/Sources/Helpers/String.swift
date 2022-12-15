//
//  String.swift
//  Twackup
//
//  Created by Daniil on 28.11.2022.
//

extension String {
    var localized: String { Bundle.appLocalize(self) }

    init?(ffiSlice slice: slice_raw_uint8_t, deallocate: Bool = false) {
        if slice.ptr == nil || slice.len == 0 {
            return nil
        }

        if let string = String(bytesNoCopy: slice.ptr, length: slice.len, encoding: .utf8, freeWhenDone: deallocate) {
            self = string
        } else {
            return nil
        }
    }

    init?(ffiSlice slice: slice_boxed_uint8_t, deallocate: Bool = false) {
        self.init(ffiSlice: slice_raw_uint8_t(ptr: slice.ptr, len: slice.len), deallocate: deallocate)
    }

    func truncate(_ length: Int, trailing: String = "...") -> String {
        return (count > length) ? prefix(length) + trailing : self
    }

    func deletePrefix(_ prefix: String) -> String {
        guard hasPrefix(prefix) else { return self }
        return String(dropFirst(prefix.count))
    }
}
