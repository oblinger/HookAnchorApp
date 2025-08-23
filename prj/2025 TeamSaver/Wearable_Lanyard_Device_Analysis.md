# Wearable Lanyard Device Analysis for TeamSaver

## The Concept
A dedicated wearable device on a lanyard that provides instant, hands-free communication just by picking it up from a charging cradle.

---

## Existing Devices That Could Work

### Option 1: Small Bluetooth Speakers with Mic

#### **JBL Clip 4** ($50-80)
- Built-in carabiner (add lanyard)
- Speakerphone capability
- 10-hour battery
- **Problem**: Too bulky for neck wear (240g)

#### **Tribit StormBox Micro** ($40-60)
- Strap included (could modify)
- Built-in mic for calls
- 8-hour battery
- **Problem**: Still chunky (290g)

### Option 2: Dedicated Speakerphone Devices

#### **Jabra Speak 410/510** ($80-150)
- Designed for conference calls
- Excellent mic array
- USB/Bluetooth
- **Problem**: Disc shape, not wearable

#### **eMeet Luna** ($70-90)
- Compact speakerphone
- 360° voice pickup
- Bluetooth 5.0
- **Problem**: Round puck design

### Option 3: Wearable Speakers (Neck Speakers)

#### **Sony SRS-NB10** ($100-150) ⭐ BEST FIT
- **Designed to wear around neck**
- All-day comfort (113g)
- Dedicated mute button
- 20-hour battery
- USB-C charging
- **Built for Zoom/Teams calls**
- Directional speakers (others don't hear)

#### **Bose SoundWear Companion** ($200-300)
- Rests on shoulders
- 12-hour battery
- Good mic quality
- **Problem**: Discontinued, expensive

#### **JBL Soundgear Sense** ($150)
- Open-ear design
- Rests on shoulders
- 6-hour battery
- Good for calls

### Option 4: Walkie-Talkie Style Devices

#### **Relay+** ($50 + subscription)
- Kid-focused but works for adults
- Push-to-talk device
- GPS + WiFi/LTE
- Clips to clothes
- **Problem**: Requires monthly subscription

#### **Motorola Talkabout** ($30-60 pair)
- Classic walkie-talkie
- Belt clip (add lanyard)
- Push-to-talk
- **Problem**: Not WiFi, uses radio frequencies

---

## The Perfect Device: Custom Hardware

### What We'd Want
```
Ideal TeamSaver Pendant
├── Size: AirPods case dimensions
├── Weight: <50g
├── Battery: 8+ hours
├── Charging: Wireless/magnetic cradle
├── Audio: Decent speaker + good mic
├── Controls: One button or automatic
├── Connection: WiFi or Bluetooth to phone/computer
└── Price: <$100
```

### DIY Approach: Raspberry Pi Zero W Build

#### Components
- Raspberry Pi Zero W: $15
- I2S MEMS Microphone: $5
- Mini speaker (2W): $5
- Battery (1000mAh): $10
- Case + lanyard: $10
- Charging circuit: $5
- **Total: ~$50**

#### Challenges
- Assembly required
- Software development needed
- Not consumer-ready
- Size still bulky

---

## Repurposing Existing Devices

### Best Option: Old Smartphone as Pendant

#### iPhone SE (2016) or iPhone 12 Mini
- Already has everything needed
- Can run TeamSaver app
- Good speaker/mic
- 5-10 hour battery for audio
- Wireless charging capable
- **Cost**: $50-150 used

#### Implementation
```swift
// Dedicated "Pendant Mode" in app
class PendantMode {
    func activate() {
        // Always-on screen with simple UI
        showLargeStatusDisplay()
        
        // Proximity sensor for auto-activation
        if proximitySensor.nearBody {
            enableCommunication()
        }
        
        // One-button interface
        volumeButton.action = toggleMicrophone
    }
}
```

#### Advantages
- No new hardware needed
- Full app capabilities
- Visual feedback on screen
- Can still see avatars

---

## Smart Badge Concept

### Existing Smart Badges

#### **Humane AI Pin** ($699 + subscription) 
- Magnetic attachment
- Projector display
- Voice-first interface
- **Problem**: Expensive, overkill

#### **Vocera Badge** ($300-400)
- Hospital communication device
- Hands-free voice
- "Genie, call Dr. Smith"
- **Problem**: Enterprise only, expensive

#### **OrCam Hear** ($200)
- Pendant-style device
- AI-powered audio
- **Problem**: Focused on hearing aid

---

## Cost Comparison

| Solution | Initial Cost | Ongoing | Pros | Cons |
|----------|-------------|---------|------|------|
| **Sony Neck Speaker** | $100-150 | None | Purpose-built, comfortable | Not truly pendant-style |
| **Old iPhone as Pendant** | $50-150 | None | Full featured, visual display | Might be heavy |
| **Custom Raspberry Pi** | $50 | None | Customizable | DIY only |
| **AirPods** | $130-250 | None | Great audio, familiar | Must remember to wear |
| **Smart Badge (future)** | $200-300? | Maybe | Purpose-built | Doesn't exist yet |

---

## The Magnetic Cradle Charging Station

### For iPhone/Android Pendant
```
MagSafe/Qi Charging Stand
├── Always visible on desk
├── Phone displays TeamSaver status
├── One motion to grab and wear
├── Auto-activates pendant mode
└── Returns to normal when docked
```

Commercial options:
- Belkin MagSafe Stand: $40
- Anker PowerWave: $30
- Native Union Dock: $60

### DIY Arduino Version
```cpp
// Detect device pickup
if (!deviceOnCradle) {
    sendMessage("User mobile");
    activatePendantMode();
}
```

---

## User Experience Comparison

### Lanyard Device Flow
```
1. See device on cradle (always visible reminder)
2. Pick up and put on (one motion)
3. Walk away (automatically available)
4. Return and place on cradle (done)

Friction: Almost zero
Reliability: Very high
```

### AirPods Flow
```
1. Remember to get AirPods
2. Put them in ears
3. Open app, activate mode
4. Put phone in pocket
5. Remember to take out/charge

Friction: Moderate
Reliability: Good audio, more steps
```

---

## Recommendation

### Immediate Solution: Sony SRS-NB10 ($100)
- Designed for exactly this use case
- Comfortable for extended wear
- Good battery life
- Mute button accessible
- Already exists and tested

### Budget Solution: Old iPhone SE + Lanyard Case ($75)
```
iPhone SE (used): $50
Lanyard case: $15
MagSafe adapter: $10
Total: $75
```

Run TeamSaver in "pendant mode" with always-on display showing avatar status.

### Future Vision: Custom TeamSaver Badge
If TeamSaver succeeds, consider hardware:
- Partner with manufacturer
- ~$100 target price
- E-ink display for avatars
- 24-hour battery
- Magnetic charging
- One-button interface

---

## Why Lanyard Device Could Be Better

### Advantages Over AirPods
✅ **Zero preparation** - Just grab and go
✅ **Visual reminder** - Always visible on desk
✅ **No ear fatigue** - External speaker
✅ **Shareable** - Others can hear if wanted
✅ **Visible status** - Others see you're "wired"

### Disadvantages
❌ **Audio quality** - Not as good as AirPods
❌ **Privacy** - Others might overhear
❌ **Fashion** - Wearing device visible
❌ **Another device** - One more thing to manage

---

## Behavioral Psychology

### The Lanyard Advantage
```
Behavioral trigger chain:
See device → Pick up → Wear → Available

vs AirPods:
Remember → Find → Put in → App → Pocket
```

The lanyard device could become like a "communication necklace" - putting it on signals to yourself and others that you're in "available mode."

---

## Prototype Test Plan

### Phase 1: Test with Sony Neck Speaker
- $100 investment
- Test the interaction model
- Gauge user acceptance

### Phase 2: Old Phone Pendant
- Develop pendant mode UI
- Test weight/comfort
- Validate the concept

### Phase 3: Consider Custom Hardware
- Only if proven demand
- Partner with hardware manufacturer
- Kickstarter potential

---

## Conclusion

**Yes, this could work!** The Sony SRS-NB10 neck speaker or an old iPhone with lanyard could create an even more frictionless experience than AirPods. The key insight is that **physical tokens** (putting on the device) can be more intuitive than **digital modes** (activating lunch mode).

The $100-150 price point is reasonable for dedicated users who value the seamless experience. The "grab and go" interaction could be the differentiator that makes TeamSaver truly effortless.

---

## Document History
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-01-10 | Initial | Wearable device analysis |