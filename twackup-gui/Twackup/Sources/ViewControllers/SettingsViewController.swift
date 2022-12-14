//
//  SettingsViewController.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

struct SettingsViewController: View {
    private var preferences = Preferences.default

    var body: some View {
        NavigationView {
            List {
                Section(content: {
                    SelectionCellView(name: "Compression type",
                                      data: Compression.Kind.allValues,
                                      selectHandler: { preferences.compression.kind = $0},
                                      currentValue: preferences.compression.kind)

                    SelectionCellView(name: "Compression level",
                                      data: Compression.Level.allValues,
                                      selectHandler: { preferences.compression.level = $0},
                                      currentValue: preferences.compression.level)
                }, header: {
                    Text("Packages compression")
                }, footer: {
                    Text("Compression will affect on packaging time for single DEB archive and on space occupied by the final file. Select proper for your device and free disk space.")
                })

                Section(content: {
                    DiskSpaceUsage()
                        .padding(.vertical, 8.0)
                    Button("Clear Twackup data") {

                    }
                }, header: {
                    Text("Device disk usage")
                })

                Section(content: {
                    Link("☕️ Buy me a coffee", destination: URL(string: "https://github.com/danpashin/twackup-rs")!)
                    Link("🪲 Report a bug on GitHub",
                         destination: URL(string: "https://github.com/danpashin/twackup-rs/issues/new")!
                    )
                }, footer: {
                    Text("Copyright (c) 2022 danpashin. All rights reserved")
                })
            }
            .listStyle(.insetGrouped)
        }
        .navigationTitle("Settings")
        .navigationViewStyle(.stack)
    }
}
