# Vinyl Transfer Cleaner — Requirements & UI Wireframe

## 1. Purpose & Goals

**Purpose:**
Provide a consumer-friendly application that cleans vinyl transfers by removing clicks, pops, and crackles while preserving musical detail. The app should be visually engaging and require minimal configuration.

**Primary Goals:**
- Automatically detect and remove impulsive noise (clicks, pops, crackles).
- Maintain musical fidelity (avoid dulling transients or introducing artifacts).
- Offer an intuitive, visually pleasing user experience.
- Minimize user settings; emphasize intelligence and automation.

**Non-Goals:**
- Professional mastering-grade, per-band spectral repair tools.
- Advanced manual editing or waveform surgery features.

## 2. Target Users

- **Primary:** Vinyl hobbyists who digitize records at home.
- **Secondary:** Casual listeners digitizing older collections with minimal technical knowledge.
- **Constraints:** Users may have limited audio engineering experience and want “one-click” cleanup.

## 3. User Experience Principles

1. **One-Click Clean:** Provide a single “Clean” action that works well in most cases.
2. **Smart Defaults:** Use AI/ML or robust heuristics to auto-adjust internally.
3. **Transparency:** Show a clear before/after comparison and cleaning confidence.
4. **Minimal Cognitive Load:** Avoid complex sliders or jargon-heavy controls.
5. **Beautiful Visuals:** Elegant animations and satisfying feedback for progress and success.

## 4. Core Functional Requirements

### 4.1 Audio Import & Export
- **Input Formats:** WAV, AIFF, FLAC, MP3 (as source), with sample rates from 44.1k–192k.
- **Output Formats:** WAV, FLAC, AIFF, MP3.
- **Metadata Support:** Preserve tags if possible (artist, album, track, year).
- **Batch Support:** Optional but prioritized for future release.

### 4.2 Automatic Cleaning (Core Engine)
- **Detect clicks/pops/crackle** using signal analysis (impulse detection, spectral analysis).
- **Remove noise** without audible pumping or loss of musical transients.
- **Music preservation:** Must not alter pitch/timbre significantly.
- **Target quality:** Audible improvement with minimal artifacts on 90% of typical vinyl rips.

### 4.3 Intelligent Processing
- **Auto-sensitivity adaptation** based on audio characteristics (genre, loudness, noise profile).
- **Confidence scoring:** Internal quality confidence shown as “Cleanliness meter.”
- **No manual threshold tweaking** as a default: provide advanced panel only in “Expert mode.”

### 4.4 Visual & UX Features
- **Waveform + Spectrogram view** for visual trust in the process.
- **Before/After toggling:** single button or slider to compare.
- **Progress feedback:** animated cleaning path with time estimate.
- **Visual highlights:** show where clicks/pops were removed (subtle markers).

## 5. User Interface Requirements

### 5.1 Primary Screen
- **Header:** App title + project file name.
- **Main panel:**
  - Waveform/spectrogram visualization.
  - “Clean” button (prominent).
  - Playback controls (play, pause, scrub).
- **Side panel:**
  - Cleanliness meter (e.g., “Good / Great / Excellent”).
  - Export button.
  - Optional “Expert Mode” toggle (collapsed by default).

### 5.2 Expert Mode (Optional)
- Minimal additional controls:
  - “Aggressiveness” selector (Low/Standard/High).
  - “Preserve transients” toggle.
- Default remains “Standard”.

### 5.3 Accessibility
- High-contrast mode.
- Keyboard shortcuts for play/pause, toggle before/after.
- Clear labels and tooltips (non-technical language).

## 6. Processing Pipeline Requirements

### 6.1 Typical Pipeline (Suggested)
1. **Input normalization** (non-destructive, internal only).
2. **Noise detection**
   - Impulse detection (time-domain peaks).
   - Spectral analysis for narrow spikes.
3. **Repair**
   - Interpolation / spectral inpainting to replace impulses.
4. **Quality validation**
   - Verify no clipping, minimal loss of transient energy.
5. **Output**
   - Render to selected format.

### 6.2 Performance Targets
- Real-time playback after cleaning for files < 60 minutes.
- Cleaning time: **< 2x file duration** on standard consumer hardware.

## 7. Quality Requirements

### 7.1 Audio Quality
- Preserve dynamic range and transients.
- Minimal introduction of warbling, metallic artifacts, or dulling.
- No clipping introduced.

### 7.2 Visual Quality
- Smooth animations and responsive UI.
- Professional, calming aesthetic.

## 8. Reliability & Error Handling

- Informative error messages for corrupted files.
- Safe recovery from crashes (autosave last import state).
- Graceful handling of unsupported formats.

## 9. Privacy & Security

- All processing **local by default**, no uploads.
- If cloud processing is offered, require explicit opt-in.
- No tracking beyond minimal anonymous usage analytics (optional).

## 10. Platform Requirements

- **Desktop first**: macOS and Windows.
- Optional Linux support.
- Minimum hardware: 4 GB RAM, modern CPU.

## 11. Success Metrics

- **User success:** 80%+ of users can clean a file without using Expert Mode.
- **Perceived improvement:** user survey > 4/5 on cleaning quality.
- **Retention:** > 40% of users return within 30 days.

## 12. Future Considerations

- Batch cleaning.
- Automatic track splitting.
- AI-assisted genre-based profiles.
- Integration with audio libraries (iTunes, MusicBrainz).

## 13. Out of Scope (for v1)

- Manual spectral editing.
- Advanced mastering tools (EQ, compression).
- Real-time hardware input monitoring.

---

# UI Wireframe Mockup

## Desktop Layout (Primary Screen)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ Vinyl Transfer Cleaner                                      [Project ▾] [●●●] │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐    │
│  │                              Waveform                                │    │
│  │  ────────────────────────────────────────────────────────────────    │    │
│  │  |                                                                    │    │
│  │  |                                                                    │    │
│  │  |                                                                    │    │
│  │  ────────────────────────────────────────────────────────────────    │    │
│  └──────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐    │
│  │                            Spectrogram                               │    │
│  │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░        │    │
│  │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░        │    │
│  └──────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
│  [⟲ Back]   [▶ Play]  [⏸ Pause]   [⤒ Before] [⤓ After]   [⎘ Compare Slider]    │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  Cleanliness Meter:  ●●●●○  “Great”        Estimated Time: 02:34             │
│                                                                              │
│  [ CLEAN ]        [ Export ▾ ]        [ Expert Mode ▸ ]                      │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Expert Mode (Collapsed by Default)

```
┌─────────────────────────────────────────────────────────────┐
│ Expert Mode                                                  │
│ ─────────────────────────────────────────────────────────── │
│ Aggressiveness:  [ Low | Standard | High ]                   │
│ Preserve Transients:  ( ON / OFF )                           │
│ Safety Preview:  [ Play Sample ]                             │
└─────────────────────────────────────────────────────────────┘
```

## Visual Design Notes

- **Header:** App branding + project file dropdown + window controls.
- **Main Canvas:** Large waveform + spectrogram stacked; click markers appear as subtle dots.
- **Controls:** Minimal transport controls + “Before/After” toggles + A/B slider.
- **Cleanliness Meter:** Friendly text label and dot scale to communicate confidence.
- **Primary CTA:** “CLEAN” button is large, centered, and visually emphasized.
- **Expert Mode:** Hidden behind a disclosure to avoid overwhelming users.

## Key Interaction Flows

1. **Import audio** → waveform and spectrogram render automatically.
2. **Click “CLEAN”** → animated progress overlay on waveform.
3. **Preview** with before/after toggles or slider.
4. **Export** with format dropdown (WAV/FLAC/AIFF/MP3).

## Responsive Considerations

- On smaller screens, collapse spectrogram into a tab toggle with waveform.
- Move “Cleanliness Meter” and Export into a bottom sheet to preserve canvas space.
