# MVP.md — Focus Time: Minimum Viable Product

**Last Updated**: November 18, 2025
**Target Ship Date**: ✅ v1.0 Shipped
**Owner**: Camilo Martinez
**Current Status**: v1.0 Production - All Phase 1 Goals Complete

---

## Problem Statement

Knowledge workers and computer professionals lose awareness of their actual work patterns, spending hours distracted without realizing it. They need a **privacy-first, locally-running system** that makes their cognitive behavior visible—tracking intention vs. reality in real-time—without sending data to external servers.

---

## Vision

**Focus Time (Hyper Awareness) is the self-awareness engine for deep work.**

v1.0 delivers on the core promise: a native macOS menu bar application that tracks your intentions vs. actual behavior with zero cloud dependencies. The app is privacy-first, locally-stored, and provides real-time metacognition through periodic check-ins.

## Current Implementation Status (v1.0 Production - SHIPPED ✅)

**What's Working:**
- ✅ Menu bar timer with live countdown display (e.g., "14:32")
- ✅ Session goal tracking with calendar integration
- ✅ Interactive check-ins every 15 minutes (6 status options)
- ✅ Local data logging (JSONL format with session history)
- ✅ **Session Review Panel** - Timeline visualization with statistics
- ✅ Settings persistence with full customization
- ✅ macOS calendar integration (EventKit with permission flow)
- ✅ **Window positioning** - Centered on check-ins, draggable during interaction
- ✅ **Auto-hide behavior** - Window hides after Start and check-in responses
- ✅ **Standard macOS window decorations** - Close/minimize/maximize buttons
- ✅ **macOS Sequoia compatible** - Fixed tray icon rendering for macOS 15.x
- ✅ Privacy-first local storage (no cloud, no telemetry)

**Platform Support:**
- ✅ macOS 11+ (full functionality, tested on Sequoia 15.x)
- ⏳ Windows/Linux (not currently supported, future roadmap)

**Recent Technical Achievements (November 2025):**
- Fixed tray icon rendering on macOS Sequoia using `icon_as_template(false)`
- Implemented three-mode window positioning (centered, tray-relative, draggable)
- Created modular Rust architecture (main.rs, calendar.rs, logs.rs)
- Upgraded to Tauri v2.1.0 for improved performance and security

---

## Core Value Proposition

1. **Privacy-First**: All data stays on the user's machine by default. Zero cloud dependency for core functionality.
2. **Behavioral Clarity**: Compare intention (what you planned to do) vs. reality (what you actually did) with minimal friction.
3. **Desktop Native + Cloud Optional**: Ship as a macOS/Windows menu bar app first; add web/mobile later without breaking privacy promises.
4. **Offline-First Sync**: Collect data locally, optionally sync to personal cloud (user-owned server, not our servers).

---

## Key User Flows

### Flow 1: The Focus Session

```
User Sets Goal
  ↓
[Types: "Finish Chapter 4 of Thesis"]
  ↓
Starts Session (Timer begins, check-in interval starts)
  ↓
[Works for default 15 minutes]
  ↓
Check-In Triggered (Desktop auto-switches to Desktop 1, check-in UI appears)
  ↓
User Reports Status
  [Clicks: "✅ On Task" / "📱 Social Media" / "📧 Email/Chat" / "☕️ Break" / "🔀 Other Distraction"]
  ↓
Optional: Quick Note
  [Writes: "Got distracted by Slack, refocused"]
  ↓
Data Logged Locally (JSONL file to ~/Library/Application Support/com.focustime.app/)
  ↓
Session Auto-Resumes (20 second write time, then timer continues)
  ↓
[Repeat check-ins every 15 minutes until session duration complete (default 12 hours)]
```

**Optional Accountability Box:**
```
User Clicks "Accountability Box" Button
  ↓
Modal Appears with 5 Deep Reflection Questions:
  1. What did you accomplish today?
  2. What challenges did you face?
  3. What will you do tomorrow?
  4. How focused were you? (1-10)
  5. What did you learn?
  ↓
User Types Responses
  ↓
Responses Saved to accountability_box.jsonl
  ↓
User Continues Session
```

### Flow 2: Review & Insights (Planned for v0.2)

```
User Opens Session Review Panel
  ↓
Views Current Session Summary
  [See list of all check-ins with timestamps, statuses, and notes]
  ↓
Summary Statistics Displayed
  [On Task count vs. Distraction count + Break count]
  ↓
Optional: Export Data
  [Access JSONL files directly for analysis in Python/R/Excel]
```

**Current Workaround:**
Users can directly access their data files:
- Check-ins: `~/Library/Application Support/com.focustime.app/focus_log.jsonl`
- Accountability: `~/Library/Application Support/com.focustime.app/accountability_box.jsonl`
- Settings: `~/Library/Application Support/com.focustime.app/settings.json`

### Flow 3: Settings & Customization

```
User Opens Settings
  ↓
Configures:
  - Session Duration (default 12 hours)
  - Check-In Interval (default 15 min)
  - Auto-Desktop Switch (on/off)
  - Calendar Integration (on/off)
  - Data Export Location
  ↓
Saves → Settings persisted locally
```

---

## MVP Feature List

### Tier 1 (Must Have for v0.1 Alpha) — ✅ COMPLETE

| Feature                        | Status | Why                                            | Technical Scope                                      |
| ------------------------------ | ------ | ---------------------------------------------- | ---------------------------------------------------- |
| **Menu Bar Timer**             | ✅      | Always visible, non-intrusive                  | Tauri SystemTray + Rust backend                      |
| **Goal Setting UI**            | ✅      | Users can set intention                        | HTML input field, passed to Rust backend             |
| **Check-In Prompts**           | ✅      | Triggers at intervals                          | JavaScript timer + Rust desktop switching            |
| **Status Buttons**             | ✅      | Quick reporting (On Task / Distracted / Break) | Vanilla JS UI updates + IPC to Rust                  |
| **Local Data Logging (JSONL)** | ✅      | Privacy-first storage                          | File I/O to `~/Library/Application Support/...`      |
| **Settings Persistence**       | ✅      | User preferences saved                         | JSON file storage via Rust commands                  |
| **macOS Integration**          | ✅      | System tray icon + Desktop switch + Calendar   | EventKit calendar API + AppleScript desktop switch   |
| **Accountability Box**         | ✅      | Deep reflection and self-awareness             | Modal UI + JSONL logging of 5 reflection questions   |
| **Adaptive Menu Bar Icon**     | ✅      | Theme-aware icon (dark/light mode)             | macOS template images (18x18.png)                    |

**Ship Criterion**: ✅ ACHIEVED - App starts, user can set goal, track sessions, report status, data logs locally, calendar integrates, desktop switches work.

---

### Tier 2 (Should Have for v0.2 Beta) — 🚧 IN PROGRESS

| Feature                         | Status | Why                       | Technical Scope                                     |
| ------------------------------- | ------ | ------------------------- | --------------------------------------------------- |
| **Session Review Panel**        | 🚧      | See current session       | Read JSONL since session start, display in drawer   |
| **Session History UI**          | ⏳      | See past sessions         | Read JSONL, display stats by date                   |
| **Daily/Weekly Summary**        | ⏳      | Trends over time          | Aggregate logged data by day/week                   |
| **Data Export (CSV/JSON)**      | ⏳      | Interoperability          | Parse JSONL, format output for Excel/analysis tools |
| **Custom Intervals**            | ⏳      | Power users' needs        | Settings validation + timer recalc                  |
| **Calendar Event Display**      | ✅      | Context for check-ins     | ✅ Already implemented via EventKit                 |
| **Windows Support**             | ⏳      | Expand platform reach     | Tauri + Windows API adapters                        |
| **Performance Optimization**    | ⏳      | <50MB idle, <5ms logging  | Profiling + optimization                            |

**Ship Criterion**: Full session review drawer functional, Windows build working, performance targets met (<50MB idle memory, <5ms log writes).

---

### Tier 3 (Nice to Have for v1.0)

| Feature                     | Why                        | Technical Scope                         |
| --------------------------- | -------------------------- | --------------------------------------- |
| **Data Encryption**         | Enhanced privacy           | AES-256, encryption-at-rest             |
| **Personal Server Sync**    | Optional cloud, user-owned | HTTP sync protocol, conflict resolution |
| **API for Custom Analysis** | Developer extensibility    | REST API, gRPC option                   |
| **AI Insights (Optional)**  | Behavioral patterns        | Local model inference (offline)         |
| **Mobile Web App**          | Access anywhere            | React web frontend, same backend API    |
| **Notifications**           | Better UX                  | Native OS notifications                 |
| **Dark Mode**               | Modern UX                  | CSS theme toggle                        |

---

## Out-of-Scope (Explicitly NOT in MVP)

| What                        | Why Not                           | When                           |
| --------------------------- | --------------------------------- | ------------------------------ |
| **Cloud-First Sync**        | Violates privacy promise          | After v1.0, user-opted only    |
| **Proprietary Server**      | We don't own user data            | Never; users own their data    |
| **ML Model Training**       | We don't collect aggregated data  | Never without explicit consent |
| **Mobile Native Apps**      | Too much scope; web first         | v1.5+, after desktop stable    |
| **Slack/Email Integration** | Privacy nightmare                 | v2.0+, if at all               |
| **Real-Time Collaboration** | Out of scope for MVP              | v2.0+, optional feature        |
| **Browser Extension**       | Limited control, privacy concerns | v2.0+, if users demand it      |

---

## Definition of Done

### Code Quality

- [ ] **No compiler warnings** (Rust `cargo check` passes clean)
- [ ] **Test coverage ≥ 70%** (unit tests for critical paths)
- [ ] **Type safety**: All Rust code uses `Result<T, E>` for error handling
- [ ] **Zero unsafe Rust** (except platform-specific code, reviewed)
- [ ] **Linting passes**: `cargo clippy` with no errors

### Functional Requirements

- [ ] **Timer logic**: Accurate to ±1 second over 12 hours
- [ ] **Data persistence**: Settings and logs survive app restart
- [ ] **IPC reliability**: No dropped commands between frontend/backend
- [ ] **File I/O**: Concurrent writes to log file don't corrupt data

### Performance

- [ ] **Startup time**: App visible in menu bar within 500ms
- [ ] **Memory**: Idle <50MB RAM
- [ ] **Battery**: <2% CPU usage when idle
- [ ] **Logging**: Append 1 check-in entry in <5ms

### Platform Integration

- [ ] **macOS**: Menu bar icon visible, calendar integration works
- [ ] **Windows**: Menu bar equivalent (system tray) working
- [ ] **Data location**: Uses OS-standard config directories

### User Testing

- [ ] **3+ beta users** test for 1 week
- [ ] **Zero data loss** in beta
- [ ] **Usability**: Goal → Check-In → Review in <2 minutes

### DevOps & Release

- [ ] **Build automation**: GitHub Actions builds on every commit
- [ ] **Signed binaries**: Macros code-signed, Windows code-signed
- [ ] **Release notes**: Clear changelog
- [ ] **Installation**: One-click `.dmg` or `.exe` install

---

## Success Metrics

### Quantitative

| Metric             | Target                             | Measurement                    |
| ------------------ | ---------------------------------- | ------------------------------ |
| **Data Accuracy**  | ±2% error in check-in detection    | Validate 10 sessions manually  |
| **App Stability**  | 99.9% uptime (no crashes)          | Run for 168 hours (1 week)     |
| **Performance**    | <100ms latency for check-in action | Measure keystroke → logged     |
| **Beta Retention** | 80% of users session ≥2 weeks      | Track daily active beta users  |
| **Privacy Score**  | 0 KB data sent externally          | Network monitoring (Wireshark) |

### Qualitative

| Metric              | Target                                      | Method                                      |
| ------------------- | ------------------------------------------- | ------------------------------------------- |
| **Ease of Use**     | User sets goal + runs session without docs  | Observe 3 new users, no guidance            |
| **Design Clarity**  | Users understand status buttons intuitively | Ask beta: "What does this button do?"       |
| **Perceived Value** | User feels "more aware" after 1 week        | Post-session survey: "Did this help?"       |
| **Trustworthiness** | User believes data is private               | Survey: "Does this app respect my privacy?" |

---

## Learning Focus (For Camilo)

To ship this MVP and prepare for v1.0's distributed systems architecture, **you must master**:

### 1. **Desktop App Architecture** (Tauri Deep Dive)
- IPC patterns: frontend → backend communication
- State management across processes
- File I/O error handling (race conditions, permissions)
- Platform-specific APIs (macOS/Windows differences)
- **Why**: Core skill to ship v0.1 reliably

### 2. **Systems Programming (Rust Fundamentals)**
- Ownership & borrowing
- Error handling (`Result<T, E>`, proper `?` operator usage)
- Async/await (`tokio` runtime)
- Concurrency (channels, mutexes)
- **Why**: Write performant, crash-free backend

### 3. **Performance Optimization**
- Profiling tools (`cargo flamegraph`, `perf`)
- Latency analysis (Why did that function take 50ms?)
- Memory optimization (heap allocations, lifetimes)
- **Why**: Achieve <50MB idle memory, <5ms logging

### 4. **DevOps Fundamentals**
- GitHub Actions CI/CD
- Code signing (macOS/Windows)
- Semantic versioning & release management
- Build reproducibility
- **Why**: Automate releases, ensure users get secure binaries

### 5. **Data Persistence**
- JSONL format design
- Atomic writes (prevent corruption)
- Query & aggregation patterns
- Encryption at rest (optional for v0.2)
- **Why**: Core to logging, privacy

### 6. **Testing Disciplines**
- Unit tests (Rust: `#[test]`)
- Integration tests
- End-to-end tests
- Data migration tests
- **Why**: Ship without data loss bugs

---

## Success Roadmap by Phase

### Phase 0 (Week 1-2): Foundation
- [ ] Prototype timer logic (no UI)
- [ ] Set up GitHub Actions CI
- [ ] Establish JSONL logging schema
- **Learning**: Tauri command system, file I/O patterns

### Phase 1 (Week 3-6): Alpha (v0.1)
- [ ] Working timer + menu bar UI
- [ ] Check-in flow with status reporting
- [ ] Local data logging
- [ ] Basic testing
- **Learning**: IPC, state management, concurrency

### Phase 2 (Week 7-10): Beta (v0.2)
- [ ] Session history & summary views
- [ ] Data export (CSV/JSON)
- [ ] Platform support (macOS + Windows)
- [ ] Performance optimization
- **Learning**: Performance profiling, cross-platform debugging

### Phase 3 (Week 11-14): Release (v1.0)
- [ ] User testing & feedback iteration
- [ ] Signed, distributable binaries
- [ ] Documentation & help system
- [ ] Automated release pipeline
- **Learning**: DevOps maturity, release management

---

## Technical Decisions (Architectural North Star)

### Why Tauri?
- Small bundle (~3MB) for privacy-conscious users
- Rust backend = no runtime vulnerabilities
- Native system integration (menu bar, calendar, notifications)

### Why Local-First?
- No privacy debates; data ownership is clear
- Works offline from day one
- Sync is optional, not required (resilience)

### Why JSONL?
- Append-only = fast writes
- No parsing overhead
- Streaming-friendly (analytics tools)
- Human-readable for debugging

### Why Rust Backend?
- Performance critical for on-device inference (future)
- Memory safety prevents crashes
- Strong type system catches bugs at compile time

---

## Risks & Mitigation

| Risk                                         | Impact                      | Mitigation                                                | Status  |
| -------------------------------------------- | --------------------------- | --------------------------------------------------------- | ------- |
| **Data Corruption** (concurrent writes)      | Users lose check-in history | ✅ Atomic writes implemented, JSONL append-only           | ✅ Solved |
| **Platform Inconsistency** (macOS ≠ Windows) | Feature gaps, angry users   | 🚧 Abstract platform layer, conditional feature flags     | 🚧 Active |
| **Performance Degradation** (timer drift)    | Data becomes unreliable     | ⏳ Need profiling benchmarks                              | ⏳ TODO  |
| **User Confusion** (UI unclear)              | Beta churn                  | ⏳ Usability testing with 3+ external users needed        | ⏳ TODO  |
| **Scope Creep**                              | Miss ship date              | ✅ Tier 1 complete, now focused on Tier 2                 | ✅ Managed |
| **Icon Visibility Issues**                   | App unusable                | ✅ Solved with macOS template images                      | ✅ Solved |
| **Calendar Permission Friction**             | Users can't access calendar | ✅ Permission request flow implemented                    | ✅ Solved |

---

## Next Steps

1. **Code Review**: Review this MVP with 1-2 technical advisors
2. **Prototype**: Build Phase 0 (timer + logging) in next 2 weeks
3. **Validate**: Test with 3 beta users by end of Month 1
4. **Refine**: Iterate based on feedback
5. **Ship**: v0.1 Alpha by end of Phase 1

---

**By shipping this MVP, you will have:**
- ✅ A working privacy-first desktop app
- ✅ A foundation for distributed systems (v1.0)
- ✅ Deep understanding of Tauri, Rust, and IPC
- ✅ DevOps basics (CI, signing, releases)
- ✅ Proof of concept for local-first architecture

**Remember**: Done > Perfect. Ship v0.1, learn from users, iterate to v1.0.
