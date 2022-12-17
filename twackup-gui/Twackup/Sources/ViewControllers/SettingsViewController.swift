//
//  SettingsViewController.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

struct SettingsViewController: View {
    let metadata: ViewControllerMetadata

    let mainModel: MainModel

    @ObservedObject
    private var preferences = Preferences.default

    @State
    private var showClearDataAlert = false

    let diskUsageView: DiskSpaceUsage

    init(mainModel: MainModel, metadata: ViewControllerMetadata) {
        self.mainModel = mainModel
        self.metadata = metadata

        diskUsageView = DiskSpaceUsage(mainModel: mainModel)
    }

    var body: some View {
        NavigationView {
            List {
                Section(content: {
                    Picker("settings-compression-level", selection: preferences.compression.$level) {
                        ForEach(Compression.Level.allCases) { element in
                            Text(element.localized).tag(element)
                        }
                    }

                    Picker("settings-compression-type", selection: preferences.$compression.kind) {
                        ForEach(Compression.Kind.allCases) { element in
                            Text(element.localized).tag(element)
                        }
                    }
                }, header: {
                    Text("settings-compression-header")
                }, footer: {
                    Text("settings-compression-footer")
                })

                Section(content: {
                    diskUsageView
                        .padding(.vertical, 8.0)
                    Button("settings-clear-appdata-button") {
                        showClearDataAlert = true
                    }
                    .alert(isPresented: $showClearDataAlert) {
                        Alert(
                            title: Text("settings-clear-appdata-warning-title"),
                            message: Text("settings-clear-appdata-warning-message"),
                            primaryButton: .cancel(),
                            secondaryButton: .destructive(
                                Text("settings-clear-appdata-warning-clear-anyway"),
                                action: clearAppData
                            )
                        )
                    }
                }, header: {
                    Text("settings-disk-usage-header")
                })

                Section(content: {
                    Link(
                        "settings-donate-button",
                        destination: URL(string: "https://my.qiwi.com/Danyyl-PFxEvxeqrC")!
                    )
                    Link(
                        "settings-reportabug-button",
                        destination: URL(string: "https://github.com/danpashin/twackup-rs/issues/new")!
                    )
                    DetailedLabelSUI(
                        "settings-app-version-label",
                        detailed: String(utf8String: tw_library_version()) ?? "unknown".localized
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
        _ = mainModel.databasePackageProvider.deleteAll {
            NotificationCenter.default.post(name: DebsListModel.NotificationName, object: nil)
        }
    }
}
