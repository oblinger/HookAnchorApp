# TeamSaver Design Discussion

## Status: Planning Phase
**This document explores technical approaches and implementation strategies. No implementation should begin until the PRD is finalized.**

---

## 1. Technical Architecture Overview

### System Components
1. **Client Application** - Runs on user's device
2. **Presence Detection Module** - Camera/activity monitoring
3. **Virtual Space Renderer** - Avatar display and animation
4. **Communication Layer** - WebRTC for audio/video
5. **Signaling Server** - Coordinate connections between clients
6. **State Management** - Track user availability and interactions

---

## 2. Hardware Requirements & Capabilities

### Desktop/Laptop Requirements
- **Camera**: Built-in or external webcam for gaze detection
- **Microphone**: For audio communication
- **Speakers/Headphones**: For audio output
- **Display**: For virtual space visualization

### Available APIs and Capabilities

#### macOS/Windows/Linux
- **Camera Access**: 
  - MediaDevices API (getUserMedia)
  - OpenCV for face/gaze detection
  - Core ML / Windows ML for on-device inference
  
- **Activity Detection**:
  - System idle time APIs
  - Mouse/keyboard activity monitoring
  - Active window detection
  
- **Audio Processing**:
  - Web Audio API for noise detection
  - Echo cancellation
  - Noise suppression

#### iPad/Tablet
- **iOS/iPadOS**:
  - ARKit for advanced face tracking
  - Vision framework for gaze detection
  - AVFoundation for camera/audio
  
- **Android**:
  - ML Kit for face detection
  - CameraX API
  - Android Neural Networks API

---

## 3. Technology Stack Options

### Option 1: Web-Based Application
**Technologies**: 
- Frontend: React/Vue.js/Svelte + WebGL/Canvas for rendering
- Communication: WebRTC
- Backend: Node.js + Socket.io for signaling
- ML: TensorFlow.js for client-side inference

**Pros**:
- Cross-platform compatibility
- No installation required
- Easy updates
- Browser sandboxing for privacy

**Cons**:
- Limited system integration
- Browser permissions required
- Performance limitations

### Option 2: Native Desktop Application
**Technologies**:
- Electron + React/Vue (cross-platform)
- OR Native: Swift (macOS), C#/WPF (Windows), Qt (Linux)
- WebRTC or custom protocols
- Native ML frameworks

**Pros**:
- Deep system integration
- Better performance
- More control over hardware
- Background operation

**Cons**:
- Installation required
- Platform-specific development
- Update distribution

### Option 3: Hybrid Approach
**Technologies**:
- Tauri (Rust + Web frontend)
- Flutter Desktop
- React Native for Desktop

**Pros**:
- Balance of performance and development speed
- Smaller bundle size than Electron
- Good hardware access

**Cons**:
- Less mature ecosystem
- Potential compatibility issues

---

## 4. Core Technical Challenges & Solutions

### Challenge 1: Gaze Detection
**Problem**: Accurately detecting where user is looking

**Solutions**:
- **WebGazer.js**: JavaScript eye tracking library
- **OpenCV + Dlib**: More accurate but requires native code
- **Tobii SDK**: If specialized hardware available
- **Fallback**: Head pose estimation as proxy for gaze

**Recommended Approach**: 
- Start with WebGazer.js for MVP
- Implement calibration routine for accuracy
- Consider native solutions for production

### Challenge 2: Presence Detection
**Problem**: Determining when user is interruptible

**Signals to Monitor**:
```javascript
// Pseudo-code for presence detection
const presenceSignals = {
  faceVisible: checkCameraForFace(),
  systemIdle: getSystemIdleTime() > threshold,
  noActiveCall: !isInVideoCall(),
  keyboardInactive: timeSinceLastKeypress() > threshold,
  mouseStationary: timeSinceLastMouseMove() > threshold
};
```

### Challenge 3: Privacy-Preserving Architecture
**Problem**: Ensuring no actual video/audio leaks

**Solution Architecture**:
```
User Environment -> Local Processing -> Abstract Signals -> Server
                    (No raw media)      (Only metadata)
```

- All video processing happens locally
- Only send avatar state and availability signals
- Audio channels use end-to-end encryption
- Explicit consent for each escalation level

### Challenge 4: Real-time Communication
**Problem**: Low-latency audio/video when needed

**WebRTC Configuration**:
```javascript
const rtcConfig = {
  iceServers: [/* STUN/TURN servers */],
  bundlePolicy: 'max-bundle',
  rtcpMuxPolicy: 'require'
};

const audioConstraints = {
  echoCancellation: true,
  noiseSuppression: true,
  autoGainControl: true
};
```

---

## 5. Implementation Roadmap

### Phase 1: Proof of Concept
- [ ] Basic avatar display system
- [ ] Simple presence detection (active/idle)
- [ ] Manual interaction triggering
- [ ] WebRTC audio connection

### Phase 2: Core Features
- [ ] Gaze detection integration
- [ ] Virtual door states
- [ ] Automatic environment checking
- [ ] Progressive connection escalation

### Phase 3: Polish & Scale
- [ ] Multiple avatar styles
- [ ] Advanced presence heuristics
- [ ] Performance optimization
- [ ] Cross-platform testing

---

## 6. Key Libraries and APIs

### Computer Vision & ML
- **TensorFlow.js**: Browser-based ML
- **MediaPipe**: Google's ML solutions
- **face-api.js**: Face detection in browser
- **WebGazer.js**: Eye tracking

### Real-time Communication
- **WebRTC**: Peer-to-peer communication
- **Socket.io**: Signaling and presence
- **SimplePeer**: WebRTC wrapper
- **Agora SDK**: Alternative to WebRTC

### UI/Animation
- **Three.js**: 3D graphics
- **Pixi.js**: 2D graphics
- **Framer Motion**: Animations
- **Lottie**: Complex animations

### Desktop Integration
- **Electron**: Cross-platform desktop
- **Tauri**: Lightweight alternative
- **node-window-manager**: Window detection
- **node-active-window**: Active app detection

---

## 7. Security & Privacy Considerations

### Data Protection
- No video/audio recording without consent
- Local processing of biometric data
- Encrypted communication channels
- Clear data retention policies

### User Control
- Granular permission settings
- Easy opt-out mechanisms
- Activity logs for transparency
- No hidden monitoring

---

## 8. Performance Targets

### Latency Requirements
- Gaze detection: < 100ms
- Presence update: < 500ms
- Audio channel setup: < 2 seconds
- Video escalation: < 5 seconds

### Resource Usage
- CPU: < 5% idle, < 20% active
- Memory: < 200MB baseline
- Network: < 10KB/s idle, adaptive when active

---

## 9. Testing Strategy

### Unit Testing
- Presence detection logic
- State management
- Communication protocols

### Integration Testing
- End-to-end user flows
- Multi-user scenarios
- Network failure handling

### User Testing
- Gaze detection accuracy
- Interaction naturalness
- Privacy perception

---

## 10. Open Questions for Technical Design

1. **Gaze Detection Accuracy**: What's the minimum accuracy needed for good UX?
2. **Calibration**: How often should users calibrate gaze tracking?
3. **Mobile Support**: Should we support mobile devices in v1?
4. **Infrastructure**: Self-hosted vs. cloud-based signaling servers?
5. **Integration**: How deep should integration with Google Meet/Zoom be?
6. **Fallbacks**: What happens when camera/mic unavailable?

---

## Next Steps

1. **Technology Selection**: Choose between web, native, or hybrid
2. **Prototype**: Build gaze detection proof-of-concept
3. **User Research**: Test interaction patterns with target users
4. **Architecture Review**: Validate technical approach with team
5. **Security Audit**: Review privacy implications

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2025-01-10 | Initial | Created initial design discussion |