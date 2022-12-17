//
//  CapacityView.swift
//  Twackup
//
//  Created by Daniil on 14.12.2022.
//

import UIKit

struct CapacityItem {
    var title: String

    var color: UIColor

    var bytes: Int64 = 0
}

class CapacityChartView: UIView {
    let bar = CapacityBarView()

    private let legendsStack: UIStackView = {
        let legendsStack = UIStackView()
        legendsStack.distribution = .equalSpacing
        legendsStack.translatesAutoresizingMaskIntoConstraints = false

        if UIDevice.current.userInterfaceIdiom == .phone {
            legendsStack.axis = .vertical
            legendsStack.spacing = 6.0
            legendsStack.alignment = .leading
        } else {
            legendsStack.axis = .horizontal
            legendsStack.spacing = 16.0
        }

        return legendsStack
    }()

    private(set) var items: [CapacityItem] = []

    var sortedItems: [CapacityItem] {
        items.sorted { $0.bytes < $1.bytes }
    }

    override var intrinsicContentSize: CGSize {
        setNeedsDisplay()
        layoutIfNeeded()

        let spacer = 10.0
        let height = bar.frame.size.height + spacer + legendsStack.frame.size.height

        return CGSize(width: max(bar.frame.size.width, legendsStack.frame.size.width), height: height)
    }

    private(set) var generalConstraints: [NSLayoutConstraint]?

    override init(frame: CGRect) {
        super.init(frame: frame)

        addSubview(bar)
        addSubview(legendsStack)

        bar.translatesAutoresizingMaskIntoConstraints = false
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func updateConstraints() {
        super.updateConstraints()

        if generalConstraints == nil {
            let constraints = [
                bar.topAnchor.constraint(equalTo: topAnchor),
                bar.heightAnchor.constraint(equalToConstant: 24.0),
                bar.leadingAnchor.constraint(equalTo: leadingAnchor),
                bar.trailingAnchor.constraint(equalTo: trailingAnchor),

                legendsStack.topAnchor.constraint(equalTo: bar.bottomAnchor, constant: 10.0),
                legendsStack.leadingAnchor.constraint(equalTo: leadingAnchor),
                legendsStack.bottomAnchor.constraint(greaterThanOrEqualTo: bottomAnchor)
            ]

            NSLayoutConstraint.activate(constraints)
            generalConstraints = constraints
        }
    }

    func set(items: [CapacityItem], animated: Bool = false) {
        let sortedItems = items.sorted { $0.bytes < $1.bytes }
        self.items = sortedItems

        bar.items = sortedItems
        bar.update()

        for subview in legendsStack.subviews {
            subview.removeFromSuperview()
        }

        let formatter = ByteCountFormatter()

        for item in sortedItems {
            let label = CapacityBarLabel()
            label.colorDotView.backgroundColor = item.color
            label.nameLabel.text = item.title
            label.valueLabel.text = formatter.string(fromByteCount: item.bytes)

            legendsStack.addArrangedSubview(label)
        }
    }
}
