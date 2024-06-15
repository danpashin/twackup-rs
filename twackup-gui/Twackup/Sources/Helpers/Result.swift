//
//  Result.swift
//  Twackup
//
//  Created by Daniil on 15.06.2024.
//

extension Result {
    var isSuccess: Bool {
        return value != nil
    }

    var value: Success? {
        switch self {
        case let .success(value):
            return value

        case .failure:
            return nil
        }
    }
}
