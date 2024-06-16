//
//  DiskStats.swift
//  Twackup
//
//  Created by Daniil on 14.12.2022.
//

actor DiskStats {
    private(set) var totalSpace: Int64 = 0

    private(set) var usedSpace: Int64 = 0

    private(set) var appSpace: Int64 = 0

    let mainModel: MainModel

    init(mainModel: MainModel) {
        self.mainModel = mainModel
    }

    func update() async {
        let homeDir = URL(fileURLWithPath: NSHomeDirectory())

        do {
            let values = try homeDir.resourceValues(forKeys: [
                .volumeTotalCapacityKey, .volumeAvailableCapacityForImportantUsageKey
            ])

            totalSpace = Int64(values.volumeTotalCapacity ?? 0)
            usedSpace = totalSpace - Int64(values.volumeAvailableCapacityForImportantUsage ?? 0)
            appSpace = await mainModel.database.databaseSize() + mainModel.database.packagesSize()
        } catch {
           await FFILogger.shared.log(error.localizedDescription, level: .warning)
        }
    }
}
