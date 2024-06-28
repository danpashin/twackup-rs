//
//  LogViewController.swift
//  Twackup
//
//  Created by Daniil on 09.12.2022.
//

import BlankSlate
import StyledTextKit

final class LogViewController: UIViewController, ScrollableViewController {
    let metadata: ViewControllerMetadata

    private let styledLogger = StyledLogger()

    private let textView = UIView()

    private(set) lazy var scrollView: UIScrollView = {
        let view = UIScrollView()
        view.isScrollEnabled = true
        view.alwaysBounceVertical = true
        view.addSubview(textView)

        return view
    }()

    private(set) lazy var clearLogButton: UIBarButtonItem = {
        let title = "log-clear-btn".localized
        return UIBarButtonItem(title: title, style: .plain, target: self, action: #selector(actionClearLog))
    }()

    init(metadata: ViewControllerMetadata) {
        self.metadata = metadata
        super.init(nibName: nil, bundle: nil)
        tabBarItem = metadata.tabbarItem
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

        Task {
            await renderLog()
        }
    }

    private func renderLog() async {
        do {
            let category = traitCollection.preferredContentSizeCategory
            let result = try await styledLogger.render(for: view.frame.width, contentSizeCategory: category)

            textView.layer.contents = result.image
            textView.frame = CGRect(origin: CGPoint(x: result.insets.left, y: result.insets.top), size: result.size)
        } catch {
            textView.layer.contents = nil
            textView.frame = .zero
            print(error)
        }
    }

    @objc
    func actionClearLog() {
        Task {
            await styledLogger.flush()
            await renderLog()
        }
    }

    // MARK: - ScrollableViewController

    func scrollToInitialPosition(animated: Bool) {
    }
}
