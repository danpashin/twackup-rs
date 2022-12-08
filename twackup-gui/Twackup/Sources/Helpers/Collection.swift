//
//  Collection.swift
//  Twackup
//
//  Created by Daniil on 05.12.2022.
//

class UnsafeCollection<T>: Collection {

    typealias Index = Int
    typealias Element = T
    typealias Iterator = UnsafeBufferPointer<T>.Iterator

    var startIndex: Int { pointer.startIndex }

    var endIndex: Int { pointer.endIndex }

    fileprivate let pointer: UnsafeBufferPointer<T>

    init(buffer pointer: UnsafeBufferPointer<T>) {
        self.pointer = pointer
    }

    init(raw: UnsafePointer<T>, length: Int) {
        pointer = UnsafeBufferPointer(start: raw, count: length)
    }

    subscript(position: Int) -> T {
        pointer[position]
    }

    func index(after idx: Int) -> Int {
        pointer.index(after: idx)
    }

    func makeIterator() -> UnsafeBufferPointer<T>.Iterator {
        pointer.makeIterator()
    }
}

class UnsafeConsumingCollection<T>: UnsafeCollection<T> {
    deinit {
        pointer.deallocate()
    }
}
