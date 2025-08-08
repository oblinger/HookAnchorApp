import Cocoa
import Foundation
import Darwin

// Main entry point for the application
class HookAnchorMain {
    static func main() {
        // Create the application
        let app = NSApplication.shared
        
        // Create and set the delegate
        let delegate = AppDelegate()
        app.delegate = delegate
        
        // Run the application
        app.run()
    }
}

@main
struct HookAnchorApp {
    static func main() {
        HookAnchorMain.main()
    }
}

class AppDelegate: NSObject, NSApplicationDelegate {
    
    // Unified logging to anchor.log
    func log(_ message: String) {
        // Format timestamp to match Rust format: "2025-08-07 21:48:38"
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd HH:mm:ss"
        formatter.timeZone = TimeZone.current
        let timestamp = formatter.string(from: Date())
        
        let logMessage = "\(timestamp) SUPERVISOR: \(message)\n"
        
        // Also log to system log for debugging
        NSLog("HookAnchor Supervisor: \(message)")
        
        // Append to anchor.log
        let homeDir = FileManager.default.homeDirectoryForCurrentUser.path(percentEncoded: false)
        if !homeDir.isEmpty {
            let logPath = "\(homeDir)/.config/hookanchor/anchor.log"
            
            // Create config directory if needed
            let configDir = "\(homeDir)/.config/hookanchor"
            try? FileManager.default.createDirectory(atPath: configDir, withIntermediateDirectories: true)
            
            // Append to log file
            if let data = logMessage.data(using: .utf8) {
                if FileManager.default.fileExists(atPath: logPath) {
                    if let fileHandle = FileHandle(forWritingAtPath: logPath) {
                        fileHandle.seekToEndOfFile()
                        fileHandle.write(data)
                        fileHandle.closeFile()
                    }
                } else {
                    try? data.write(to: URL(fileURLWithPath: logPath))
                }
            }
        }
    }
    var rustPopupProcess: Process?
    var rustPopupPID: pid_t = 0
    var popupPath: String = ""
    var lastShowTime: Date = Date.distantPast
    // Removed timer - no need for continuous monitoring
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        // Register for URL events (for hook:// URLs)
        NSAppleEventManager.shared().setEventHandler(
            self,
            andSelector: #selector(handleURLEvent(_:withReplyEvent:)),
            forEventClass: AEEventClass(kInternetEventClass),
            andEventID: AEEventID(kAEGetURL)
        )
        // Immediate test - write to file directly with matching timestamp format
        let testPath = "/tmp/hookanchor_supervisor_test.log"
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd HH:mm:ss"
        formatter.timeZone = TimeZone.current
        let timestamp = formatter.string(from: Date())
        try? "SUPERVISOR STARTED at \(timestamp)\n".write(toFile: testPath, atomically: true, encoding: .utf8)
        
        log("Starting up...")
        
        // Set up as background app (no dock icon)
        NSApp.setActivationPolicy(.accessory)
        
        // Determine popup_server path (the actual Rust GUI)
        let bundlePath = Bundle.main.bundlePath
        if bundlePath.contains(".app") {
            // When running as app bundle - popup_server is in MacOS directory
            popupPath = "\(bundlePath)/Contents/MacOS/popup_server"
        } else {
            // When running standalone (not in app bundle)
            if let execPath = ProcessInfo.processInfo.arguments.first {
                let execURL = URL(fileURLWithPath: execPath)
                let execDir = execURL.deletingLastPathComponent()
                
                // First try: popup_server in same directory as HookAnchor
                popupPath = execDir.appendingPathComponent("popup_server").path
                
                // If not found, try development layout (up two dirs to project root)
                if !FileManager.default.fileExists(atPath: popupPath) {
                    let parentDir = execDir.deletingLastPathComponent()
                    if parentDir.lastPathComponent == "release" || parentDir.lastPathComponent == "debug" {
                        popupPath = execDir.appendingPathComponent("popup_server").path
                    }
                }
            }
        }
        
        // Check if popup binary exists
        if !FileManager.default.fileExists(atPath: popupPath) {
            log(" Cannot find popup binary at \(popupPath)")
            // Try alternative path for development
            if let execPath = ProcessInfo.processInfo.arguments.first {
                let execURL = URL(fileURLWithPath: execPath)
                let projectRoot = execURL.deletingLastPathComponent().deletingLastPathComponent().deletingLastPathComponent()
                popupPath = projectRoot.appendingPathComponent("target/release/popup_server").path
                if !FileManager.default.fileExists(atPath: popupPath) {
                    log(" Also cannot find popup at \(popupPath)")
                    NSApplication.shared.terminate(self)
                    return
                }
            }
        }
        
        log(" Starting with popup at \(popupPath)")
        
        // Launch Rust popup and show window on initial start
        launchRustPopup(showOnStart: true)
        
        // No continuous monitoring needed - we'll check when showing
        
        log(" Setup complete, waiting for events...")
    }
    
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        // Called when app is "launched" while already running
        NSLog("HookAnchor: applicationShouldHandleReopen called")
        log(" Reopen event received")
        showRustWindow()
        return true
    }
    
    @objc func handleURLEvent(_ event: NSAppleEventDescriptor, withReplyEvent replyEvent: NSAppleEventDescriptor) {
        // Handle hook:// URLs
        NSLog("HookAnchor: URL event received")
        showRustWindow()
    }
    
    func applicationDidBecomeActive(_ notification: Notification) {
        // Called when app becomes active (including via open -a)
        NSLog("HookAnchor: applicationDidBecomeActive")
        
        // Only show window if popup is already running (not on initial launch)
        if rustPopupProcess != nil && rustPopupProcess!.isRunning {
            NSLog("HookAnchor: Popup already running, showing window")
            showRustWindow()
        } else {
            NSLog("HookAnchor: Initial activation, not showing (will be handled by launch)")
        }
    }
    
    func applicationWillTerminate(_ notification: Notification) {
        // Clean up child process
        if let process = rustPopupProcess, process.isRunning {
            process.terminate()
        }
    }
    
    func launchRustPopup(showOnStart: Bool = false) {
        // Terminate any existing process
        if let process = rustPopupProcess, process.isRunning {
            process.terminate()
            process.waitUntilExit()
        }
        
        let task = Process()
        task.executableURL = URL(fileURLWithPath: popupPath)
        // No arguments needed - popup_server runs directly
        
        // Set up environment
        var environment = ProcessInfo.processInfo.environment
        environment["RUST_BACKTRACE"] = "1"
        task.environment = environment
        
        // Launch the process
        do {
            try task.run()
            rustPopupPID = task.processIdentifier
            rustPopupProcess = task
            log(" Launched popup with PID \(rustPopupPID)")
            
            // Only show window if explicitly requested (e.g., on initial app launch)
            if showOnStart {
                DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) { [weak self] in
                    self?.log("Showing initial window...")
                    self?.showRustWindow()
                }
            }
        } catch {
            log(" Failed to launch popup: \(error)")
        }
    }
    
    func showRustWindow() {
        NSLog("HookAnchor: showRustWindow called")
        // Prevent too-rapid repeated shows
        let now = Date()
        if now.timeIntervalSince(lastShowTime) < 0.1 {
            NSLog("HookAnchor: Skipping show - too rapid")
            return
        }
        lastShowTime = now
        
        NSLog("HookAnchor: Ensuring popup is running")
        // First ensure the popup is running
        ensurePopupRunning()
        
        // Send "show" command to popup_server via Unix socket
        let socketPath = "/tmp/hookanchor_popup.sock"
        
        DispatchQueue.global(qos: .userInitiated).async {
            let fileManager = FileManager.default
            
            // Check if socket exists
            if fileManager.fileExists(atPath: socketPath) {
                // Create a Unix domain socket
                let sock = socket(AF_UNIX, SOCK_STREAM, 0)
                if sock >= 0 {
                    var addr = sockaddr_un()
                    addr.sun_family = sa_family_t(AF_UNIX)
                    
                    // Copy socket path to sun_path
                    withUnsafeMutablePointer(to: &addr.sun_path.0) { ptr in
                        socketPath.withCString { cString in
                            strcpy(ptr, cString)
                        }
                    }
                    
                    // Connect to the socket
                    let connectResult = withUnsafePointer(to: &addr) { ptr in
                        ptr.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPtr in
                            connect(sock, sockaddrPtr, socklen_t(MemoryLayout<sockaddr_un>.size))
                        }
                    }
                    
                    if connectResult == 0 {
                        // Send "show" command
                        let command = "show\n"
                        command.withCString { cString in
                            send(sock, cString, strlen(cString), 0)
                        }
                        self.log(" Sent 'show' command to popup_server")
                        
                        // Verify window became visible after a short delay
                        DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) { [weak self] in
                            self?.verifyWindowVisible()
                        }
                    } else {
                        self.log(" Failed to connect to popup socket")
                    }
                    
                    // Close the socket
                    close(sock)
                }
            } else {
                self.log(" Popup socket not found at \(socketPath)")
            }
        }
        
        // Also try traditional activation methods as fallback
        if rustPopupPID > 0 {
            if let app = NSRunningApplication(processIdentifier: rustPopupPID) {
                app.activate(options: [.activateAllWindows])
                
                // Also try unhiding
                if app.isHidden {
                    app.unhide()
                }
            }
        }
    }
    
    func verifyWindowVisible() {
        guard rustPopupPID > 0 else { return }
        
        // Check if window is visible
        let options = CGWindowListOption(arrayLiteral: .excludeDesktopElements, .optionOnScreenOnly)
        guard let windowList = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] else {
            return
        }
        
        // Check if we have a visible window
        for window in windowList {
            if let ownerPID = window[kCGWindowOwnerPID as String] as? pid_t,
               ownerPID == rustPopupPID {
                // Window is visible, success!
                log(" ✓ Window is visible")
                return
            }
        }
        
        // Window not visible, log for debugging
        log(" ⚠️ Window not visible after show command")
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
                    log(" Found popup window \(windowNumber)")
                    
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
            log(" No windows found for PID \(rustPopupPID)")
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
        
        // Process has died, restart it (but don't show window)
        log(" Popup process died, restarting...")
        launchRustPopup(showOnStart: false)
    }
}

// Helper to check if we're running in an app bundle
extension Bundle {
    static var isAppBundle: Bool {
        return Bundle.main.bundleURL.pathExtension == "app"
    }
}