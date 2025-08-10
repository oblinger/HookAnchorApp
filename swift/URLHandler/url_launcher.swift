import Cocoa
import Foundation

@main
struct URLLauncher {
    static func main() {
        let app = NSApplication.shared
        let delegate = URLHandlerDelegate()
        app.delegate = delegate
        app.run()
    }
}

class URLHandlerDelegate: NSObject, NSApplicationDelegate {
    var receivedURL = false
    
    func applicationWillFinishLaunching(_ notification: Notification) {
        // Register for URL events BEFORE launch completes
        NSAppleEventManager.shared().setEventHandler(
            self,
            andSelector: #selector(handleURLEvent(_:withReplyEvent:)),
            forEventClass: AEEventClass(kInternetEventClass),
            andEventID: AEEventID(kAEGetURL)
        )
        
        // Log to help debug
        NSLog("URLHandler: Registered for URL events")
    }
    
    @objc func handleURLEvent(_ event: NSAppleEventDescriptor, 
                              withReplyEvent replyEvent: NSAppleEventDescriptor) {
        guard let urlString = event.paramDescriptor(forKeyword: keyDirectObject)?.stringValue else {
            NSLog("URLHandler: No URL in event")
            NSApplication.shared.terminate(nil)
            return
        }
        
        NSLog("URLHandler: Received URL: \(urlString)")
        receivedURL = true
        
        // Get the path to ha (it's a symlink in our MacOS folder)
        let haPath = "/Applications/HookAnchor.app/Contents/MacOS/ha"
        
        // Create process to execute ha --hook
        let task = Process()
        task.executableURL = URL(fileURLWithPath: haPath)
        task.arguments = ["--hook", urlString]
        
        // Redirect output to /dev/null to avoid blocking
        task.standardOutput = FileHandle.nullDevice
        task.standardError = FileHandle.nullDevice
        
        do {
            try task.run()
            NSLog("URLHandler: Launched ha with --hook")
            // Don't wait for it to complete - exit immediately
        } catch {
            NSLog("URLHandler: Failed to launch ha: \(error)")
        }
        
        // Exit immediately - don't block the calling app
        NSApplication.shared.terminate(nil)
    }
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        // If we didn't receive a URL within a short time, just exit
        // This handles the case where the app is launched directly
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) { [weak self] in
            if !(self?.receivedURL ?? false) {
                NSLog("URLHandler: No URL received, exiting")
                NSApplication.shared.terminate(nil)
            }
        }
    }
}