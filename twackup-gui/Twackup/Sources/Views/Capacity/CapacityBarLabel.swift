//
//  CapacityBarLabel.swift
//  Twackup
//
//  Created by Daniil on 14.12.2022.
//

import UIKit

class CircledView: UIView {
    override var bounds: CGRect {
        get { super.bounds }
        set {
            super.bounds = newValue

            precondition(newValue.width == newValue.height, "Circle bounds must be square to correctly render a circle")

            layer.cornerRadius = newValue.width / 2.0
            layer.masksToBounds = true
        }
    }
}

class CapacityBarLabel: UIView {
    let colorDotView = CircledView()

    let nameLabel: UILabel = {
        let label = UILabel()
        label.font = UIFont.preferredFont(forTextStyle: .subheadline)

        return label
    }()

    let valueLabel: UILabel = {
        let label = UILabel()

        let font = UIFont.preferredFont(forTextStyle: .subheadline)
        label.font = UIFont.boldSystemFont(ofSize: font.fontDescriptor.pointSize)

        return label
    }()

    override init(frame: CGRect) {
        super.init(frame: frame)

        addSubview(colorDotView)
        addSubview(nameLabel)
        addSubview(valueLabel)

        colorDotView.translatesAutoresizingMaskIntoConstraints = false
        nameLabel.translatesAutoresizingMaskIntoConstraints = false
        valueLabel.translatesAutoresizingMaskIntoConstraints = false

        NSLayoutConstraint.activate([
            colorDotView.widthAnchor.constraint(equalToConstant: 8.0),
            colorDotView.heightAnchor.constraint(equalToConstant: 8.0),
            colorDotView.leadingAnchor.constraint(equalTo: leadingAnchor),
            colorDotView.centerYAnchor.constraint(equalTo: nameLabel.centerYAnchor),

            nameLabel.topAnchor.constraint(equalTo: topAnchor),
            nameLabel.bottomAnchor.constraint(equalTo: bottomAnchor),
            nameLabel.leadingAnchor.constraint(equalTo: colorDotView.trailingAnchor, constant: 8.0),

            valueLabel.topAnchor.constraint(equalTo: topAnchor),
            valueLabel.bottomAnchor.constraint(equalTo: bottomAnchor),
            valueLabel.trailingAnchor.constraint(equalTo: trailingAnchor),
            valueLabel.leadingAnchor.constraint(equalTo: nameLabel.trailingAnchor)
        ])
    }

    required init(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
}
