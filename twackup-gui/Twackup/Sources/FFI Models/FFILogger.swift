//
//  Logger.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import OSLog

protocol FFILoggerSubscriber: Hashable, Sendable {
    func log(message: FFILogger.Message, level: FFILogger.Level) async

    func flush() async
}

actor FFILogger {
    enum Level: UInt8 {
        case off
        case error
        case warning
        case info
        case debug

        var osLogType: OSLogType {
            switch self {
            case .off: .fault
            case .error: .error
            case .warning: .default
            case .info: .info
            case .debug: .debug
            }
        }
    }

    struct Message {
        let text: String

        var target: String?
    }

    struct InitError: Error {
        init() {
        }
    }

    // Force try is intentional here
    // Rust `log` crate supports setting only one logger for all program lifetime
    // So if `FFLogger` will be called multiple times, `log` will panic
    static let shared = try! FFILogger(level: .debug) // swiftlint:disable:this force_try

    private var subscribers: Set<AnyHashable> = []

    init(level: Level) throws {
        if tw_is_logging_enabled() {
            throw InitError()
        }

        var funcs = TwLogFunctions()
        funcs.context = Unmanaged<FFILogger>.passRetained(self).toOpaque()
        funcs.log = { context, ffiMsg, level in
            guard let context,
                  let msgText = String(ffiSlice: ffiMsg.text, deallocate: true),
                  let msgTarget = String(ffiSlice: ffiMsg.target, deallocate: true),
                  let level = Level(rawValue: level) else {
                return
            }

            let logger = Unmanaged<FFILogger>.fromOpaque(context).takeUnretainedValue()

            Task(priority: .utility) {
                await logger.log(message: Message(text: msgText, target: msgTarget), level: level)
            }
        }
        funcs.flush = { context  in
            guard let context else { return }

            let logger = Unmanaged<FFILogger>.fromOpaque(context).takeUnretainedValue()
            Task(priority: .utility) {
                await logger.flush()
            }
        }

        tw_enable_logging(funcs, .init(UInt32(level.rawValue)))
    }

    func log(message: Message, level: Level) {
        subscribers
            // Since subscriber can only be added via addSubscriber() it must conform to FFILoggerSubscriber
            // So force casting is safe here
            .map { $0 as! any FFILoggerSubscriber } // swiftlint:disable:this force_cast
            .forEach { subscriber in
                Task {
                    await subscriber.log(message: message, level: level)
                }
            }
    }

    func log(_ text: String, level: Level = .info) {
        log(message: Message(text: text), level: level)
    }

    func addSubscriber(_ subscriber: any FFILoggerSubscriber) {
        subscribers.insert(AnyHashable(subscriber))
    }

    func removeSubscriber(_ subscriber: any FFILoggerSubscriber) {
        subscribers.remove(subscriber)
    }

    private func flush() {
        subscribers
            // Since subscriber can only be added via addSubscriber() it must conform to FFILoggerSubscriber
            // So force casting is safe here
            .map { $0 as! any FFILoggerSubscriber } // swiftlint:disable:this force_cast
            .forEach { subscriber in
                Task {
                    await subscriber.flush()
                }
            }
    }
}
