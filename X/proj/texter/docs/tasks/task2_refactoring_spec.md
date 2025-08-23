# Task 2: TextBuffer/SpeechEngine Refactoring Specification

## Overview
Refactor the text capture application to cleanly separate text management from speech recognition through a simple anchor-based API.

## Core Concepts

### TextAnchor
A position in the text buffer that maintains its location even as text around it changes. Each anchor tracks a range of text that can be updated atomically.

### TextBuffer
Manages the document text and provides anchors for external systems to modify specific positions.

### SpeechEngine
Handles speech recognition and updates the text buffer through anchors, without the buffer knowing anything about speech.

---

## API Specification

### 1. TextAnchor

```swift
class TextAnchor {
    // The current text at this anchor position (read-only)
    private(set) var text: String
    
    // Replace the text at this anchor with new text
    // - Parameter newText: The text to insert (empty string removes text)
    func update(_ newText: String)
}
```

**Key Behaviors:**
- Anchors maintain their position as text is inserted/deleted elsewhere
- Calling `update()` replaces whatever text the anchor currently has
- Anchors handle position tracking internally
- Update with empty string (`""`) removes text at anchor

---

### 2. TextBuffer

```swift
protocol TextBuffer {
    // Get an anchor at the current cursor position
    func getAnchor() -> TextAnchor
}
```

**Key Behaviors:**
- Simple, single-method interface
- Always returns anchor at current cursor position
- No knowledge of speech or any specific use case
- Anchors remain valid until released

---

### 3. SpeechEngine

```swift
protocol SpeechEngine {
    // Initialize with a text buffer to update
    init(textBuffer: TextBuffer)
    
    // Start listening and transcribing
    func start()
    
    // Stop listening
    func stop()
    
    // Interrupt current recognition (for future implementation)
    func interrupt(gentle: Bool)
}
```

**Internal Implementation Pattern:**
```swift
class SpeechEngineImpl: SpeechEngine {
    private let textBuffer: TextBuffer
    private var currentAnchor: TextAnchor?
    
    init(textBuffer: TextBuffer) {
        self.textBuffer = textBuffer
    }
    
    private func onSpeechStart() {
        // Get new anchor at cursor position
        currentAnchor = textBuffer.getAnchor()
    }
    
    private func onPartialText(_ text: String) {
        // Update anchor with partial
        currentAnchor?.update(text)
    }
    
    private func onFinalText(_ text: String) {
        // Update anchor with final
        currentAnchor?.update(text)
        // Ready for next phrase
        currentAnchor = textBuffer.getAnchor()
    }
}
```

---

## Usage Flow

### Basic Speech Recognition Cycle

1. **User starts speaking**
   ```swift
   speechEngine.start()
   // Internally: currentAnchor = textBuffer.getAnchor()
   ```

2. **Partial text arrives** (e.g., "Hello")
   ```swift
   // Internally: currentAnchor.update("Hello")
   ```

3. **More partial text** (e.g., "Hello world")
   ```swift
   // Internally: currentAnchor.update("Hello world")
   // Previous "Hello" is replaced with "Hello world"
   ```

4. **Final text arrives**
   ```swift
   // Internally: currentAnchor.update("Hello world.")
   ```

5. **New phrase begins**
   ```swift
   // Internally: currentAnchor = textBuffer.getAnchor()
   // New anchor at current cursor position
   ```

### Cursor Movement Handling

```swift
// Scenario: User moves cursor while speaking

1. Speech in progress at position A
   currentAnchor.update("Hello")

2. User moves cursor to position B
   // Text buffer internally tracks cursor change

3. More speech arrives for position A
   currentAnchor.update("Hello world")  // Still updates position A

4. Speech finalizes
   currentAnchor.update("Hello world.")

5. Next speech starts at position B
   currentAnchor = textBuffer.getAnchor()  // New anchor at B
```

---

## Implementation Benefits

### 1. **Clean Separation**
- TextBuffer has no speech-specific methods
- SpeechEngine has no text manipulation logic
- Anchors encapsulate position tracking

### 2. **Extensibility**
- Anchors can be used for other features:
  - Autocomplete
  - Code snippets
  - Multi-cursor editing
  - Find and replace

### 3. **Simplicity**
- Minimal API surface
- Single responsibility for each component
- Natural handling of cursor movement

### 4. **Robustness**
- Anchors maintain position through edits
- No manual index math required
- Context tracking for position recovery

---

## Migration Path

### Phase 1: Implement TextAnchor
1. Create TextAnchor class with position tracking
2. Add context storage for position recovery
3. Implement update() method

### Phase 2: Add TextBuffer Protocol
1. Define protocol with getAnchor()
2. Implement in existing text view
3. Test anchor stability through edits

### Phase 3: Refactor Speech Engines
1. Update SpeechEngine protocol
2. Modify Apple implementation to use anchors
3. Modify OpenAI implementation to use anchors
4. Remove old partial/final text handling

### Phase 4: Testing & Polish
1. Test cursor movement during speech
2. Test rapid anchor creation/updates
3. Optimize anchor cleanup

---

## Edge Cases Handled

1. **Cursor moves during speech** - Current anchor continues updating original position
2. **Text deleted at anchor** - Anchor tracks to nearest valid position
3. **Rapid speech updates** - Each update replaces previous atomically
4. **Multiple anchors** - Each maintains independent position
5. **Buffer cleared** - Anchors update to beginning/end as appropriate

---

## Future Extensions

- **Anchor groups** - Multiple cursors updating simultaneously
- **Anchor persistence** - Save/restore anchor positions
- **Anchor styling** - Highlight text at anchor positions
- **Anchor callbacks** - Notify when anchor position changes
- **Undo/redo** - Anchors maintain position through history

---

## Success Criteria

1. TextBuffer has no knowledge of speech recognition
2. SpeechEngine only interacts through anchors
3. Cursor movement doesn't break ongoing transcription
4. API is simple enough to explain in one paragraph
5. System is extensible for future features