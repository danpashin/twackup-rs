//
//  ProxiedObservedObject.swift
//  Twackup
//
//  Created by Daniil on 14.12.2022.
//

import Combine
import SwiftUI

@propertyWrapper
struct ProxiedObservedObject<Value>
where Value: ObservableObject, Value.ObjectWillChangePublisher == ObservableObjectPublisher {
    private class Proxy<T: ObservableObject>
    where T.ObjectWillChangePublisher == ObservableObjectPublisher {
        var subscriber: AnyCancellable?

        var target: ObservableObjectPublisher?

        init(proxySource: T) {
            subscriber = proxySource.objectWillChange.sink { [weak self] _ in
                self?.target?.send()
            }
        }
    }

    @ObservedObject
    var wrappedValue: Value

    var projectedValue: ObservedObject<Value>.Wrapper {
        $wrappedValue
    }

    private let proxy: Proxy<Value>

    init(wrappedValue: Value) {
        self.wrappedValue = wrappedValue

        proxy = Proxy(proxySource: wrappedValue)
    }

    func setPublisher(_ publisher: ObservableObjectPublisher?) {
        proxy.target = publisher
    }
}
