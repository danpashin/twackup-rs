//
//  LogViewController.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import BlankSlate
import StyledTextKit

final class LogViewController: UIViewController, FFILoggerSubscriber, ScrollableViewController {
    let metadata: ViewControllerMetadata

    let mainModel: MainModel

    let currentText = NSMutableAttributedString()

    private var wantsToScrollBottom: Bool = false

    private(set) lazy var logView: StyledTextView = {
        let view = StyledTextView()

        return view
    }()

    private(set) lazy var scrollView: UIScrollView = {
        let view = UIScrollView()
        view.isScrollEnabled = true
        view.alwaysBounceVertical = true
        view.addSubview(logView)

        view.bs.setDataSourceAndDelegate(self)

        return view
    }()

    private(set) lazy var clearLogButton: UIBarButtonItem = {
        let title = "log-clear-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionClearLog))
    }()

    init(mainModel: MainModel, metadata: ViewControllerMetadata) {
        self.mainModel = mainModel
        self.metadata = metadata
        super.init(nibName: nil, bundle: nil)
        tabBarItem = metadata.tabbarItem

        Task {
            await FFILogger.shared.addSubscriber(self)
        }
    }

    deinit {
        Task { [self] in
            await FFILogger.shared.removeSubscriber(self)
        }
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func loadView() {
        self.view = scrollView
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        navigationItem.title = metadata.navTitle
        navigationItem.rightBarButtonItem = clearLogButton
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)

        renderLog()
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)

        scrollView.bs.reloadBlankSlate()
        scrollToBottomIfNeeded()
    }

    private func renderLog() {
        let category = UIApplication.shared.preferredContentSizeCategory
        let builder = StyledTextBuilder(attributedText: currentText)
        let renderer = StyledTextRenderer(string: builder.build(), contentSizeCategory: category)
        logView.configure(with: renderer, width: view.bounds.width)

        scrollView.contentSize = logView.bounds.size
    }

    private func scrollToBottomIfNeeded() {
        if wantsToScrollBottom {
            scrollView.contentOffset = scrollView.maximumContentOffset
            wantsToScrollBottom = false
        }
    }

    @objc
    func actionClearLog() {
        currentText.setAttributedString(NSAttributedString())
        renderLog()

        scrollView.contentOffset = scrollView.minimumContentOffset
        scrollView.bs.reloadBlankSlate()
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

        currentText.append(NSAttributedString(string: "[\(message.target ?? "nil")]  ", attributes: [
            .font: UIFont.boldSystemFont(ofSize: UIFont.systemFontSize),
            .foregroundColor: targetColor
        ]))

        currentText.append(NSAttributedString(string: message.text, attributes: [
            .font: UIFont.monospacedSystemFont(ofSize: UIFont.systemFontSize, weight: .regular),
            .foregroundColor: UIColor.label
        ]))

        currentText.append(NSAttributedString(string: "\n"))

        wantsToScrollBottom = true
    }

    func flush() async {
    }

    // MARK: - ScrollableViewController

    func scrollToInitialPosition(animated: Bool) {
        wantsToScrollBottom = true
        scrollToBottomIfNeeded()
    }
}
