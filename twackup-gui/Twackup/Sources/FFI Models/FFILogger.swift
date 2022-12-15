//
//  Logger.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import Foundation

protocol FFILoggerSubscriber: NSObjectProtocol {
    func log(message: FFILogger.Message, level: FFILogger.Level)

    func flush()
}

class FFILogger {
    @objc
    enum Level: UInt8 {
        case off
        case error
        case warning
        case info
        case debug
    }

    struct Message {
        let text: String

        var target: String?
    }

    static let shared = FFILogger(level: .debug)

    private var subscribers: [FFILoggerSubscriber] = []

    init(level: Level) {
        var funcs = TwLogFunctions()
        funcs.context = Unmanaged<FFILogger>.passUnretained(self).toOpaque()
        funcs.log = { context, ffiMsg, level in
            guard let context,
                  let msgText = String(ffiSlice: ffiMsg.text, deallocate: true),
                  let msgTarget = String(ffiSlice: ffiMsg.target, deallocate: true),
                  let level = Level(rawValue: level) else {
                return
            }

            let logger = Unmanaged<FFILogger>.fromOpaque(context).takeUnretainedValue()
            logger.log(message: Message(text: msgText, target: msgTarget), level: level)
        }

        tw_enable_logging(funcs, level.rawValue)
    }

    func log(message: Message, level: Level) {
        DispatchQueue.global().async { [self] in
            for subscriber in subscribers {
                subscriber.log(message: message, level: level)
            }
        }
    }

    func log(_ text: String, level: Level = .info) {
        log(message: Message(text: text), level: level)
    }

    func addSubsriber(_ subscriber: FFILoggerSubscriber) {
        subscribers.append(subscriber)
    }

    func removeSubscriber(_ subscriber: FFILoggerSubscriber) {
        subscribers.removeAll { existing in
            existing.isEqual(subscriber)
        }
    }
}
