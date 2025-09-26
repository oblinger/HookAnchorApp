# Product Requirements Document: Text Capture App

## Overview
An always-on-top text capture application for macOS that continuously transcribes audio from a specific microphone and provides seamless text routing to target applications.



hh:mm:ss.nnn [-> hh:mm:ss.nnn]        white 34 -> white 33       make->miss



## Core Features

### 1. Always-On-Top Text Window
- Floating window that remains above all other applications
- Editable text area for real-time transcription display
- User can directly edit captured text within the window

### 2. Continuous Audio Capture
- Monitors and transcribes audio from a designated microphone
- Real-time speech-to-text conversion
- Appends transcribed text to the edit window continuously

### 3. Smart Text Routing
- Tab/Enter triggers push text to currently designated output application
- Configurable output destination selection
- Maintains focus on capture window while routing text

### 4. Control Mechanisms

#### Keyboard Shortcuts
- Quick switching between target applications
- Text manipulation commands
- System control functions

#### UI Buttons
- Visual controls for output destination selection
- Quick access to common functions
- Status indicators for current target

#### Voice Commands
- Spoken control commands recognized within text stream
- System behavior modulation via voice
- Context-aware command processing

## Technical Requirements

### Speech Recognition
- **Local Processing**: Apple Speech framework for low-latency, privacy-focused transcription
- **Cloud Processing**: Apple Speech cloud services for enhanced accuracy when needed
- **Alternative**: OpenAI Whisper API integration for advanced transcription capabilities

### Platform
- macOS native application
- Integration with macOS accessibility and input APIs
- Support for multiple microphone sources

## Advanced Behaviors
- Intelligent text routing based on context
- Command detection and execution within speech stream
- Multiple output mode configurations
- Text transformation capabilities before routing

## Recommended Technology Stack

For building this application, **Swift** would be the most appropriate language for the following reasons:

### Primary Choice: Swift
1. **Native macOS Integration**: Direct access to all macOS APIs including:
   - NSWindow for always-on-top functionality
   - Speech framework for transcription
   - Accessibility APIs for text injection
   - Event monitoring and keyboard shortcuts

2. **Performance**: Native compilation and minimal overhead for real-time audio processing

3. **Speech Framework Access**: Built-in support for both on-device and cloud-based Apple Speech recognition

4. **UI Development**: SwiftUI or AppKit for creating the floating window interface

### Alternative Considerations:
- **Electron + Node.js**: If cross-platform support becomes important, though with limitations on system integration
- **Python + PyObjC**: Rapid prototyping option with access to macOS APIs, but with performance trade-offs

### Recommended Architecture:
```
Swift (Core App)
├── SwiftUI/AppKit (UI)
├── Speech Framework (Apple Speech)
├── AVFoundation (Audio Capture)
├── Accessibility API (Text Injection)
└── Optional: Whisper API Client (REST)
```

## Key Challenges to Address
1. Maintaining window focus while injecting text into other applications
2. Handling multiple audio input sources reliably
3. Distinguishing between dictation text and voice commands
4. Ensuring low-latency text routing
5. Managing permissions for accessibility and microphone access

## Success Metrics
- Sub-second transcription latency
- 95%+ transcription accuracy
- Seamless text injection without focus loss
- Minimal CPU/memory footprint
- Reliable command recognition