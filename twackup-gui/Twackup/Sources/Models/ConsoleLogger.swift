//
//  ConsoleLogger.swift
//  Twackup
//
//  Created by Daniil on 11.06.2024.
//

@preconcurrency import OSLog

final class ConsoleLogger: Hashable, FFILoggerSubscriber, Sendable {
    private let uuid = UUID()

    private let innerLog = OSLog(subsystem: "Twackup", category: "FFI")

    func log(message: FFILogger.Message, level: FFILogger.Level) async {
        os_log(
            level.osLogType,
            log: innerLog,
            "[%{public}@]: %{public}@",
            (message.target ?? "Unknown") as CVarArg,
            message.text
        )
    }

    func flush() async {
    }

    static func == (lhs: ConsoleLogger, rhs: ConsoleLogger) -> Bool {
        lhs.uuid == rhs.uuid
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(uuid)
    }
}
