//
//  KeyValueLabelView.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class KeyValueLabel: UIView {
    private(set) lazy var keyLabel: UILabel = {
        var label = UILabel()
        label.textColor = .secondaryLabel
        label.font = UIFont.systemFont(ofSize: UIFont.labelFontSize, weight: .semibold)
        label.translatesAutoresizingMaskIntoConstraints = false

        return label
    }()

    private(set) lazy var valueLabel: UILabel = {
        var label = UILabel()
        label.adjustsFontSizeToFitWidth = true
        label.translatesAutoresizingMaskIntoConstraints = false

        return label
    }()

    private(set) var generalConstraints: [NSLayoutConstraint]?

    init(key: String, value: String? = nil) {
        super.init(frame: .zero)

        keyLabel.text = key
        valueLabel.text = value

        addSubview(keyLabel)
        addSubview(valueLabel)
    }

    override func updateConstraints() {
        super.updateConstraints()

        if generalConstraints == nil {
            let constraints = [
                keyLabel.leadingAnchor.constraint(equalTo: leadingAnchor),
                keyLabel.topAnchor.constraint(equalTo: topAnchor),
                keyLabel.bottomAnchor.constraint(equalTo: bottomAnchor),

                valueLabel.leadingAnchor.constraint(equalTo: keyLabel.trailingAnchor, constant: 8.0),
                valueLabel.trailingAnchor.constraint(equalTo: trailingAnchor),
                valueLabel.topAnchor.constraint(equalTo: topAnchor),
                valueLabel.bottomAnchor.constraint(equalTo: bottomAnchor)
            ]

            NSLayoutConstraint.activate(constraints)
            generalConstraints = constraints
        }
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
}
