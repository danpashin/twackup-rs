//
//  DetailedLabelSUI.swift
//  Twackup
//
//  Created by Daniil on 16.12.2022.
//

import SwiftUI

struct DetailedLabelSUI: View {
    let text: LocalizedStringKey

    let detailed: any StringProtocol

    var body: some View {
        HStack {
            Text(text)
            Spacer()
            Text(detailed)
                .foregroundColor(.secondary)
        }
    }

    init(_ text: LocalizedStringKey, detailed: any StringProtocol) {
        self.text = text
        self.detailed = detailed
    }
}
