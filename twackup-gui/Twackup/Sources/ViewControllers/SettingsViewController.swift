//
//  SettingsViewController.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

struct SettingsViewController: View {
    let metadata: ViewControllerMetadata

    @ObservedObject
    private var preferences = Preferences.default

    @State
    private var showClearDataAlert = false

    private let diskUsageView = DiskSpaceUsage()

    init(metadata: ViewControllerMetadata) {
        self.metadata = metadata
    }

    var body: some View {
        NavigationView {
            List {
                Section(content: {
                    Picker("settings-compression-level".localized, selection: preferences.compression.$level) {
                        ForEach(Compression.Level.allCases) { element in
                            Text(element.localized).tag(element)
                        }
                    }

                    Picker("settings-compression-type".localized, selection: preferences.$compression.kind) {
                        ForEach(Compression.Kind.allCases) { element in
                            Text(element.localized).tag(element)
                        }
                    }
                }, header: {
                    Text("settings-compression-header".localized)
                }, footer: {
                    Text("settings-compression-footer".localized)
                })

                Section(content: {
                    diskUsageView
                        .padding(.vertical, 8.0)
                    Button("settings-clear-appdata-button".localized) {
                        showClearDataAlert = true
                    }
                    .alert(isPresented: $showClearDataAlert) {
                        Alert(
                            title: Text("settings-clear-appdata-warning-title".localized),
                            message: Text("settings-clear-appdata-warning-message".localized),
                            primaryButton: .cancel(),
                            secondaryButton: .destructive(
                                Text("settings-clear-appdata-warning-clear-anyway".localized),
                                action: clearAppData
                            )
                        )
                    }
                }, header: {
                    Text("settings-disk-usage-header".localized)
                })

                Section(content: {
                    Link(
                        "settings-donate-button".localized,
                        destination: URL(string: "https://my.qiwi.com/Danyyl-PFxEvxeqrC")!
                    )
                    Link(
                        "settings-reportabug-button".localized,
                        destination: URL(string: "https://github.com/danpashin/twackup-rs/issues/new")!
                    )
                    DetailedLabelSUI(
                        "settings-app-version-label".localized,
                        detailed: String(utf8String: tw_library_version()) ?? "unknown"
                    )
                }, footer: {
                    Text("Copyright (c) 2022 danpashin. All rights reserved")
                })
            }
            .listStyle(.insetGrouped)
            .navigationTitle(metadata.navTitle)
        }
        .navigationViewStyle(.stack)
    }

    func clearAppData() {
        guard let delegate = UIApplication.shared.delegate as? AppDelegate else { return }

        let provider = DatabasePackageProvider(delegate.database)
        _ = provider.deleteAll {
            NotificationCenter.default.post(name: PackageVC.DebsListModel.NotificationName, object: nil)
        }
    }
}
