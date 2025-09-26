# TextCapture API Reference

Compact overview of all modules, classes, and methods in the system.

# Main

## TextCaptureApp
class : App // Main application entry point
- body -> Scene // SwiftUI scene definition

# Models

## TextAnchor
class // Maintains a position in text that survives edits
- init(position, buffer) // Create anchor at specific position
- text -> String // Get current text at anchor
- position -> Int // Get current position
- update(newText, smart) // Replace text at anchor position
- delete() // Remove anchor
- adjustPosition(changePosition, lengthDelta) // Adjust for external edits

## TextBuffer
protocol // Interface for text buffer operations
- getAnchor() -> TextAnchor // Get anchor at current position

## TextBufferView
class : TextBuffer // SwiftUI binding-based text buffer
- init(text, cursorPosition) // Wrap SwiftUI text bindings
- getAnchor() -> TextAnchor // Create anchor at cursor
- getContext(position, length) -> (before, after) // Get surrounding text
- replaceText(position, length, newText) // Update text at position
- cleanupAnchors() // Remove deallocated anchors

# Services

## SpeechEngine
protocol // Interface for speech recognition engines
- init(textBuffer) // Initialize with buffer
- isListening -> Bool // Current listening state
- requestAuthorization() // Request speech permissions
- start() // Begin recognition
- stop() // End recognition

## AppleSpeechEngine
class : SpeechEngine // Apple Speech framework implementation
- init(textBuffer) // Initialize with buffer
- isListening -> Bool // Current state
- requestAuthorization() // Request permissions
- start() // Start continuous recognition
- stop() // Stop and cleanup
- startAudioEngine() // Initialize audio pipeline
- startRecognitionSession() // Begin new recognition
- gatherContextualVocabulary() -> [String] // Get app names for context
- cleanupAudioSession() // Release audio resources
- scheduleRestart(delay) // Restart after delay
- requestMicrophonePermission() // Request mic access

## ActionIdentifier
enum // Voice command identifiers
- beginDictation // Start speech recognition
- endDictation // Stop speech recognition
- deleteLastText // Delete previous text
- clearAllText // Clear all text
- sendText // Send to target app
- sendTextWithReturn // Send with return key

## Actions
class : ObservableObject // Central action execution system
- shared -> Actions // Singleton instance
- beginDictationHandler // Callback for begin
- endDictationHandler // Callback for end
- executeAction(identifier) // Execute any action
- beginDictation() // Start dictation
- endDictation() // Stop dictation
- deleteLastText() // Delete last text
- clearAllText() // Clear all text
- sendText() // Send to app
- sendTextWithReturn() // Send with return

## ActionDispatcher
class // Intercepts text for voice commands
- init() // Initialize dispatcher
- processText(text) -> String // Find and execute triggers
- loadTriggerWords() // Load from config

# Views

## ContentView
class : View // Main application view
- text // Current text content
- isTranscribing // Recognition state
- cursorPosition // Cursor location
- selectionRange // Selected text range
- isTextEditorFocused // Focus state
- speechEngine // Recognition engine
- textBuffer // Text buffer
- body -> View // SwiftUI view
- toggleTranscription() // Start/stop recognition
- setupWindow() // Configure window
- parseWindowFrame(frameString) -> NSRect // Parse position string
- setupFocusTracking(window) // Track app focus
- setupWindowObserver(window) // Monitor window changes
- sendTextToTargetApp(text) // Send to other app
- typeTextToProcess(text, pid) // Type to process

## TrackedTextEditor
class : NSViewRepresentable // Text editor with cursor tracking
- init(text, cursorPosition, selectionRange) // Create editor
- makeNSView(context) -> NSScrollView // Create AppKit view
- updateNSView(nsView, context) // Update view
- makeCoordinator() -> Coordinator // Create delegate

## Coordinator
class : NSObject, NSTextViewDelegate // Text view delegate
- init(text, cursorPosition, selectionRange) // Initialize
- textDidChange(notification) // Handle text changes
- textViewDidChangeSelection(notification) // Handle selection

## CustomTextEditor
class : View // SwiftUI text editor wrapper
- text // Bound text
- cursorPosition // Bound cursor
- selectionRange // Bound selection
- body -> View // SwiftUI view

## CustomTextView
class : NSTextView // Custom AppKit text view
- init() // Initialize view
- init?(coder) // Decode from storyboard
- insertNewline(sender) // Handle return key
- insertTab(sender) // Handle tab key
- deleteBackward(sender) // Handle delete key

# Utilities

## Logger
class // Centralized logging system
- shared -> Logger // Singleton instance
- init() // Private initializer
- log(message, file, line, function) // Standard log
- detail(message, file, line, function) // Detailed log
- error(message, file, line, function) // Error log
- formatMessage(level, message, file, line, function) -> String // Format output
- write(formattedMessage) // Write to console

## Config
class : Codable // Application configuration
- recognizer // Speech engine type
- appleOnDevice // Use local recognition
- openaiApiKey // OpenAI API key
- whisperChunkDuration // Audio chunk size
- whisperSilenceThreshold // Silence detection
- whisperMaxSilence // Max silence duration
- windowFrame // Window position/size
- alwaysOnTop // Window stays on top
- triggerWords // Voice command triggers
- load() -> Config // Load from file
- parseConfigFile(url) -> Config // Parse YAML file
- parseYAML(content) -> [String: Any] // Parse YAML string
- parseYAMLValue(value) -> Any // Parse single value
- extractValue(dict, keys) -> Any // Get nested value