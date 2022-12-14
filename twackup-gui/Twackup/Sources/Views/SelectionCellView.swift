//
//  SelectionCellView.swift
//  Twackup
//
//  Created by Daniil on 13.12.2022.
//

import SwiftUI

protocol AsString: Identifiable {
    var asString: String { get }
}

struct SelectionCellView<Data>: View where Data: Identifiable & AsString {
    let name: String

    let data: [Data]

    let selectHandler: (Data) -> Void

    @State var currentValue: Data

    var body: some View {
        HStack {
            Text(name)
            Spacer()

            // swiftlint:disable multiple_closures_with_trailing_closure
            Menu(content: {
                ForEach(data) { element in
                    Button(element.asString, action: {
                        currentValue = element
                        selectHandler(element)
                    })
                }
            }) {
                Text(currentValue.asString)
            }
        }
    }
}
