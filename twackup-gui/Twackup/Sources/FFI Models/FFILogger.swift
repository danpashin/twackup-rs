//
//  Logger.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

protocol FFILoggerSubscriber: NSObjectProtocol, Sendable {
    func log(message: FFILogger.Message, level: FFILogger.Level) async

    func flush() async
}

actor FFILogger {
    @objc
    enum Level: UInt32 {
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
                  let level = Level(rawValue: level.rawValue) else {
                return
            }

            let logger = Unmanaged<FFILogger>.fromOpaque(context).takeUnretainedValue()

            Task(priority: .utility) {
                await logger.log(message: Message(text: msgText, target: msgTarget), level: level)
            }
        }

        tw_enable_logging(funcs, .init(UInt32(level.rawValue)))
    }

    func log(message: Message, level: Level) {
        for subscriber in subscribers {
            Task {
                await subscriber.log(message: message, level: level)
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
