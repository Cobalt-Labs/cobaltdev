//
//  MessengerApp.swift
//  Messenger
//
//  Created by Ibrahim Haji on 12/04/26.
//

import SwiftUI
import CoreData

@main
struct MessengerApp: App {
    let persistenceController = PersistenceController.shared

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(\.managedObjectContext, persistenceController.container.viewContext)
        }
    }
}
