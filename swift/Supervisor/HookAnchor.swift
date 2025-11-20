import Cocoa
import Foundation
import Darwin
import Carbon

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
    
    // Get the path to the ha binary (HookAnchorCommand)
    // Tries bundled binary first, falls back to development path
    func getHaBinaryPath() -> String? {
        // Try bundled binary first (for distribution)
        if let bundlePath = Bundle.main.resourcePath {
            let bundledHaPath = "\(bundlePath)/../MacOS/ha"
            if FileManager.default.fileExists(atPath: bundledHaPath) {
                return bundledHaPath
            }
        }

        // Fall back to development path
        let devPath = "/Users/oblinger/ob/proj/HookAnchor/target/release/HookAnchorCommand"
        if FileManager.default.fileExists(atPath: devPath) {
            return devPath
        }

        return nil
    }

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

    func setupApplicationMenu() {
        // Create main menu bar
        let mainMenu = NSMenu()

        // Create HookAnchor menu
        let appMenuItem = NSMenuItem()
        mainMenu.addItem(appMenuItem)

        let appMenu = NSMenu()
        appMenuItem.submenu = appMenu

        // Add "Show HookAnchor" menu item
        // This exact text must be used in System Settings → Keyboard → App Shortcuts
        let showItem = NSMenuItem(
            title: "Show HookAnchor",
            action: #selector(menuShowHookAnchor),
            keyEquivalent: ""
        )
        appMenu.addItem(showItem)

        // Add separator
        appMenu.addItem(NSMenuItem.separator())

        // Add Quit menu item
        let quitItem = NSMenuItem(
            title: "Quit HookAnchor",
            action: #selector(NSApplication.terminate(_:)),
            keyEquivalent: "q"
        )
        appMenu.addItem(quitItem)

        // Set the menu bar
        NSApp.mainMenu = mainMenu

        log("Application menu created with 'Show HookAnchor' item")
    }

    @objc func menuShowHookAnchor() {
        log("Menu: Show HookAnchor triggered")
        showRustWindow()
    }

    func setupGlobalHotkey() {
        // Read hotkey configuration from config file
        let homeDir = FileManager.default.homeDirectoryForCurrentUser.path
        let configPath = "\(homeDir)/.config/hookanchor/config.yaml"

        var keyCode: UInt32 = 49 // Space key by default
        var modifiers: UInt32 = UInt32(optionKey) // Option modifier by default

        // Try to read config file
        if let configData = try? String(contentsOfFile: configPath, encoding: .utf8) {
            // Parse global_hotkey line (simple parsing for "global_hotkey: Option+Space")
            let lines = configData.components(separatedBy: .newlines)
            for line in lines {
                if line.hasPrefix("global_hotkey:") {
                    let value = line.replacingOccurrences(of: "global_hotkey:", with: "").trimmingCharacters(in: .whitespaces)
                    let parts = value.components(separatedBy: "+")

                    // Parse modifiers and key
                    var parsedModifiers: UInt32 = 0
                    var parsedKey: UInt32?

                    for part in parts {
                        let trimmed = part.trimmingCharacters(in: .whitespaces)
                        switch trimmed.lowercased() {
                        case "option", "alt":
                            parsedModifiers |= UInt32(optionKey)
                        case "command", "cmd":
                            parsedModifiers |= UInt32(cmdKey)
                        case "control", "ctrl":
                            parsedModifiers |= UInt32(controlKey)
                        case "shift":
                            parsedModifiers |= UInt32(shiftKey)
                        case "space":
                            parsedKey = 49
                        case "return", "enter":
                            parsedKey = 36
                        case "`", "backtick":
                            parsedKey = 50
                        default:
                            break
                        }
                    }

                    if parsedKey != nil {
                        keyCode = parsedKey!
                        modifiers = parsedModifiers
                    }
                    break
                }
            }
        }

        // Create hotkey ID
        let hotKeyID = EventHotKeyID(signature: 0x484B4152, id: 1) // 'HKAR'

        // Register the hotkey
        let registerResult = RegisterEventHotKey(
            keyCode,
            modifiers,
            hotKeyID,
            GetEventDispatcherTarget(),
            0,
            &hotKeyRef
        )

        if registerResult == noErr {
            log("Global hotkey registered successfully (keyCode=\(keyCode), modifiers=\(modifiers))")
        } else {
            log("Failed to register global hotkey: \(registerResult)")
        }

        // Install event handler
        let eventSpec = [EventTypeSpec(eventClass: OSType(kEventClassKeyboard),
                                       eventKind: UInt32(kEventHotKeyPressed))]

        InstallEventHandler(GetEventDispatcherTarget(),
                           hotKeyEventHandler,
                           1,
                           eventSpec,
                           UnsafeMutableRawPointer(Unmanaged.passUnretained(self).toOpaque()),
                           &eventHandler)
    }

    var rustPopupProcess: Process?
    var rustPopupPID: pid_t = 0
    var popupPath: String = ""
    var lastShowTime: Date = Date.distantPast
    // Removed timer - no need for continuous monitoring

    // Global hotkey registration
    var eventHandler: EventHandlerRef?
    var hotKeyRef: EventHotKeyRef?
    
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

        // DO NOT call performHealthCheckAndRestart() here!
        // That function runs `ha --restart`, which creates circular dependency:
        //   1. User runs `ha --restart`
        //   2. Which starts supervisor
        //   3. Supervisor calls performHealthCheckAndRestart()
        //   4. Which runs `ha --restart` AGAIN
        //   5. Result: duplicate popup_server processes
        //
        // Instead, let `ha --restart` handle everything.
        // The supervisor just ensures command server is available.

        // Ensure command server is running before we do anything else
        startCommandServerIfNeeded()

        // Set up supervisor control socket for lifecycle management
        setupSupervisorSocket()

        // Set up application menu (enables keyboard shortcuts via System Settings)
        setupApplicationMenu()

        // Set up global hotkey (Option+Space by default)
        setupGlobalHotkey()

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

        // Read config to determine if we should show popup on startup
        let homeDir = FileManager.default.homeDirectoryForCurrentUser.path
        let configPath = "\(homeDir)/.config/hookanchor/config.yaml"
        var showPopupOnStartup = false // Default to false

        if let configData = try? String(contentsOfFile: configPath, encoding: .utf8) {
            let lines = configData.components(separatedBy: .newlines)
            for line in lines {
                if line.contains("show_popup_on_startup:") {
                    let value = line.replacingOccurrences(of: "show_popup_on_startup:", with: "")
                        .trimmingCharacters(in: .whitespaces)
                        .lowercased()
                    showPopupOnStartup = (value == "true")
                    self.log("Config: show_popup_on_startup = \(showPopupOnStartup)")
                    break
                }
            }
        }

        // Wait a moment to see if a URL event arrives (cold launch URL handling)
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
            guard let self = self else { return }

            // Check if we have a pending URL
            if let url = self.pendingURL {
                self.log("Processing pending URL from launch: \(url)")
                self.processURL(url)
                self.pendingURL = nil
            } else if showPopupOnStartup {
                // No URL received, and config says to show popup on startup
                self.log("No URL received at launch, showing window (show_popup_on_startup=true)")
                self.showRustWindow()
            } else {
                // No URL and config says NOT to show popup on startup
                self.log("No URL received at launch, staying hidden (show_popup_on_startup=false)")
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

            // Pass the full URL to the ha CLI for URL handling
            // Using --hook to route through URL handler for prefix support (p/, a/, x/)
            if let haPath = getHaBinaryPath() {
                // Execute ha with the --hook option to handle the full URL
                let task = Process()
                task.executableURL = URL(fileURLWithPath: haPath)
                task.arguments = ["--hook", urlString]
                
                // Don't wait for output or termination - fire and forget
                task.standardOutput = nil
                task.standardError = nil
                task.terminationHandler = nil
                
                // Run in background - we don't need to wait for completion
                do {
                    try task.run()
                    log("Dispatched URL '\(urlString)' to ha --hook (async)")
                    // Don't wait for the process to complete - return immediately
                } catch {
                    log("Failed to dispatch URL to ha CLI: \(error)")
                    // Don't show window on error - just log it
                }
            } else {
                log("ha CLI not found, falling back to show window")
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
        let startTime = Date()
        NSLog("HookAnchor: [SHOW CHAIN] Step 1: showRustWindow() called")
        log("⏱️ [SHOW] Starting popup display sequence")

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
        let beforeEnsure = Date()
        // First ensure the popup is running
        ensurePopupRunning()
        let ensureDuration = Date().timeIntervalSince(beforeEnsure) * 1000
        log(String(format: "⏱️ [SHOW] ensurePopupRunning: %.2fms", ensureDuration))

        // Send "show" command to popup_server via Unix socket
        let socketPath = getPopupSocketPath()

        let beforeSocket = Date()
        DispatchQueue.global(qos: .userInitiated).async {
            let socketStartTime = Date()
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
                        let connectDuration = Date().timeIntervalSince(socketStartTime) * 1000
                        self.log(String(format: "⏱️ [SHOW] Socket connect: %.2fms", connectDuration))

                        // Send "show" command
                        let command = "show\n"
                        let beforeSend = Date()
                        command.withCString { cString in
                            send(sock, cString, strlen(cString), 0)
                        }
                        let sendDuration = Date().timeIntervalSince(beforeSend) * 1000
                        self.log(String(format: "⏱️ [SHOW] Socket send: %.2fms", sendDuration))
                        
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

    func performHealthCheckAndRestart() {
        log("⚕️ Performing health check on servers...")

        guard let haPath = getHaBinaryPath() else {
            log("❌ Health check failed: ha binary not found")
            return
        }

        // Run `ha --restart` to ensure both servers are fresh and running
        let restartTask = Process()
        restartTask.executableURL = URL(fileURLWithPath: haPath)
        restartTask.arguments = ["--restart"]

        // Capture output for logging
        let outputPipe = Pipe()
        let errorPipe = Pipe()
        restartTask.standardOutput = outputPipe
        restartTask.standardError = errorPipe

        do {
            try restartTask.run()
            restartTask.waitUntilExit()

            // Read and log output
            let outputData = outputPipe.fileHandleForReading.readDataToEndOfFile()
            let errorData = errorPipe.fileHandleForReading.readDataToEndOfFile()

            if let output = String(data: outputData, encoding: .utf8), !output.isEmpty {
                log("Health check output: \(output.trimmingCharacters(in: .whitespacesAndNewlines))")
            }

            if let error = String(data: errorData, encoding: .utf8), !error.isEmpty {
                log("Health check stderr: \(error.trimmingCharacters(in: .whitespacesAndNewlines))")
            }

            if restartTask.terminationStatus == 0 {
                log("✅ Health check complete: servers restarted successfully")
            } else {
                log("⚠️  Health check: restart command returned status \(restartTask.terminationStatus)")
            }
        } catch {
            log("❌ Health check failed: \(error)")
        }
    }

    func ensurePopupRunning(skipProcessCheck: Bool = false) {
        log("🔍 ensurePopupRunning: Checking if popup is running...")

        // First check if our known process is still running
        if let process = rustPopupProcess, process.isRunning {
            log("  ✓ Popup process already running (tracked)")
            return
        }

        // FAST CHECK: If the socket file exists, the server is running (avoid expensive pgrep)
        let socketPath = getPopupSocketPath()
        log("  📁 Checking socket: \(socketPath)")
        if FileManager.default.fileExists(atPath: socketPath) {
            log("  ✓ Socket exists, popup is running")
            return
        }

        // Socket doesn't exist - do the expensive process check only as last resort
        // Skip this check if we just stopped the process (avoid finding zombie processes)
        if !skipProcessCheck {
            log("  ⚙️  Socket not found, checking with pgrep...")
            let checkTask = Process()
            checkTask.executableURL = URL(fileURLWithPath: "/bin/sh")
            checkTask.arguments = ["-c", "pgrep -f 'popup_server' > /dev/null 2>&1"]

            do {
                try checkTask.run()
                checkTask.waitUntilExit()

                let status = checkTask.terminationStatus
                log("  ⚙️  pgrep returned status: \(status)")

                if status == 0 {
                    log("  ⚠️  pgrep found popup_server but socket missing (race condition)")
                    // A popup_server is running but socket doesn't exist yet (race condition)
                    // Wait a moment for socket to be created
                    Thread.sleep(forTimeInterval: 0.01)
                    return
                } else {
                    log("  ℹ️  pgrep found no popup_server processes")
                }
            } catch {
                log("  ⚠️  pgrep check failed: \(error)")
                // Ignore errors, proceed to launch
            }
        } else {
            log("  ⏭️  Skipping pgrep check (just stopped process)")
        }

        // No popup_server found anywhere, restart it (but don't show window)
        log("  🚀 No popup found, launching...")
        launchRustPopup(showOnStart: false)
    }
    
    func startCommandServerIfNeeded() {
        // Check if command server is already running
        let checkTask = Process()
        checkTask.executableURL = URL(fileURLWithPath: "/bin/sh")
        checkTask.arguments = ["-c", "pgrep -f 'HookAnchorCommand --start-server' > /dev/null 2>&1"]
        
        do {
            try checkTask.run()
            checkTask.waitUntilExit()
            
            if checkTask.terminationStatus != 0 {
                // Server not running, start it
                log("Command server not running, starting it...")

                if let haPath = getHaBinaryPath() {
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

    // MARK: - Socket Path Helpers

    /// Get the secure popup socket path in user's config directory
    func getPopupSocketPath() -> String {
        let configDir = FileManager.default.homeDirectoryForCurrentUser
            .appendingPathComponent(".config")
            .appendingPathComponent("hookanchor")
        return configDir.appendingPathComponent("popup.sock").path
    }

    // MARK: - Supervisor Control Socket

    /// Set up the supervisor control socket for lifecycle management
    /// Creates a secure Unix socket at ~/.config/hookanchor/supervisor.sock
    /// with permissions 0600 (owner read/write only)
    func setupSupervisorSocket() {
        DispatchQueue.global(qos: .userInitiated).async {
            let configDir = FileManager.default.homeDirectoryForCurrentUser
                .appendingPathComponent(".config")
                .appendingPathComponent("hookanchor")

            // Ensure config directory exists
            try? FileManager.default.createDirectory(at: configDir, withIntermediateDirectories: true)

            let socketPath = configDir.appendingPathComponent("supervisor.sock").path

            // Remove any existing socket
            try? FileManager.default.removeItem(atPath: socketPath)

            self.log("Setting up supervisor socket at: \(socketPath)")

            // Create Unix domain socket
            let sock = socket(AF_UNIX, SOCK_STREAM, 0)
            guard sock >= 0 else {
                self.log("❌ Failed to create supervisor socket")
                return
            }

            var addr = sockaddr_un()
            addr.sun_family = sa_family_t(AF_UNIX)

            // Copy socket path to sun_path
            withUnsafeMutablePointer(to: &addr.sun_path.0) { ptr in
                socketPath.withCString { cString in
                    strcpy(ptr, cString)
                }
            }

            // Bind the socket
            let bindResult = withUnsafePointer(to: &addr) { ptr in
                ptr.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPtr in
                    Darwin.bind(sock, sockaddrPtr, socklen_t(MemoryLayout<sockaddr_un>.size))
                }
            }

            guard bindResult == 0 else {
                self.log("❌ Failed to bind supervisor socket: \(String(cString: strerror(errno)))")
                close(sock)
                return
            }

            // Set restrictive permissions (0600 - owner read/write only)
            chmod(socketPath, 0o600)

            // Listen for connections
            guard listen(sock, 5) == 0 else {
                self.log("❌ Failed to listen on supervisor socket")
                close(sock)
                return
            }

            self.log("✅ Supervisor socket listening (secure mode 0600)")

            // Accept connections in a loop
            while true {
                var clientAddr = sockaddr_un()
                var clientAddrLen = socklen_t(MemoryLayout<sockaddr_un>.size)

                let clientSock = withUnsafeMutablePointer(to: &clientAddr) { ptr in
                    ptr.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPtr in
                        accept(sock, sockaddrPtr, &clientAddrLen)
                    }
                }

                guard clientSock >= 0 else {
                    continue
                }

                // Handle client in separate thread
                DispatchQueue.global(qos: .userInitiated).async {
                    self.handleSupervisorCommand(clientSocket: clientSock)
                }
            }
        }
    }

    /// Handle a supervisor command received via socket
    func handleSupervisorCommand(clientSocket: Int32) {
        defer { close(clientSocket) }

        // Read command from socket
        var buffer = [UInt8](repeating: 0, count: 1024)
        let bytesRead = read(clientSocket, &buffer, buffer.count)

        guard bytesRead > 0 else {
            return
        }

        let command = String(bytes: buffer[..<bytesRead], encoding: .utf8)?.trimmingCharacters(in: .whitespacesAndNewlines) ?? ""

        self.log("📡 Received supervisor command: \(command)")

        let response: String

        switch command {
        case "start":
            response = handleStartCommand()
        case "stop":
            response = handleStopCommand()
        case "restart":
            response = handleRestartCommand()
        default:
            response = "ERROR: Unknown command '\(command)'. Valid commands: start, stop, restart"
        }

        // Send response back to client
        response.withCString { cString in
            write(clientSocket, cString, strlen(cString))
        }
    }

    /// Handle 'start' command - ensure servers are running (idempotent)
    /// - Parameter skipProcessCheck: Skip pgrep check (use when we just stopped processes)
    func handleStartCommand(skipProcessCheck: Bool = false) -> String {
        log("🚀 START command: Ensuring servers are running...")

        // Start popup server if not running
        ensurePopupRunning(skipProcessCheck: skipProcessCheck)

        // Start command server if not running
        startCommandServerIfNeeded()

        // Give servers a moment to initialize
        Thread.sleep(forTimeInterval: 0.5)

        log("✅ START complete: Servers ensured running")
        return "OK: Servers started"
    }

    /// Handle 'stop' command - stop all servers (idempotent)
    func handleStopCommand() -> String {
        log("🛑 STOP command: Stopping all servers...")

        var stoppedCount = 0

        // Stop popup server
        if let process = rustPopupProcess, process.isRunning {
            process.terminate()
            process.waitUntilExit()
            rustPopupProcess = nil
            rustPopupPID = 0
            log("  ✓ Stopped popup server")
            stoppedCount += 1
        } else {
            // Try killing via process name as fallback
            let killPopup = Process()
            killPopup.executableURL = URL(fileURLWithPath: "/usr/bin/killall")
            killPopup.arguments = ["popup_server"]
            try? killPopup.run()
            killPopup.waitUntilExit()
            if killPopup.terminationStatus == 0 {
                log("  ✓ Stopped popup server (via killall)")
                stoppedCount += 1
            }
        }

        // Clean up stale socket files after stopping servers
        let socketPath = getPopupSocketPath()
        if FileManager.default.fileExists(atPath: socketPath) {
            try? FileManager.default.removeItem(atPath: socketPath)
            log("  🧹 Cleaned up popup socket")
        }

        // Stop command server
        if let haPath = getHaBinaryPath() {
            let stopCmd = Process()
            stopCmd.executableURL = URL(fileURLWithPath: haPath)
            stopCmd.arguments = ["--stop-server"]

            do {
                try stopCmd.run()
                stopCmd.waitUntilExit()
                if stopCmd.terminationStatus == 0 {
                    log("  ✓ Stopped command server")
                    stoppedCount += 1
                }
            } catch {
                log("  ⚠ Failed to stop command server: \(error)")
            }
        }

        log("✅ STOP complete: Stopped \(stoppedCount) server(s)")
        return "OK: Stopped \(stoppedCount) server(s)"
    }

    /// Handle 'restart' command - stop then start servers
    func handleRestartCommand() -> String {
        log("🔄 RESTART command: Restarting all servers...")

        // Stop first
        let _ = handleStopCommand()

        // Wait briefly for processes to fully terminate (shorter delay is fine now)
        Thread.sleep(forTimeInterval: 0.5)

        // Start fresh - skip process check since we just stopped (avoid finding zombie processes)
        let _ = handleStartCommand(skipProcessCheck: true)

        log("✅ RESTART complete: All servers restarted with fresh binaries")
        return "OK: Servers restarted"
    }
}

// Helper to check if we're running in an app bundle
extension Bundle {
    static var isAppBundle: Bool {
        return Bundle.main.bundleURL.pathExtension == "app"
    }
}

// Global hotkey event handler (C callback)
private func hotKeyEventHandler(
    eventHandlerCall: EventHandlerCallRef?,
    event: EventRef?,
    userData: UnsafeMutableRawPointer?
) -> OSStatus {
    guard let userData = userData else {
        return OSStatus(eventNotHandledErr)
    }

    // Get the AppDelegate instance from userData
    let appDelegate = Unmanaged<AppDelegate>.fromOpaque(userData).takeUnretainedValue()

    // Extract hotkey ID from event
    var hotKeyID = EventHotKeyID()
    let error = GetEventParameter(
        event,
        UInt32(kEventParamDirectObject),
        UInt32(typeEventHotKeyID),
        nil,
        MemoryLayout<EventHotKeyID>.size,
        nil,
        &hotKeyID
    )

    if error != noErr {
        return error
    }

    // Verify it's our hotkey
    if hotKeyID.signature == 0x484B4152 && hotKeyID.id == 1 {
        // Trigger the show window action
        DispatchQueue.main.async {
            appDelegate.showRustWindow()
        }
        return noErr
    }

    return OSStatus(eventNotHandledErr)
}