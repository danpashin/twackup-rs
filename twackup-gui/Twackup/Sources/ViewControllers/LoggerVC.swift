//
//  LoggerVC.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import DZNEmptyDataSet

class LoggerViewController: UIViewController, FFILoggerSubscriber, DZNEmptyDataSetSource, DZNEmptyDataSetDelegate {
    let metadata: ViewControllerMetadata

    let mainModel: MainModel

    private lazy var logTextView: UITextView = {
        let view = UITextView()
        view.isScrollEnabled = true
        view.isEditable = false
        view.alwaysBounceVertical = true

        view.emptyDataSetSource = self
        view.emptyDataSetDelegate = self

        return view
    }()

    private var logTextViewAppeared: Bool = false

    private lazy var clearLogButton: UIBarButtonItem = {
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
        self.view = logTextView
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        navigationItem.title = metadata.navTitle
        navigationItem.rightBarButtonItem = clearLogButton
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)

        if !logTextViewAppeared {
            logTextViewAppeared = true

            logTextView.reloadEmptyDataSet()
        }
    }

    private func insert(attributedString: NSAttributedString) {
        guard let selectedRange = logTextView.selectedTextRange else { return }

        let cursorIndex = logTextView.offset(from: logTextView.beginningOfDocument, to: selectedRange.start)
        let mutableAttributedText = NSMutableAttributedString(attributedString: logTextView.attributedText)
        mutableAttributedText.insert(attributedString, at: cursorIndex)
        logTextView.attributedText = mutableAttributedText
    }

    private func scrollToBottom() {
        let textCount: Int = logTextView.text.count
        guard textCount >= 1 else { return }
        logTextView.scrollRangeToVisible(NSRange(location: textCount - 1, length: 1))
    }

    func log(message: FFILogger.Message, level: FFILogger.Level) {
        DispatchQueue.main.async { [self] in
            var targetColor: UIColor
            switch level {
            case .off: targetColor = .clear
            case .debug: targetColor = .systemIndigo
            case .info: targetColor = .systemBlue
            case .warning: targetColor = .systemOrange
            case .error: targetColor = .systemRed
            }

            let string = NSMutableAttributedString(string: "[\(message.target ?? "nil")]  ", attributes: [
                .font: UIFont.boldSystemFont(ofSize: UIFont.systemFontSize),
                .foregroundColor: targetColor as Any
            ])

            string.append(NSAttributedString(string: message.text, attributes: [
                .font: UIFont.monospacedSystemFont(ofSize: UIFont.systemFontSize, weight: .regular),
                .foregroundColor: UIColor.label
            ]))

            string.append(NSAttributedString(string: "\n"))

            insert(attributedString: string)
            scrollToBottom()

            logTextView.reloadEmptyDataSet()
        }
    }

    func flush() {
    }

    @objc
    func actionClearLog() {
        logTextView.text = ""
        logTextView.reloadEmptyDataSet()
    }

    func title(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "log-controller-empty-title".localized)
    }

    func description(forEmptyDataSet scrollView: UIScrollView) -> NSAttributedString? {
        NSAttributedString(string: "log-controller-empty-subtitle".localized)
    }

    func image(forEmptyDataSet scrollView: UIScrollView) -> UIImage? {
        UIImage(
            systemName: "text.alignleft",
            withConfiguration: UIImage.SymbolConfiguration(pointSize: 120, weight: .light)
        )
    }

    func imageTintColor(forEmptyDataSet scrollView: UIScrollView?) -> UIColor? {
        .tertiaryLabel
    }

    func emptyDataSetShouldDisplay(_ scrollView: UIScrollView?) -> Bool {
        logTextView.text.isEmpty
    }
}
