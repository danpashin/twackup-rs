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
    @objc enum Level: UInt8 {
        case off
        case debug
        case info
        case warning
        case error
    }

    struct Message {
        let text: String

        let target: String
    }

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

            let message = Message(text: msgText, target: msgTarget)

            DispatchQueue.global().async {
                let logger = Unmanaged<FFILogger>.fromOpaque(context).takeUnretainedValue()
                for subscriber in logger.subscribers {
                    subscriber.log(message: message, level: level)
                }
            }
        }

        tw_enable_logging(funcs, level.rawValue)
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
