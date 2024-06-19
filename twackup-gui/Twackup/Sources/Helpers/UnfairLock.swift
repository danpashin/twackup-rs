//
//  UnfairLock.swift
//  Twackup
//
//  Created by Daniil on 16.06.2024.
//

final class UnfairLock<Value> {
    private let lock = UnsafeMutablePointer<os_unfair_lock>.allocate(capacity: 1)

    private var _value: Value

    var value: Value {
        get { whileLocked { _value } }
        set { whileLocked { _value = newValue } }
    }

    init(value: Value) {
        _value = value
        lock.initialize(to: .init())
    }

    deinit {
        lock.deinitialize(count: 1)
        lock.deallocate()
    }

    func whileLocked<T>(_ action: () -> T) -> T {
        os_unfair_lock_lock(lock)
        defer { os_unfair_lock_unlock(lock) }
        return action()
    }
}

@propertyWrapper
struct UnfairLockWrap<Value> {
    private let lock: UnfairLock<Value>

    var wrappedValue: Value {
        get { lock.value }
        set { lock.value = newValue }
    }

    init(wrappedValue: Value) {
        lock = UnfairLock(value: wrappedValue)
    }
}
