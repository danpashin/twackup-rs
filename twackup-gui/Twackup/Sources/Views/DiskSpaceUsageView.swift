//
//  DiskSpaceUsageView.swift
//  Twackup
//
//  Created by Daniil on 14.12.2022.
//

import SwiftUI
import UIKit

class DiskSpaceUsageView: UIView {
    let diskStats = DiskStats()

    let activityIndicator = UIActivityIndicatorView(style: .medium)

    let chart = CapacityChartView()

    private(set) lazy var appItem = CapacityItem(title: "app", color: tintColor)

    private(set) var deviceItem = CapacityItem(title: "device", color: .systemGray3)

    private(set) var totalItem = CapacityItem(title: "total", color: .systemGray5)

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
            appItem.bytes = diskStats.appSpace
            appItem.title = Bundle.appLocalize("disk-usage-app") + " • "

            deviceItem.bytes = diskStats.usedSpace
            deviceItem.title = Bundle.appLocalize("disk-usage-other") + " • "

            totalItem.bytes = diskStats.totalSpace
            totalItem.title = Bundle.appLocalize("disk-usage-total-space") + " • "

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

    let view: DiskSpaceUsageView

    private var reloadObserver: NSObjectProtocol

    init() {
        let view = DiskSpaceUsageView()
        self.view = view

        reloadObserver = NotificationCenter.default.addObserver(
            forName: PackageVC.DebsListModel.NotificationName,
            object: nil,
            queue: .current
        ) { _  in
            DispatchQueue.main.async {
                view.update()
            }
        }
    }

    func makeUIView(context: Context) -> UIViewType {
        view
    }

    func updateUIView(_ uiView: UIViewType, context: Context) {
        uiView.update()
    }
}
