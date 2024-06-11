//
//  DiskSpaceUsageView.swift
//  Twackup
//
//  Created by Daniil on 14.12.2022.
//

import SwiftUI
import UIKit

class DiskSpaceUsageView: UIView {
    let diskStats: DiskStats

    let activityIndicator = UIActivityIndicatorView(style: .medium)

    let chart = CapacityChartView()

    private(set) lazy var appItem = CapacityItem(title: "app", color: tintColor)

    private(set) var deviceItem = CapacityItem(title: "device", color: .systemGray2)

    private(set) var totalItem = CapacityItem(title: "total", color: .systemGray4)

    private(set) var generalConstraints: [NSLayoutConstraint]?

    private var reloadObserver: NSObjectProtocol?

    override var intrinsicContentSize: CGSize {
        chart.intrinsicContentSize
    }

    init(mainModel: MainModel) {
        diskStats = DiskStats(mainModel: mainModel)
        super.init(frame: .zero)

        addSubview(activityIndicator)
        addSubview(chart)

        activityIndicator.translatesAutoresizingMaskIntoConstraints = false
        chart.translatesAutoresizingMaskIntoConstraints = false

        chart.set(items: [appItem, deviceItem, totalItem])

        reloadObserver = NotificationCenter.default.addObserver(
            forName: DebsListModel.NotificationName,
            object: nil,
            queue: .main
        ) { [weak self] _  in
            guard let self else { return }
            Task {
                await self.update()
            }
        }
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    deinit {
        // Apple moment
        Task {
            await MainActor.run {
                NotificationCenter.default.removeObserver(reloadObserver as Any)
            }
        }
    }

    override func updateConstraints() {
        super.updateConstraints()

        if generalConstraints == nil {
            let constraints = [
                chart.topAnchor.constraint(equalTo: topAnchor),
                chart.bottomAnchor.constraint(equalTo: bottomAnchor),
                chart.leadingAnchor.constraint(equalTo: leadingAnchor),
                chart.trailingAnchor.constraint(equalTo: trailingAnchor),

                activityIndicator.centerYAnchor.constraint(equalTo: centerYAnchor),
                activityIndicator.centerXAnchor.constraint(equalTo: centerXAnchor)
            ]

            NSLayoutConstraint.activate(constraints)
            generalConstraints = constraints
        }
    }

    func update() {
        chart.isHidden = true
        activityIndicator.startAnimating()

        Task {
            await diskStats.update()

            appItem.bytes = await diskStats.appSpace
            appItem.title = "disk-usage-app".localized + " • "

            deviceItem.bytes = await diskStats.usedSpace
            deviceItem.title = "disk-usage-other".localized + " • "

            totalItem.bytes = await diskStats.totalSpace
            totalItem.title = "disk-usage-total-space".localized + " • "

            chart.set(items: [appItem, deviceItem, totalItem])

            activityIndicator.stopAnimating()
            chart.isHidden = false
        }
    }
}

@MainActor
struct DiskSpaceUsage: UIViewRepresentable {
    typealias UIViewType = DiskSpaceUsageView

    let view: DiskSpaceUsageView

    let mainModel: MainModel

    init(mainModel: MainModel) {
        self.mainModel = mainModel

        let view = DiskSpaceUsageView(mainModel: mainModel)
        self.view = view
    }

    func makeUIView(context: Context) -> UIViewType {
        view.update()

        return view
    }

    func updateUIView(_ uiView: UIViewType, context: Context) {
    }
}
