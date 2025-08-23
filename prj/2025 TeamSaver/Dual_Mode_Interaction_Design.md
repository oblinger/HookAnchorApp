# TeamSaver Dual-Mode Interaction Design

## Two Distinct Scenarios
1. **Returning Scenario**: Natural, no preparation required
2. **Lunch Scenario**: Intentional availability with AirPods

---

## Mode 1: Returning Scenario (Default)

### Interaction Flow
```
1. DETECTION (10+ feet away)
   ├── Camera detects face approaching
   ├── Avatar shows "returning" state
   └── Team notified: "Sarah is returning"

2. AVAILABILITY GATE (Must reach desk)
   ├── Not available for audio until at webcam range
   ├── Visual indicator: Avatar translucent → solid
   └── Initiator can "queue" interest

3. AT DESK (Within webcam view)
   ├── Mover now hearable (laptop speakers work)
   ├── Initiator can speak
   └── Mover hears but can't respond yet

4. SEATED & READY (Gaze/mouse detected)
   ├── System detects: seated + looking at screen OR mouse movement
   ├── Two-way audio enabled
   └── Natural conversation begins
```

### Technical Requirements
```javascript
class ReturningScenario {
  constructor() {
    this.states = {
      AWAY: 'away',
      APPROACHING: 'approaching',  // Seen but far
      AT_DESK: 'at_desk',          // In webcam range
      ENGAGED: 'engaged'            // Can respond
    };
  }
  
  async updateState(faceData, mouseData) {
    if (!faceData.detected) {
      return this.states.AWAY;
    }
    
    const distance = this.estimateDistance(faceData);
    
    if (distance > 6) {
      return this.states.APPROACHING;
    }
    
    if (distance <= 6 && !this.isSeated(faceData)) {
      return this.states.AT_DESK;  // Can hear, not respond
    }
    
    if (this.isSeated(faceData) && 
        (this.detectGaze(faceData) || mouseData.recentMovement)) {
      return this.states.ENGAGED;  // Full communication
    }
    
    return this.states.AT_DESK;
  }
}
```

---

## Mode 2: Lunch Scenario (Opt-in Extended Availability)

### Setup Phase
```
User explicitly indicates extended availability:
1. Puts in AirPods
2. Opens TeamSaver mobile app
3. Taps "Extended Availability" mode
4. App confirms: "You're available for next [30] minutes"
```

### Interaction Flow
```
1. MODE ACTIVATION
   ├── Mobile app in "Extended Availability"
   ├── Avatar shows special state (cooking icon?)
   ├── Team sees: "Sarah is available (making lunch)"
   └── Phone in pocket, AirPods connected

2. INITIATOR ENGAGES
   ├── Tom sees Sarah available
   ├── Clicks/gazes at Sarah's avatar
   ├── Says: "Hey Sarah, quick question?"
   └── Audio routes to Sarah's AirPods

3. MOVER ACKNOWLEDGMENT OPTIONS
   ├── Option A: AirPod tap (most reliable)
   ├── Option B: Voice activation (if possible)
   ├── Option C: Phone vibrate pattern response
   └── Option D: Apple Watch tap

4. CONVERSATION
   ├── Two-way audio through AirPods
   ├── Hands remain free for cooking
   └── Can end with AirPod tap or voice command
```

---

## Mobile App Audio Capabilities

### iOS Capabilities
```swift
// iOS Background Audio
class TeamSaverAudioSession {
  func setupBackgroundAudio() {
    // iOS allows background audio with proper permissions
    AVAudioSession.sharedInstance().setCategory(.playAndRecord, 
                                                mode: .voiceChat,
                                                options: [.allowBluetooth])
    
    // Can receive audio while app is backgrounded
    UIApplication.shared.beginReceivingRemoteControlEvents()
  }
  
  func handleAirPodTap() {
    // Can intercept play/pause button
    MPRemoteCommandCenter.shared().playCommand.addTarget { event in
      self.toggleMicrophone()
      return .success
    }
  }
}
```

**iOS Permissions Needed**:
- Background Audio capability
- Microphone access
- Bluetooth access
- Push notifications for initial alert

### Android Capabilities
```kotlin
// Android Foreground Service for Audio
class TeamSaverAudioService : Service() {
  override fun onCreate() {
    // Must run as foreground service
    startForeground(NOTIFICATION_ID, createNotification())
    
    // Can run audio in background
    audioManager = getSystemService(Context.AUDIO_SERVICE)
    audioManager.requestAudioFocus(...)
  }
  
  fun detectVoiceActivation() {
    // Android allows continuous mic access in foreground service
    // But battery impact is significant
    speechRecognizer = SpeechRecognizer.createSpeechRecognizer(this)
  }
}
```

**Android Limitations**:
- Must show persistent notification
- Battery optimization might kill service
- Voice activation less reliable than iOS

---

## Activation Methods for Lunch Scenario

### Method 1: AirPod Controls (Most Reliable)
```
Single Tap: Toggle mic on/off
Double Tap: End conversation
Long Press: Activate Siri/Assistant → "TeamSaver respond"
```

**Pros**: Works reliably, no extra hardware
**Cons**: Must remember tap patterns

### Method 2: Wake Word Detection
```javascript
class WakeWordDetector {
  constructor() {
    // Continuous listening for "Hey TeamSaver"
    this.wakeWords = ['hey teamsaver', 'teamsaver yes', 'go ahead'];
    this.listening = false;
  }
  
  async startListening() {
    // iOS: Can use SiriKit or continuous recognition
    // Android: SpeechRecognizer with partial results
    // Battery impact: ~10-15% per hour
  }
}
```

**Pros**: Most natural interaction
**Cons**: Battery drain, privacy concerns, might not work in noisy kitchen

### Method 3: Vibration Pattern Response
```javascript
class VibrationResponse {
  notifyIncoming() {
    // Distinct pattern for "someone wants to talk"
    navigator.vibrate([200, 100, 200, 100, 400]);
  }
  
  detectResponse() {
    // User shakes phone twice = accept
    // User ignores = decline after 10 seconds
    // Problem: Phone in pocket, hard to shake intentionally
  }
}
```

**Pros**: No audio needed
**Cons**: Unreliable detection in pocket

### Method 4: Smart Watch Integration
```swift
// Apple Watch Companion
class TeamSaverWatch {
  func incomingRequest(from: String) {
    // Tap notification on wrist
    WKInterfaceDevice.current().play(.notification)
    
    // Show accept/decline buttons
    presentController(withName: "AcceptCall", context: from)
  }
  
  func handleTap() {
    // Raise wrist to accept
    // Or tap complication
  }
}
```

**Pros**: Very reliable, natural gesture
**Cons**: Requires Apple Watch/WearOS

---

## Privacy & Consent Model

### Returning Scenario
- **Implicit consent**: Returning to desk implies availability
- **Progressive exposure**: Must reach desk before audio
- **Clear states**: Visual indicators of availability level

### Lunch Scenario  
- **Explicit opt-in**: Must activate "Extended Availability"
- **Time-boxed**: Auto-expires after set period
- **Clear indicator**: Special avatar state shows intentional availability
- **Easy exit**: Single tap to end availability

---

## Implementation Architecture

```
TeamSaver System
├── Desktop App
│   ├── Camera Module (detect returning)
│   ├── Avatar Renderer (show states)
│   ├── Audio Router (laptop speakers)
│   └── State Manager (track availability)
│
├── Mobile App (for Lunch Scenario)
│   ├── Background Audio Service
│   ├── AirPod Integration
│   ├── Wake Word Detection (optional)
│   ├── Vibration Patterns
│   └── Extended Availability Mode
│
└── Watch App (optional)
    ├── Haptic Notifications
    ├── Quick Responses
    └── Wrist Raise Detection
```

---

## User Experience Flows

### Returning Scenario (Zero Friction)
```
Sarah goes to bathroom
    ↓ (no action required)
Returns to desk area
    ↓ (detected automatically)
Tom sees "Sarah returning"
    ↓
Tom: "Hey Sarah, got a sec?"
    ↓
Sarah reaches desk, hears Tom
    ↓
Sarah sits, looks at screen
    ↓ (gaze detected)
Sarah: "Sure, what's up?"
```

### Lunch Scenario (Intentional Availability)
```
Sarah puts in AirPods
    ↓
Opens TeamSaver app
    ↓
Taps "Making Lunch (30 min)"
    ↓
Starts cooking
    ↓
Tom sees Sarah available
    ↓
Tom: "Hey Sarah, quick question?"
    ↓ (audio to AirPods)
Sarah taps AirPod
    ↓
Sarah: "Go ahead, Tom"
    ↓
Hands-free conversation while cooking
```

---

## Technical Feasibility

### What Works Well
✅ Returning scenario: Fully achievable with webcam
✅ iOS background audio: Well-supported
✅ AirPod tap controls: Reliable activation
✅ Time-boxed availability: Clear boundaries

### What's Challenging
⚠️ Android background audio: Battery/reliability issues
⚠️ Wake word in pocket: Muffled detection
⚠️ Vibration responses: Hard to detect intentional shakes
⚠️ Cross-platform consistency: iOS easier than Android

### What's Not Possible
❌ Truly hands-free response without wake word
❌ Reliable voice activation with phone in pocket
❌ Zero-battery-impact continuous listening

---

## Recommendations

### MVP: Returning Scenario Only
- Focus on desk-based interactions
- No mobile app required
- Natural, zero-friction experience
- Covers 80% of use cases

### Phase 2: Add Lunch Scenario
- Optional mobile app
- AirPod tap activation (simplest)
- 30-minute time boxes
- Clear opt-in process

### Phase 3: Enhanced Activation
- Apple Watch support
- Wake word detection (when feasible)
- Smart home integration

---

## Conclusion

The dual-mode approach balances:
- **Returning**: Zero-friction, natural, works for most interactions
- **Lunch**: Intentional availability for extended periods, requires AirPods

This design is honest about limitations while delivering real value in both scenarios. The returning scenario alone is valuable enough to ship, with lunch scenario as a premium enhancement.

---

## Document History
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-01-10 | Initial | Dual-mode interaction design |