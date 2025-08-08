import Cocoa
import Foundation

@main
class AppDelegate: NSObject, NSApplicationDelegate {
    var rustPopupProcess: Process?
    var rustPopupPID: pid_t = 0
    var popupPath: String = ""
    var lastShowTime: Date = Date.distantPast
    var windowCheckTimer: Timer?
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        // Set up as background app (no dock icon)
        NSApp.setActivationPolicy(.accessory)
        
        // Determine popup path
        if let bundlePath = Bundle.main.resourcePath {
            // When running as app bundle
            popupPath = "\(bundlePath)/popup"
        } else {
            // When running as standalone script during development
            if let execPath = ProcessInfo.processInfo.arguments.first {
                let execURL = URL(fileURLWithPath: execPath)
                let parentDir = execURL.deletingLastPathComponent().deletingLastPathComponent()
                popupPath = parentDir.appendingPathComponent("target/release/popup").path
            }
        }
        
        // Check if popup binary exists
        if !FileManager.default.fileExists(atPath: popupPath) {
            NSLog("HookAnchor Supervisor: Cannot find popup binary at \(popupPath)")
            // Try alternative path for development
            if let execPath = ProcessInfo.processInfo.arguments.first {
                let execURL = URL(fileURLWithPath: execPath)
                let projectRoot = execURL.deletingLastPathComponent().deletingLastPathComponent().deletingLastPathComponent()
                popupPath = projectRoot.appendingPathComponent("target/release/popup").path
                if !FileManager.default.fileExists(atPath: popupPath) {
                    NSLog("HookAnchor Supervisor: Also cannot find popup at \(popupPath)")
                    NSApplication.shared.terminate(self)
                    return
                }
            }
        }
        
        NSLog("HookAnchor Supervisor: Starting with popup at \(popupPath)")
        
        // Launch Rust popup once
        launchRustPopup()
        
        // Set up periodic check to ensure popup is running
        windowCheckTimer = Timer.scheduledTimer(withTimeInterval: 5.0, repeats: true) { _ in
            self.ensurePopupRunning()
        }
    }
    
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        // Called when app is "launched" while already running
        NSLog("HookAnchor Supervisor: Reopen event received")
        showRustWindow()
        return true
    }
    
    func applicationWillTerminate(_ notification: Notification) {
        // Clean up child process
        if let process = rustPopupProcess, process.isRunning {
            process.terminate()
        }
    }
    
    func launchRustPopup() {
        // Terminate any existing process
        if let process = rustPopupProcess, process.isRunning {
            process.terminate()
            process.waitUntilExit()
        }
        
        let task = Process()
        task.executableURL = URL(fileURLWithPath: popupPath)
        task.arguments = ["--server"] // Run in server mode
        
        // Set up environment
        var environment = ProcessInfo.processInfo.environment
        environment["RUST_BACKTRACE"] = "1"
        task.environment = environment
        
        // Launch the process
        do {
            try task.run()
            rustPopupPID = task.processIdentifier
            rustPopupProcess = task
            NSLog("HookAnchor Supervisor: Launched popup with PID \(rustPopupPID)")
            
            // Give it a moment to initialize
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                // Initial show
                self.showRustWindow()
            }
        } catch {
            NSLog("HookAnchor Supervisor: Failed to launch popup: \(error)")
        }
    }
    
    func showRustWindow() {
        // Prevent too-rapid repeated shows
        let now = Date()
        if now.timeIntervalSince(lastShowTime) < 0.1 {
            return
        }
        lastShowTime = now
        
        // First ensure the popup is running
        ensurePopupRunning()
        
        // Method 1: Try to activate by PID using NSRunningApplication
        if rustPopupPID > 0 {
            if let app = NSRunningApplication(processIdentifier: rustPopupPID) {
                NSLog("HookAnchor Supervisor: Activating app with PID \(rustPopupPID)")
                app.activate(options: [.activateAllWindows])
                
                // Also try unhiding
                if app.isHidden {
                    app.unhide()
                }
            }
        }
        
        // Method 2: Find and show windows by PID using CGWindowListCopyWindowInfo
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            self.showWindowsByPID()
        }
        
        // Method 3: Send a signal or message to show (if the popup supports it)
        // For now, the popup should detect activation and show itself
    }
    
    func showWindowsByPID() {
        guard rustPopupPID > 0 else { return }
        
        // Get all windows
        let options = CGWindowListOption(arrayLiteral: .excludeDesktopElements, .optionOnScreenOnly)
        guard let windowList = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] else {
            return
        }
        
        // Find windows belonging to our popup process
        var foundWindow = false
        for window in windowList {
            if let ownerPID = window[kCGWindowOwnerPID as String] as? pid_t,
               ownerPID == rustPopupPID {
                foundWindow = true
                
                // Try to bring window to front using Accessibility API
                if let windowNumber = window[kCGWindowNumber as String] as? CGWindowID {
                    NSLog("HookAnchor Supervisor: Found popup window \(windowNumber)")
                    
                    // Create AXUIElement for the application
                    let appElement = AXUIElementCreateApplication(rustPopupPID)
                    
                    // Get all windows
                    var windowsValue: CFTypeRef?
                    let result = AXUIElementCopyAttributeValue(appElement, kAXWindowsAttribute as CFString, &windowsValue)
                    
                    if result == .success,
                       let windows = windowsValue as? [AXUIElement],
                       !windows.isEmpty {
                        // Raise the first window
                        AXUIElementPerformAction(windows[0], kAXRaiseAction as CFString)
                        
                        // Also try to make it main
                        let mainValue: CFTypeRef = kCFBooleanTrue
                        AXUIElementSetAttributeValue(windows[0], kAXMainAttribute as CFString, mainValue)
                        
                        // And focused
                        let focusedValue: CFTypeRef = kCFBooleanTrue
                        AXUIElementSetAttributeValue(windows[0], kAXFocusedAttribute as CFString, focusedValue)
                    }
                }
            }
        }
        
        if !foundWindow {
            NSLog("HookAnchor Supervisor: No windows found for PID \(rustPopupPID)")
            // The popup might be hidden or not have created its window yet
            // Try activating the app again
            if let app = NSRunningApplication(processIdentifier: rustPopupPID) {
                app.activate(options: [.activateAllWindows])
            }
        }
    }
    
    func ensurePopupRunning() {
        if let process = rustPopupProcess, process.isRunning {
            // Process is still running
            return
        }
        
        // Process has died, restart it
        NSLog("HookAnchor Supervisor: Popup process died, restarting...")
        launchRustPopup()
    }
}

// Helper to check if we're running in an app bundle
extension Bundle {
    static var isAppBundle: Bool {
        return Bundle.main.bundleURL.pathExtension == "app"
    }
}