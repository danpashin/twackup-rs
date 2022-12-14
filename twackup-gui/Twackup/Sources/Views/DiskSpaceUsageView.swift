//
//  DiskSpaceUsageView.swift
//  Twackup
//
//  Created by Daniil on 14.12.2022.
//

import UIKit
import SwiftUI

class DiskSpaceUsageView: UIView {
    let diskStats = DiskStats()

    let activityIndicator = UIActivityIndicatorView(style: .medium)

    let chart = CapacityChartView()

    private(set) lazy var appItem = CapacityItem(title: "app", color: tintColor)

    private(set) var deviceItem = CapacityItem(title: "", color: .systemGray3)

    private(set) var totalItem = CapacityItem(title: "", color: .systemGray5)

    override var intrinsicContentSize: CGSize {
        chart.intrinsicContentSize
    }

    override init(frame: CGRect) {
        super.init(frame: frame)

        addSubview(activityIndicator)
        addSubview(chart)

        activityIndicator.translatesAutoresizingMaskIntoConstraints = false
        chart.translatesAutoresizingMaskIntoConstraints = false

        NSLayoutConstraint.activate([
            chart.topAnchor.constraint(equalTo: topAnchor),
            chart.bottomAnchor.constraint(equalTo: bottomAnchor),
            chart.leadingAnchor.constraint(equalTo: leadingAnchor),
            chart.trailingAnchor.constraint(equalTo: trailingAnchor),

            activityIndicator.centerYAnchor.constraint(equalTo: centerYAnchor),
            activityIndicator.centerXAnchor.constraint(equalTo: centerXAnchor)
        ])

        chart.set(items: [appItem, deviceItem, totalItem])
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    func update() {
        chart.isHidden = true
        activityIndicator.startAnimating()

        diskStats.update { [self] in
            let fmt = ByteCountFormatter()
            fmt.countStyle = .file
            fmt.allowedUnits = [.useMB, .useGB, .usePB]

            let appSpaceString = fmt.string(fromByteCount: diskStats.appSpace)
            appItem.bytes = UInt64(diskStats.appSpace)
            appItem.title = Bundle.appLocalize("disk-usage-app") + " • \(appSpaceString)"

            let usedSpaceString = fmt.string(fromByteCount: diskStats.usedSpace)
            deviceItem.bytes = UInt64(diskStats.usedSpace)
            deviceItem.title = Bundle.appLocalize("disk-usage-other") + " • \(usedSpaceString)"

            let totalSpaceString = fmt.string(fromByteCount: diskStats.totalSpace)
            totalItem.bytes = UInt64(diskStats.totalSpace)
            totalItem.title = Bundle.appLocalize("disk-usage-total-space") + " • \(totalSpaceString)"

            DispatchQueue.main.sync {
                chart.set(items: [appItem, deviceItem, totalItem])

                activityIndicator.stopAnimating()
                chart.isHidden = false
            }
        }
    }
}

struct DiskSpaceUsage: UIViewRepresentable {
    typealias UIViewType = DiskSpaceUsageView
    func makeUIView(context: Context) -> UIViewType {
        DiskSpaceUsageView()
    }

    func updateUIView(_ uiView: UIViewType, context: Context) {
        uiView.update()
    }
}
