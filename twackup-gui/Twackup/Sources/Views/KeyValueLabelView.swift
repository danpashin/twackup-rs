//
//  KeyValueLabelView.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class KeyValueLabel: UIView {
    lazy private(set) var keyLabel: UILabel = {
        var label = UILabel()
        label.textColor = .secondaryLabel
        label.font = UIFont.systemFont(ofSize: UIFont.labelFontSize, weight: .semibold)

        return label
    }()

    lazy private(set) var valueLabel: UILabel = {
        var label = UILabel()
        label.adjustsFontSizeToFitWidth = true

        return label
    }()

    init(key: String, value: String? = nil) {
        super.init(frame: .zero)

        keyLabel.text = key
        valueLabel.text = value

        addSubview(keyLabel)
        addSubview(valueLabel)

        keyLabel.translatesAutoresizingMaskIntoConstraints = false
        valueLabel.translatesAutoresizingMaskIntoConstraints = false

        NSLayoutConstraint.activate([
            keyLabel.leadingAnchor.constraint(equalTo: leadingAnchor),
            keyLabel.topAnchor.constraint(equalTo: topAnchor),
            keyLabel.bottomAnchor.constraint(equalTo: bottomAnchor),

            valueLabel.leadingAnchor.constraint(equalTo: keyLabel.trailingAnchor, constant: 8.0),
            valueLabel.trailingAnchor.constraint(equalTo: trailingAnchor),
            valueLabel.topAnchor.constraint(equalTo: topAnchor),
            valueLabel.bottomAnchor.constraint(equalTo: bottomAnchor)
        ])
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
}
