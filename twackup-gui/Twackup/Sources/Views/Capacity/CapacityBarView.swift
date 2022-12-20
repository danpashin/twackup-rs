//
//  CapacityBarView.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import UIKit

class CapacityBarView: UIView {
    // Must be sorted
    var items: [CapacityItem] = []

    var separatorColor: UIColor = .systemBackground

    override init(frame: CGRect) {
        super.init(frame: frame)

        layer.backgroundColor = UIColor.clear.cgColor
        layer.cornerCurve = .circular
        layer.cornerRadius = 6.0
        layer.masksToBounds = true
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) is not supported")
    }

    override func draw(_ rect: CGRect) {
        guard let context = UIGraphicsGetCurrentContext() else { return }
        guard let maxValue = items.map({ $0.bytes }).max() else { return }

        var currentXPos = 0.0

        for category in items {
            let itemWidth = max(CGFloat(category.bytes) / CGFloat(maxValue) * rect.width, 2.5)
            let itemRect = CGRect(x: currentXPos, y: 0, width: min(itemWidth, rect.width), height: rect.height)
            currentXPos += itemRect.width

            context.setFillColor(category.color.cgColor)
            context.fill(itemRect)

            let separatorRect = CGRect(x: currentXPos, y: 0, width: 1.25, height: rect.height)
            currentXPos += separatorRect.width

            context.setFillColor(separatorColor.cgColor)
            context.fill(separatorRect)
        }
    }

    override func traitCollectionDidChange(_ previousTraitCollection: UITraitCollection?) {
        super.traitCollectionDidChange(previousTraitCollection)

        update()
    }

    func update() {
        precondition(Thread.isMainThread, "\(#function) must be called from main thread only!")
        setNeedsDisplay()
    }
}
