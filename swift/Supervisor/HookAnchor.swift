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
    
    // Store any URL that arrives before we're ready to handle it
    var pendingURL: String?
    
    // Track URL event frequency for debugging
    var recentURLEvents: [Date] = []
    var lastURLEventLog: Date = Date.distantPast
    
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
        let homeDir = FileManager.default.homeDirectoryForCurrentUser.path
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
    
    func applicationWillFinishLaunching(_ notification: Notification) {
        // Register for URL events BEFORE launch completes - this is critical!
        NSAppleEventManager.shared().setEventHandler(
            self,
            andSelector: #selector(handleURLEvent(_:withReplyEvent:)),
            forEventClass: AEEventClass(kInternetEventClass),
            andEventID: AEEventID(kAEGetURL)
        )
        log("Early URL handler registration complete")
    }
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        log("Starting up...")
        log("URL event handler already registered")
        
        // Ensure command server is running before we do anything else
        startCommandServerIfNeeded()
        
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
        
        // Launch Rust popup but DON'T show window immediately
        // We'll wait briefly to see if we were launched by a URL
        launchRustPopup(showOnStart: false)
        
        // No continuous monitoring needed - we'll check when showing
        
        log(" Setup complete, waiting for events...")
        
        // Wait a moment to see if a URL event arrives (cold launch URL handling)
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
            guard let self = self else { return }
            
            // Check if we have a pending URL
            if let url = self.pendingURL {
                self.log("Processing pending URL from launch: \(url)")
                self.processURL(url)
                self.pendingURL = nil
            } else {
                // No URL received, show the window (normal launch)
                self.log("No URL received at launch, showing window")
                self.showRustWindow()
            }
        }
    }
    
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        // Called when app is "launched" while already running (e.g., from caps lock)
        NSLog("HookAnchor: ===== CAPS LOCK TRIGGER CHAIN START =====")
        NSLog("HookAnchor: applicationShouldHandleReopen called")
        log(" [CAPS LOCK] Reopen event received - starting popup display chain")
        showRustWindow()
        NSLog("HookAnchor: ===== CAPS LOCK TRIGGER CHAIN END =====")
        return true
    }
    
    @objc func handleURLEvent(_ event: NSAppleEventDescriptor, withReplyEvent replyEvent: NSAppleEventDescriptor) {
        let startTime = Date()
        
        // Track event frequency
        recentURLEvents.append(startTime)
        // Keep only events from last 5 seconds
        let fiveSecondsAgo = Date().addingTimeInterval(-5)
        recentURLEvents = recentURLEvents.filter { $0 > fiveSecondsAgo }
        
        // Log warning if we're getting bombarded with events
        if recentURLEvents.count > 10 {
            let timeSinceLastLog = startTime.timeIntervalSince(lastURLEventLog)
            if timeSinceLastLog > 1.0 { // Only log this warning once per second
                log("⚠️ URL_EVENT_FLOOD: \(recentURLEvents.count) events in last 5 seconds!")
                lastURLEventLog = startTime
            }
        }
        
        // Handle hook:// URLs by extracting the command and executing it
        guard let urlString = event.paramDescriptor(forKeyword: keyDirectObject)?.stringValue else {
            log("URL_EVENT: Received but no URL found")
            return
        }
        
        // Log with timestamp and caller info if possible
        let processName = ProcessInfo.processInfo.processName
        let callerPID = event.attributeDescriptor(forKeyword: keySenderPIDAttr)?.int32Value ?? -1
        
        log("URL_EVENT_START: '\(urlString)' from PID:\(callerPID)")
        
        // If we're not fully initialized yet, store the URL for later
        if rustPopupProcess == nil {
            log("URL_EVENT: App not ready, storing URL for later processing")
            pendingURL = urlString
            let elapsed = Date().timeIntervalSince(startTime) * 1000
            log("URL_EVENT_END: Deferred in \(String(format: "%.2f", elapsed))ms")
            return
        }
        
        // Process the URL immediately
        processURL(urlString)
        
        let elapsed = Date().timeIntervalSince(startTime) * 1000
        log("URL_EVENT_END: Processed in \(String(format: "%.2f", elapsed))ms")
    }
    
    func processURL(_ urlString: String) {
        // Extract the command from hook://command format
        if urlString.hasPrefix("hook://") {
            let command = String(urlString.dropFirst(7)) // Remove "hook://"
            
            log("Processing hook URL with command: '\(command)'")
            
            // Pass the command to the ha CLI for execution
            // Using -x to execute the top match for the command
            let haPath = "/Users/oblinger/ob/proj/HookAnchor/target/release/ha"
            
            // Check if ha binary exists
            if FileManager.default.fileExists(atPath: haPath) {
                // Execute ha with the -x option to execute the command
                let task = Process()
                task.executableURL = URL(fileURLWithPath: haPath)
                task.arguments = ["-x", command]
                
                // Don't wait for output or termination - fire and forget
                task.standardOutput = nil
                task.standardError = nil
                task.terminationHandler = nil
                
                // Run in background - we don't need to wait for completion
                do {
                    try task.run()
                    log("Dispatched command '\(command)' to ha CLI (async)")
                    // Don't wait for the process to complete - return immediately
                } catch {
                    log("Failed to dispatch command to ha CLI: \(error)")
                    // Don't show window on error - just log it
                }
            } else {
                log("ha CLI not found at \(haPath), falling back to show window")
                // Fallback: show the window if ha isn't available
                showRustWindow()
            }
        } else {
            log("URL doesn't start with hook://, ignoring: \(urlString)")
        }
    }
    
    func applicationDidBecomeActive(_ notification: Notification) {
        // Called when app becomes active (including via open -a)
        NSLog("HookAnchor: applicationDidBecomeActive")
        
        // Don't show window on activation - this gets called for URL events too!
        // The window should only be shown explicitly via:
        // 1. Initial launch (handled in applicationDidFinishLaunching)
        // 2. Reopen event (handled in applicationShouldHandleReopen)
        // 3. Direct user action (not just becoming active)
        
        // Just log that we became active
        NSLog("HookAnchor: App became active (no action taken)")
    }
    
    func applicationWillTerminate(_ notification: Notification) {
        // Clean up child process
        if let process = rustPopupProcess, process.isRunning {
            process.terminate()
        }
    }
    
    // Alternative URL handling method - some macOS versions use this
    func application(_ application: NSApplication, open urls: [URL]) {
        log("application:open:urls called with \(urls.count) URLs")
        for url in urls {
            log("Received URL via application:open: \(url.absoluteString)")
            if rustPopupProcess == nil {
                pendingURL = url.absoluteString
            } else {
                processURL(url.absoluteString)
            }
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
            
            // Note: Window showing is now handled separately in applicationDidFinishLaunching
            // to allow checking for URL events first
        } catch {
            log(" Failed to launch popup: \(error)")
        }
    }
    
    func showRustWindow() {
        NSLog("HookAnchor: [SHOW CHAIN] Step 1: showRustWindow() called")
        log("[SHOW] Initiating popup display sequence")
        
        // Prevent too-rapid repeated shows
        let now = Date()
        let timeSinceLast = now.timeIntervalSince(lastShowTime)
        if timeSinceLast < 0.1 {
            NSLog("HookAnchor: [SHOW CHAIN] BLOCKED: Too rapid (\(timeSinceLast)s < 0.1s)")
            log("[SHOW] Skipped - too rapid trigger")
            return
        }
        lastShowTime = now
        
        NSLog("HookAnchor: [SHOW CHAIN] Step 2: Checking popup_server process")
        log("[SHOW] Ensuring popup_server is running")
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
                        let error = String(cString: strerror(errno))
                        NSLog("HookAnchor: [SOCKET] ERROR: Connection failed - \(error)")
                        self.log(" [ERROR] Failed to connect to popup socket: \(error)")
                    }
                    
                    // Close the socket
                    close(sock)
                }
            } else {
                NSLog("HookAnchor: [SOCKET] ERROR: Socket file not found at \(socketPath)")
                self.log(" [ERROR] Popup socket not found at \(socketPath)")
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
        // First check if our known process is still running
        if let process = rustPopupProcess, process.isRunning {
            // Process is still running
            return
        }
        
        // Check if ANY popup_server process is running (might have been started by ha --restart)
        let checkTask = Process()
        checkTask.executableURL = URL(fileURLWithPath: "/bin/sh")
        checkTask.arguments = ["-c", "pgrep -f 'popup_server' > /dev/null 2>&1"]
        
        do {
            try checkTask.run()
            checkTask.waitUntilExit()
            
            if checkTask.terminationStatus == 0 {
                // A popup_server is already running somewhere
                log(" Popup_server already running (launched externally)")
                return
            }
        } catch {
            // Ignore errors, proceed to launch
        }
        
        // No popup_server found anywhere, restart it (but don't show window)
        log(" Popup process died, restarting...")
        launchRustPopup(showOnStart: false)
    }
    
    func startCommandServerIfNeeded() {
        // Check if command server is already running
        let checkTask = Process()
        checkTask.executableURL = URL(fileURLWithPath: "/bin/sh")
        checkTask.arguments = ["-c", "pgrep -f 'ha --start-server-daemon' > /dev/null 2>&1"]
        
        do {
            try checkTask.run()
            checkTask.waitUntilExit()
            
            if checkTask.terminationStatus != 0 {
                // Server not running, start it
                log("Command server not running, starting it...")
                
                let haPath = "/Users/oblinger/ob/proj/HookAnchor/target/release/ha"
                if FileManager.default.fileExists(atPath: haPath) {
                    let startTask = Process()
                    startTask.executableURL = URL(fileURLWithPath: haPath)
                    startTask.arguments = ["--start-server"]
                    
                    do {
                        try startTask.run()
                        log("Started command server")
                    } catch {
                        log("Failed to start command server: \(error)")
                    }
                } else {
                    log("ha binary not found, cannot start command server")
                }
            } else {
                log("Command server already running")
            }
        } catch {
            log("Error checking command server status: \(error)")
        }
    }
}

// Helper to check if we're running in an app bundle
extension Bundle {
    static var isAppBundle: Bool {
        return Bundle.main.bundleURL.pathExtension == "app"
    }
}