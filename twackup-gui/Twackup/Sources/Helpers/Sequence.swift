//
//  Sequence.swift
//  Twackup
//
//  Created by Daniil on 16.06.2024.
//

extension Sequence {
    func sorted<T: Comparable>(by keyPath: KeyPath<Element, T>) -> [Element] {
        sorted { $0[keyPath: keyPath] < $1[keyPath: keyPath] }
    }
}
