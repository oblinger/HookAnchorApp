# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TextCapture is a macOS speech-to-text application built in Swift that provides an always-on-top floating window for continuous voice transcription. The main codebase is located in the `code/TextCapture/` directory (accessed via symlink).

## Development Commands

### Building
```bash
cd code/TextCapture
./build.sh
```

### Running
```bash
cd code/TextCapture
open TextCapture.app
```

### Development Environment
```bash
# Start tmux session with multiple panes for development
tmuxp load .tmuxp.yaml
```

## Architecture

### Core Components

**TextAnchor System**: The app uses an anchor-based text management system where text positions are tracked through editing operations. Each anchor maintains its position as surrounding text changes.

**Speech Engine Abstraction**: Speech recognition is abstracted through a protocol-based system supporting:
- Apple Speech Framework (local and cloud)
- OpenAI Whisper API integration

**Action System**: Voice commands are processed through a centralized action dispatcher that:
- Intercepts text for command patterns
- Executes system actions (dictation control, text manipulation)
- Supports customizable trigger words

### Key Architecture Patterns

1. **Protocol-Based Design**: Core interfaces (TextBuffer, SpeechEngine) are protocol-based for extensibility
2. **Anchor-Based Text Management**: Position tracking through TextAnchor objects that survive text edits
3. **SwiftUI + AppKit Hybrid**: Uses SwiftUI for main UI with custom NSTextView for advanced text editing
4. **Observable Pattern**: State management through @ObservableObject and Combine

## Configuration

The app uses `config.yaml` for configuration including:
- Speech recognition settings (Apple vs OpenAI)
- Window positioning and behavior
- Voice command trigger words
- Text sending preferences
- Logging configuration

## Key Directories

- `Sources/Models/` - TextAnchor and TextBuffer implementations
- `Sources/Services/` - Speech engines and action system
- `Sources/Views/` - SwiftUI views and custom text editor
- `Sources/Utilities/` - Logging and configuration utilities
- `docs/` - API documentation and task specifications

## Voice Commands

The app supports voice commands for:
- Starting/stopping dictation ("commence", "halt")
- Text manipulation ("strike" for delete, "zap" for clear)
- Text sending ("submit", "dispatch")
- Line formatting ("newline", "break")

Commands work through trigger word detection in the speech stream and can be customized via config.yaml.

## Development Notes

- The app requires microphone and speech recognition permissions
- Uses Swift Package Manager for dependencies
- Supports both on-device and cloud-based speech recognition
- Window positioning is configurable and persisted
- Comprehensive logging system with configurable channels