//
//  StyledLogger.swift
//  Twackup
//
//  Created by Daniil on 19.06.2024.
//

import StyledTextKit

actor StyledLogger: FFILoggerSubscriber {
    struct RenderOptions {
        let boldFont: UIFont

        let monospacedFont: UIFont

        let color: UIColor
    }

    struct RenderIssue: Error {
        init() {
        }
    }

    struct RenderResult {
        let image: CGImage

        let size: CGSize

        let scale: CGFloat

        let insets: UIEdgeInsets
    }

    private let uuid = UUID()

    private let builder = StyledTextBuilder(text: "")

    private var cachedRenderer: StyledTextRenderer?

    private(set) var wantsRerender = false

    let renderOptions: RenderOptions = {
        let systemSize = UIFont.systemFontSize
        let bold = UIFont.boldSystemFont(ofSize: systemSize)
        let monospaced = UIFont.monospacedSystemFont(ofSize: systemSize, weight: .regular)
        return RenderOptions(boldFont: bold, monospacedFont: monospaced, color: .label)
    }()

    init() {
        Task {
            await FFILogger.shared.addSubscriber(self)
        }
    }

    deinit {
        Task {
            await FFILogger.shared.removeSubscriber(self)
        }
    }

    func render(for width: CGFloat, contentSizeCategory: UIContentSizeCategory) throws -> RenderResult {
        if wantsRerender {
            wantsRerender = false
            cachedRenderer = nil
        }

        if let cachedRenderer {
            let cachedResult = cachedRenderer.cachedRender(for: width)
            if let cachedImage = cachedResult.image, let cachedSize = cachedResult.size {
                return RenderResult(
                    image: cachedImage,
                    size: cachedSize,
                    scale: cachedRenderer.scale,
                    insets: cachedRenderer.inset
                )
            }
        }

        let renderer = StyledTextRenderer(string: builder.build(), contentSizeCategory: contentSizeCategory)
        cachedRenderer = renderer

        let (image, size) = renderer.render(for: width)
        guard let image else {
            throw RenderIssue()
        }

        return RenderResult(
            image: image,
            size: size,
            scale: renderer.scale,
            insets: renderer.inset
        )
    }

    func render(for width: CGFloat, contentSizeCategory: UIContentSizeCategory) async throws -> RenderResult {
        try await withCheckedThrowingContinuation { continuation in
            continuation.resume(with: Result {
                try render(for: width, contentSizeCategory: contentSizeCategory)
            })
        }
    }

    // MARK: - FFILoggerSubscriber

    func log(message: FFILogger.Message, level: FFILogger.Level) async {
        let targetColor: UIColor = switch level {
        case .off: .clear
        case .debug: .systemIndigo
        case .info: .systemBlue
        case .warning: .systemOrange
        case .error: .systemRed
        }

        builder.add(text: "[\(message.target ?? "nil")]  ", attributes: [
            .font: renderOptions.boldFont,
            .foregroundColor: targetColor
        ])

        builder.add(text: message.text, attributes: [
            .font: renderOptions.monospacedFont,
            .foregroundColor: renderOptions.color
        ])

        builder.add(text: "\n")

        wantsRerender = true
    }

    func flush() async {
        builder.clearText()
        wantsRerender = true
    }

    // MARK: - Hashable

    static func == (lhs: StyledLogger, rhs: StyledLogger) -> Bool {
        lhs.uuid == rhs.uuid
    }

    nonisolated func hash(into hasher: inout Hasher) {
        hasher.combine(uuid)
    }
}
