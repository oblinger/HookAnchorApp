# iOS Background Audio Capabilities

## What Background Audio Gives You

### Core Capabilities
When you enable background audio mode in iOS, your app can:

1. **Continue playing/recording audio when:**
   - App moves to background
   - Screen locks
   - User switches to other apps
   - Phone is in pocket

2. **Receive remote control events:**
   - Play/pause from AirPods
   - Skip/previous from Control Center
   - Volume changes
   - Siri commands

3. **Maintain network connections:**
   - WebRTC connections stay alive
   - WebSocket connections maintained
   - Can receive push notifications to wake app

### Code Implementation
```swift
// Enable background audio in Info.plist
<key>UIBackgroundModes</key>
<array>
    <string>audio</string>
    <string>voip</string>  // For call-like apps
</array>

// Configure audio session
import AVFoundation

class TeamSaverAudio {
    func configureBackgroundAudio() {
        do {
            try AVAudioSession.sharedInstance().setCategory(
                .playAndRecord,  // Both play and record
                mode: .voiceChat,  // Optimized for voice
                options: [
                    .allowBluetooth,  // AirPods
                    .defaultToSpeaker,  // Speakerphone
                    .mixWithOthers  // Don't stop music
                ]
            )
            
            try AVAudioSession.sharedInstance().setActive(true)
        } catch {
            print("Failed to set audio session")
        }
    }
}
```

---

## Screen Lock Timing & App Lifecycle

### Default Lock Times
- **Default setting**: 30 seconds - 5 minutes (user configurable)
- **During active audio**: Screen can lock but app continues
- **With background audio**: App runs indefinitely while audio active

### App States with Background Audio

```
FOREGROUND (app visible)
    ↓ (user switches apps or locks)
BACKGROUND (still running)
    ├─ With audio active: Runs indefinitely
    └─ Without audio: ~30 seconds then suspended
    
SUSPENDED (frozen in memory)
    ├─ Can be woken by push notification
    └─ Can be woken by VoIP push (special)
    
TERMINATED (killed)
    └─ Must be relaunched by user
```

### Keeping App Alive During Lunch Scenario

```swift
class LunchModeManager {
    var silentAudioPlayer: AVAudioPlayer?
    
    func startLunchMode() {
        // Play silent audio to keep app alive
        // This is a common technique but Apple may reject if obvious
        playSilentAudio()
        
        // Better approach: Use VoIP background mode
        startVoIPSession()
    }
    
    func startVoIPSession() {
        // VoIP apps can run indefinitely
        // Must show CallKit UI for incoming "calls"
        let provider = CXProvider(configuration: config)
        provider.setDelegate(self, queue: nil)
    }
    
    func playSilentAudio() {
        // Play inaudible audio file on loop
        let url = Bundle.main.url(forResource: "silence", withExtension: "mp3")
        silentAudioPlayer = try? AVAudioPlayer(contentsOf: url)
        silentAudioPlayer?.numberOfLoops = -1  // Infinite
        silentAudioPlayer?.volume = 0.01  // Nearly silent
        silentAudioPlayer?.play()
    }
}
```

---

## VoIP Mode (Best for TeamSaver)

### What VoIP Background Mode Provides
- **Indefinite background execution** while "call" active
- **High-priority push notifications** that wake app
- **Network connections maintained**
- **Can start audio from background**

### Requirements for VoIP Mode
```swift
// Must use CallKit for incoming calls
import CallKit

class TeamSaverCallManager: CXProviderDelegate {
    func reportIncomingCall(from: String) {
        let update = CXCallUpdate()
        update.remoteHandle = CXHandle(type: .generic, value: from)
        update.hasVideo = false
        
        provider.reportNewIncomingCall(
            with: UUID(),
            update: update
        ) { error in
            // Handle error
        }
    }
}
```

**Important**: Apple requires CallKit UI for VoIP apps. Users will see standard iOS call interface.

---

## Real-World Behavior for TeamSaver

### Scenario: Lunch Mode with Locked Phone

```
Timeline:
0:00 - User activates "Lunch Mode", puts phone in pocket
0:30 - Screen auto-locks (but app still running)
5:00 - Colleague initiates conversation
5:01 - App receives notification, starts audio to AirPods
30:00 - Still running fine with audio active
35:00 - User ends lunch mode
35:01 - App can go to background/suspend
```

### Power Consumption

| Mode | Battery Impact | Duration |
|------|---------------|----------|
| Silent audio loop | 5-8% per hour | Indefinite |
| Active VoIP session | 10-15% per hour | Indefinite |
| WebRTC audio active | 15-20% per hour | Indefinite |
| Microphone always on | 20-25% per hour | Indefinite |
| Screen on + audio | 30-40% per hour | Until battery dies |

---

## Push Notifications to Wake App

### Standard Push
```swift
// Can wake suspended app briefly (~30 seconds)
func application(_ application: UIApplication, 
                didReceiveRemoteNotification userInfo: [AnyHashable : Any],
                fetchCompletionHandler completionHandler: @escaping (UIBackgroundFetchResult) -> Void) {
    // Process notification
    // Start audio session if needed
    completionHandler(.newData)
}
```

### VoIP Push (PushKit)
```swift
// Wakes app immediately, even if terminated
import PushKit

func pushRegistry(_ registry: PKPushRegistry, 
                 didReceiveIncomingPushWith payload: PKPushPayload,
                 for type: PKPushType) {
    // App wakes immediately
    // Must report CallKit call or Apple will terminate app
    reportIncomingCall(from: payload.dictionaryPayload["caller"])
}
```

---

## AirPods Integration While Backgrounded

### Available Controls
```swift
class AirPodController {
    func setupRemoteControls() {
        let commandCenter = MPRemoteCommandCenter.shared()
        
        // Single tap (play/pause)
        commandCenter.playCommand.addTarget { event in
            self.toggleMicrophone()
            return .success
        }
        
        // Double tap (next track)
        commandCenter.nextTrackCommand.addTarget { event in
            self.endConversation()
            return .success
        }
        
        // Can't capture triple tap or long press
        // Those go to Siri/system
    }
}
```

---

## Recommendations for TeamSaver

### Approach 1: VoIP Mode (Most Reliable)
**Pros:**
- Runs indefinitely during lunch mode
- Can receive calls while suspended
- Full AirPod integration

**Cons:**
- Must show CallKit UI (looks like phone call)
- Apple review scrutiny
- Users might find it confusing

### Approach 2: Background Audio with Silent Loop
**Pros:**
- Simpler implementation
- No CallKit required
- Less intrusive

**Cons:**
- Apple might reject for "misuse"
- Uses more battery
- Feels hacky

### Approach 3: Time-Limited Sessions
**Pros:**
- Most honest approach
- Better battery life
- Clear expectations

**Cons:**
- Might suspend during long lunch
- Requires periodic "check-ins"

### Recommended: Hybrid Approach
```swift
class TeamSaverBackgroundManager {
    func startLunchMode(duration: TimeInterval) {
        // 1. Register for VoIP pushes
        registerForVoIPPushes()
        
        // 2. Start audio session (not silent)
        startAudioSession()
        
        // 3. Set expiration timer
        scheduleLocalNotification(after: duration)
        
        // 4. When someone calls, use CallKit
        // This is legitimate VoIP usage
    }
}
```

---

## Key Insights

1. **Background audio works indefinitely** - As long as audio is playing/recording, app stays alive even with locked screen

2. **VoIP mode is powerful** - Can wake terminated apps, maintain connections, but requires CallKit

3. **30-minute lunch mode is achievable** - Either through VoIP or background audio

4. **Battery impact is manageable** - 5-10% for 30-minute lunch session

5. **AirPod controls work in background** - Can use tap for push-to-talk

The lunch scenario is technically feasible on iOS with background audio or VoIP mode. The main decision is whether to use CallKit (more reliable but shows call UI) or background audio (simpler but might get suspended).

---

## Document History
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-01-10 | Initial | iOS background audio analysis |