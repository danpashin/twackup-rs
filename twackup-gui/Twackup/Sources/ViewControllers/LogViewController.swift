//
//  LogViewController.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import DZNEmptyDataSet

final class LogViewController: UIViewController, FFILoggerSubscriber {
    let metadata: ViewControllerMetadata

    let mainModel: MainModel

    let currentText = NSMutableAttributedString()

    private lazy var logQueue = DispatchQueue(label: "twackup-log", qos: .default)

    private var wantsToScrollBottom: Bool = false

    private(set) lazy var logView: UITextView = {
        let view = UITextView()
        view.isScrollEnabled = true
        view.isEditable = false
        view.alwaysBounceVertical = true

        view.emptyDataSetSource = self
        view.emptyDataSetDelegate = self

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

        mainModel.logger.addSubsriber(self)
        tabBarItem = metadata.tabbarItem
    }

    deinit {
        mainModel.logger.removeSubscriber(self)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func loadView() {
        self.view = logView
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        navigationItem.title = metadata.navTitle
        navigationItem.rightBarButtonItem = clearLogButton
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)

        logView.attributedText = currentText
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)

        logView.reloadEmptyDataSet()
        scrollToBottomIfNeeded()
    }

    private func scrollToBottomIfNeeded() {
        if wantsToScrollBottom {
            logView.contentOffset = CGPoint(x: 0, y: logView.contentSize.height)
            wantsToScrollBottom = false
        }
    }

    @objc
    func actionClearLog() {
        currentText.setAttributedString(NSAttributedString())
        logView.text = ""

        logView.contentOffset = .zero
        logView.reloadEmptyDataSet()
    }

    // MARK: - FFILoggerSubscriber

    func log(message: FFILogger.Message, level: FFILogger.Level) {
        // This queue is serial, so it will put all messages in real `queue`
        logQueue.async { [self] in
            var targetColor: UIColor
            switch level {
            case .off: targetColor = .clear
            case .debug: targetColor = .systemIndigo
            case .info: targetColor = .systemBlue
            case .warning: targetColor = .systemOrange
            case .error: targetColor = .systemRed
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
    }

    func flush() {
    }
}
