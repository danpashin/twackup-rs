//
//  DiskStats.swift
//  Twackup
//
//  Created by Daniil on 14.12.2022.
//

class DiskStats {
    private(set) var totalSpace: Int64 = 0

    private(set) var usedSpace: Int64 = 0

    private(set) var appSpace: Int64 = 0

    let mainModel: MainModel

    init(mainModel: MainModel) {
        self.mainModel = mainModel
    }

    func update(callback: @escaping () -> Void) {
        DispatchQueue.global().async { [self] in
            let homeDir = URL(fileURLWithPath: NSHomeDirectory())

            do {
                let values = try homeDir.resourceValues(forKeys: [
                    .volumeTotalCapacityKey, .volumeAvailableCapacityForImportantUsageKey
                ])

                totalSpace = Int64(values.volumeTotalCapacity ?? 0)
                usedSpace = totalSpace - Int64(values.volumeAvailableCapacityForImportantUsage ?? 0)
                appSpace = mainModel.database.databaseSize() + mainModel.database.packagesSize()
            } catch {
                FFILogger.shared.log(error.localizedDescription, level: .warning)
            }

            callback()
        }
    }
}
