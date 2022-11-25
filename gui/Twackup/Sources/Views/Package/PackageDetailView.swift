//
//  PackageDetailView.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import UIKit

class PackageDetailView : UIView {
    let identifierLabel = KeyValueLabelView(key: "Identifier:")
    let versionLabel = KeyValueLabelView(key: "Version:")
    let sectionLabel = KeyValueLabelView(key: "Section:")

    init() {
        super.init(frame: CGRectZero)

        addSubview(identifierLabel)
        addSubview(versionLabel)
        addSubview(sectionLabel)

        identifierLabel.translatesAutoresizingMaskIntoConstraints = false
        versionLabel.translatesAutoresizingMaskIntoConstraints = false
        sectionLabel.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            identifierLabel.leadingAnchor.constraint(equalTo: leadingAnchor),
            identifierLabel.topAnchor.constraint(equalTo: topAnchor),

            versionLabel.leadingAnchor.constraint(equalTo: leadingAnchor),
            versionLabel.topAnchor.constraint(equalTo: identifierLabel.bottomAnchor, constant: 8.0),

            sectionLabel.leadingAnchor.constraint(equalTo: leadingAnchor),
            sectionLabel.topAnchor.constraint(equalTo: versionLabel.bottomAnchor, constant: 8.0),
        ])
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    func updateContents(forPackage package: Package) {
        identifierLabel.valueLabel.text = package.id
        versionLabel.valueLabel.text = package.version
        sectionLabel.valueLabel.text = package.section.rawValue
    }
}
