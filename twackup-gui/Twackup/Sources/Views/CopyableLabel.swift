//
//  CopyableLabel.swift
//  Twackup
//
//  Created by Daniil on 22.12.2022.
//

class CopyableLabel: UILabel {
    override var canBecomeFirstResponder: Bool {
        return true
    }

    override init(frame: CGRect) {
        super.init(frame: frame)

        isUserInteractionEnabled = true
        addGestureRecognizer(UILongPressGestureRecognizer(target: self, action: #selector(showMenu)))
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    @objc
    func showMenu(_ sender: UILongPressGestureRecognizer) {
        if sender.state != .began { return }

        becomeFirstResponder()
        let menu = UIMenuController.shared
        if !menu.isMenuVisible {
            menu.showMenu(from: self, rect: bounds)
        }
    }

    override func copy(_ sender: Any?) {
        UIPasteboard.general.string = text
    }
}
