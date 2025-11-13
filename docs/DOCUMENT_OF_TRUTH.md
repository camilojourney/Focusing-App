# DOCUMENT_OF_TRUTH.md — Hyper Awareness System Design & Engineering Roadmap

**Last Updated**: November 3, 2025
**Audience**: Technical co-founders, engineers, Camilo's learning journal
**Status**: Living Document (updated after every major decision)

---

## 1. Vision & Mission

### Why Hyper Awareness Exists

Billions of hours are lost to distraction daily. Knowledge workers don't realize they're distracted *until it's too late*. We're building the **self-awareness engine**—a system that makes the invisible visible.

**Long-term mission**: Transform how humans understand their own productivity through real-time, private, honest feedback.

### How This Connects to Your Engineering Growth (Camilo)

By building Hyper Awareness from zero to production:
- **Desktop**: You'll master modern app architecture (Tauri, IPC, systems programming)
- **Cloud v1.5+**: You'll design distributed systems for data sync, conflict resolution, multi-device support
- **DevOps**: You'll own the entire pipeline from local build to production deployment
- **Performance**: Every decision you make will be constrained by latency, memory, and battery

**Goal by v1.0**: You will not only ship a product but understand how every component works at the systems level.

---

## 2. System Architecture

### High-Level Block Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Hyper Awareness                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐  │
│  │         FRONTEND LAYER (Web UI)                     │  │
│  │  ┌──────────────────────────────────────────────┐  │  │
│  │  │  HTML/CSS/JavaScript (Vanilla, no frameworks)  │  │  │
│  │  │  - Timer Display                              │  │  │
│  │  │  - Check-In UI                               │  │  │
│  │  │  - Settings Panel                            │  │  │
│  │  │  - Session History View                      │  │  │
│  │  └──────────────────────────────────────────────┘  │  │
│  │           ↑ IPC Bridge ↓  (JSON serialization)     │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐  │
│  │         BACKEND LAYER (Rust)                        │  │
│  │  ┌──────────────────────────────────────────────┐  │  │
│  │  │  Command Handlers (@tauri::command macros)   │  │  │
│  │  │  ✓ get_settings()                           │  │  │
│  │  │  ✓ save_settings()                          │  │  │
│  │  │  ✓ update_tray_timer()                      │  │  │
│  │  │  ✓ log_check_in()                          │  │  │
│  │  │  ✓ get_current_event()                     │  │  │
│  │  └──────────────────────────────────────────────┘  │  │
│  │                                                      │  │
│  │  ┌──────────────────────────────────────────────┐  │  │
│  │  │  Core Business Logic                         │  │  │
│  │  │  ✓ Timer State Machine                      │  │  │
│  │  │  ✓ Session Duration Tracking               │  │  │
│  │  │  ✓ Check-in Interval Calculation           │  │  │
│  │  │  ✓ Status Aggregation                      │  │  │
│  │  └──────────────────────────────────────────────┘  │  │
│  │                                                      │  │
│  │  ┌──────────────────────────────────────────────┐  │  │
│  │  │  System Integrations (macOS/Windows)         │  │  │
│  │  │  ✓ System Tray / Menu Bar                   │  │  │
│  │  │  ✓ EventKit (Calendar, macOS only)         │  │  │
│  │  │  ✓ Desktop Switching (AppleScript)         │  │  │
│  │  │  ✓ Native Notifications                    │  │  │
│  │  └──────────────────────────────────────────────┘  │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐  │
│  │         DATA LAYER (Local Storage)                  │  │
│  │  ┌──────────────────────────────────────────────┐  │  │
│  │  │  ~/.config/hyper-awareness/ (Linux/macOS)   │  │  │
│  │  │  %APPDATA%\hyper-awareness\ (Windows)       │  │  │
│  │  │                                              │  │  │
│  │  │  ├── settings.json (User preferences)       │  │  │
│  │  │  ├── focus_log.jsonl (Check-in history)    │  │  │
│  │  │  └── [encrypted/] (Optional encryption)    │  │  │
│  │  └──────────────────────────────────────────────┘  │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  RUNTIME ENVIRONMENT                                        │
│  - Tauri v1.8.3 (webview wrapper)                          │
│  - OS-native WebView (Safari on macOS, Edge on Windows)   │
│  - Tokio runtime (async Rust execution)                   │
└─────────────────────────────────────────────────────────────┘
```

### Component Breakdown

#### Frontend (HTML/CSS/JavaScript)

**Responsibilities:**
- Render user interface (timer, buttons, settings)
- Handle user input (clicks, text input)
- Display real-time timer updates
- Call backend via Tauri IPC

**Key Files:**
- `src/index.html` — Main timer UI
- `src/settings.html` — Settings panel
- `src/assets/` — Stylesheets, images

**Technology Stack:**
- Vanilla JavaScript (no React/Vue)
- HTML5 semantic markup
- CSS3 (Flexbox, CSS Grid)
- Tauri API (`window.__TAURI__.invoke()`)

**Why Vanilla?**
- Smaller bundle size
- Faster startup
- No framework overhead for simple UI
- Easier to debug (direct DOM access)

---

#### Backend (Rust)

**Responsibilities:**
- Execute commands from frontend
- Manage timer state
- Handle file I/O (settings, logs)
- Integrate with OS (calendar, system tray)
- Enforce data integrity

**Key Files:**
- `src-tauri/src/main.rs` — Entry point, command handlers
- `src-tauri/src/calendar.rs` — macOS calendar integration

**Key Structures:**
```rust
#[derive(Serialize, Deserialize)]
struct Settings {
    session_duration: u32,      // Total session time in minutes
    check_in_interval: u32,     // Minutes between check-ins
    write_time: u32,            // Buffer/write timeout
}

#[derive(Serialize, Deserialize)]
struct CheckIn {
    timestamp: String,          // ISO 8601
    status: String,             // "On Task" / "Distracted" / "Break"
    notes: Option<String>,      // User's optional note
}
```

**Technology Stack:**
- Rust (2021 edition)
- Tauri 1.8.3 (IPC, system tray, file paths)
- Serde (JSON serialization)
- Tokio (async runtime, optional for v1.0+)

---

#### Data Layer (Local Storage)

**Format**: JSON + JSONL

**Location (OS-dependent):**
```
macOS/Linux:  ~/.config/hyper-awareness/
Windows:      C:\Users\{user}\AppData\Roaming\hyper-awareness\
```

**Files:**
1. **settings.json** — User preferences (writable by frontend)
   ```json
   {
     "session_duration": 720,
     "check_in_interval": 15,
     "write_time": 25,
     "auto_desktop_switch": true,
     "enable_calendar": true
   }
   ```

2. **focus_log.jsonl** — Append-only check-in history
   ```jsonl
   {"timestamp":"2025-11-03T10:15:00Z","goal":"Thesis Chapter 4","status":"On Task","notes":""}
   {"timestamp":"2025-11-03T10:30:00Z","goal":"Thesis Chapter 4","status":"Social Media","notes":"Got distracted by Twitter"}
   {"timestamp":"2025-11-03T10:45:00Z","goal":"Thesis Chapter 4","status":"On Task","notes":"Refocused"}
   ```

**Why JSONL?**
- Append-only (atomic writes, no full-file rewrites)
- Streaming-friendly (process line-by-line)
- Query-friendly (grep, jq tools)
- Human-readable for debugging

---

### Deployment Topology (v0.1 Alpha - Desktop Only)

```
┌──────────────────────────────────┐
│   User's macOS / Windows Machine │
│                                  │
│  ┌────────────────────────────┐  │
│  │  Hyper Awareness App.exe   │  │
│  │  (Standalone binary)       │  │
│  │  ~3MB                      │  │
│  │                            │  │
│  │  ├─ Tauri WebView         │  │
│  │  ├─ Rust Runtime          │  │
│  │  └─ Native Libraries      │  │
│  │      (Calendar, Tray)     │  │
│  └────────────────────────────┘  │
│                                  │
│  ┌────────────────────────────┐  │
│  │  ~/.config/hyper-          │  │
│  │  awareness/                │  │
│  │  (Local data storage)      │  │
│  └────────────────────────────┘  │
│                                  │
└──────────────────────────────────┘
     No external dependencies
     100% privacy maintained
```

### v1.0+ Distributed Architecture (Planned)

By v1.0, we'll support optional sync to user-owned servers:

```
┌──────────────────────────────────┐
│   User's Desktop (v0.1)           │
│  - Primary data store             │
│  - Full app functionality         │
└──────────────────────────────────┘
           ↓ (optional sync)
┌──────────────────────────────────┐
│   User's Personal Server          │
│   (User-hosted, e.g., NAS)       │
│  - Backup copy of data            │
│  - Cross-device sync point        │
│  - Conflict resolution            │
└──────────────────────────────────┘
           ↓ (optional)
┌──────────────────────────────────┐
│   Mobile Web App                  │
│   (Read-only, or sync-enabled)   │
└──────────────────────────────────┘
```

**Why this architecture?**
- Users own their data completely
- No vendor lock-in
- Works offline 100%
- Opt-in sync (not required)

---

## 3. Communication Patterns

### Inter-Process Communication (IPC): Frontend ↔ Backend

**Protocol**: Tauri IPC (built on WebSockets)

**Serialization**: JSON (automatic via Serde)

**Latency Target**: <10ms (roundtrip)

#### Example 1: Fetching Settings

**Frontend → Backend:**
```javascript
const settings = await window.__TAURI__.invoke('get_settings');
```

**What Tauri Does:**
1. Serialize JavaScript object to JSON
2. Send over IPC bridge
3. Deserialize in Rust (type-safe)
4. Execute `get_settings()` function
5. Serialize result back to JSON
6. Return to JavaScript

**Backend (Rust):**
```rust
#[tauri::command]
fn get_settings(app: AppHandle) -> Result<Settings, String> {
    // Read settings.json from disk
    let settings = load_settings(&app)?;
    Ok(settings)  // Auto-serialized to JSON
}
```

**Latency Breakdown:**
- IPC overhead: ~0.5ms
- File I/O: ~2-3ms
- JSON serialization: ~0.5ms
- **Total**: ~3-4ms

---

#### Example 2: Logging a Check-In (Performance Critical)

**Frontend → Backend:**
```javascript
await window.__TAURI__.invoke('log_check_in', {
    log_line: JSON.stringify({
        timestamp: new Date().toISOString(),
        status: "On Task",
        notes: ""
    })
});
```

**Backend (Rust):**
```rust
#[tauri::command]
fn log_check_in(app: AppHandle, log_line: String) -> Result<(), String> {
    let path = log_file_path(&app)?;

    // Atomic append (critical for reliability)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", log_line)?;  // One syscall
    Ok(())
}
```

**Latency Target**: <5ms (must be fast, called every 15 min)

**Why JSONL Matters Here:**
- Append-only = one write, no read-modify-write
- No locking on the entire file
- Atomic at line boundary

---

### Inter-Service Communication (v1.0+): Desktop ↔ Server

**Protocol**: HTTP/HTTPS REST (later: gRPC for performance)

**Sync Strategy**: Eventual Consistency (CRDTs or Last-Write-Wins)

**Conflict Resolution**:
- Desktop data always wins (user's primary store)
- Server acts as backup/sync point
- Merge on sync: `max(timestamp)` for each entry

---

## 4. Performance & Latency Expectations

### Targets by Component

| Component                   | Target                        | Rationale                        |
| --------------------------- | ----------------------------- | -------------------------------- |
| **App Startup**             | <500ms to menu bar icon       | User perceives "instant"         |
| **Timer Update**            | Every 1s, <1ms latency        | Smooth visual updates            |
| **Check-In Trigger**        | <100ms from interval hit      | Feels responsive                 |
| **Status Button Click**     | <50ms to confirmation         | Snappy UI feedback               |
| **Log Write**               | <5ms to disk                  | Async operation, non-blocking    |
| **Session Review Load**     | <200ms to display 30 sessions | Smooth scrolling                 |
| **Memory (Idle)**           | <50MB RAM                     | Respects background app contract |
| **Memory (Active Session)** | <100MB RAM                    | Session data + UI                |
| **CPU (Idle)**              | <1% when timer running        | Battery-friendly                 |
| **Battery Drain**           | <2% per 8-hour session        | Acceptable for utility app       |

### Profiling Strategy

**For v0.1:**
```bash
# Profile in release mode (not debug)
cargo build --release

# Measure startup time
time ./target/release/hyper-awareness

# Check binary size
ls -lh target/release/hyper-awareness

# Memory usage (macOS)
/usr/bin/time -l ./target/release/hyper-awareness
```

**For v0.2:**
```bash
# Flamegraph: Where is the CPU time spent?
cargo install flamegraph
cargo flamegraph

# Profiling: Detailed latency analysis
cargo bench

# Memory: Heap profiling with Valgrind (Linux) or Instruments (macOS)
```

### Optimization Priorities (by impact)

1. **Startup Time** — App must be visible in 500ms
   - Lazy load settings (don't read all logs on start)
   - Cache calendar events (query only daily)

2. **Check-In Latency** — Must feel instant
   - Pre-allocate timer loop buffers
   - Use JSONL for atomic appends

3. **Memory** — Idle app shouldn't hog resources
   - Drop cached data after 1 minute inactivity
   - Use Arc<Mutex<>> for shared state, not clones

4. **Battery** — Respect mobile-first constraints
   - Timer loop yields to OS scheduler
   - Coalesce file writes when possible

---

## 5. Data Flow Diagram

### Sequence: Check-In Flow

```
Time: T = 10:00 (session start)
      T = 10:15 (check-in interval triggers)

┌─────────────┐                      ┌──────────────┐
│  Frontend   │                      │  Backend     │
│  (JavaScript)                      │  (Rust)      │
└─────────────┘                      └──────────────┘
      │                                    │
      │ Timer fires (10:15)                │
      │─────────────────────────────────>  │
      │ check_in_triggered()               │
      │                                    │ [State: Paused]
      │ <─────────────────────────────────│
      │ Show check-in popup                │
      │                                    │
      │ [User clicks "On Task"]            │
      │─────────────────────────────────>  │
      │ log_check_in({                     │
      │   timestamp: "2025-11-03T10:15Z"  │
      │   status: "On Task",              │
      │   notes: ""                        │
      │ })                                 │
      │                                    │ [Write to focus_log.jsonl]
      │ <─────────────────────────────────│
      │ Session resumes (OK)               │
      │                                    │ [State: Running]
      │                                    │
      │ update_tray_timer("10:15")        │
      │─────────────────────────────────> │
      │ Update menu bar display            │
      │ <─────────────────────────────────│
      │ "🧠 On Task • 10:15"              │
      │                                    │
```

### State Machine: Timer States

```
┌─────────────┐
│   IDLE      │  (App started, no session)
└──────┬──────┘
       │ user_start_session()
       ▼
┌─────────────┐
│  RUNNING    │  (Timer counting)
└──────┬──────┘
       │
       ├── check_in_interval_hit() ──────┐
       │                                 ▼
       │                          ┌──────────────┐
       │                          │  CHECK_IN    │  (Waiting for user response)
       │                          └──────┬───────┘
       │                                 │
       │                    user_responds_to_checkin()
       │                                 │
       └─────────────────────────────────┘
       │
       │ session_complete()
       ▼
┌─────────────┐
│  COMPLETE   │  (Session ended, show summary)
└─────────────┘
```

---

## 6. Deployment & CI/CD Strategy

### Local Development → GitHub → Signed Release

```
┌─────────────────────────────────────┐
│  1. Developer Local Workflow        │
│  ─────────────────────────────────  │
│  $ git commit                       │
│  $ git push origin feature-branch   │
└──────────────────┬──────────────────┘
                   │
┌──────────────────▼──────────────────┐
│  2. GitHub CI Pipeline              │
│  ─────────────────────────────────  │
│  .github/workflows/build.yml        │
│                                     │
│  Jobs:                              │
│  - Run cargo check                 │
│  - Run cargo test                  │
│  - Build release binaries          │
│  - Sign macOS binary               │
│  - Sign Windows binary             │
│  - Upload artifacts                │
└──────────────────┬──────────────────┘
                   │
┌──────────────────▼──────────────────┐
│  3. Pull Request Review             │
│  ─────────────────────────────────  │
│  Require:                           │
│  - Code review (1+ humans)         │
│  - CI pass (all checks green)      │
│  - Clippy no warnings              │
│  - Tests pass (≥70% coverage)      │
└──────────────────┬──────────────────┘
                   │ (on approval + merge to main)
┌──────────────────▼──────────────────┐
│  4. Release Build (main branch)      │
│  ─────────────────────────────────  │
│  - Tag: v0.1.0                     │
│  - Build final binaries            │
│  - Generate changelog              │
│  - Create GitHub Release           │
│  - Upload .dmg, .exe               │
└──────────────────┬──────────────────┘
                   │
┌──────────────────▼──────────────────┐
│  5. Distribution                    │
│  ─────────────────────────────────  │
│  - Users download from GitHub      │
│  - Verify code signature           │
│  - Install .dmg or .exe            │
└──────────────────────────────────────┘
```

### GitHub Actions Workflow Structure

**File**: `.github/workflows/build.yml`

```yaml
name: Build & Release

on:
  push:
    branches: [main, develop]
    tags: ['v*']
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo check
      - run: cargo test
      - run: cargo clippy -- -D warnings

  build:
    needs: test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - run: npm install
      - run: npm run build
      - uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.os }}-build
          path: src-tauri/target/release/bundle/

  release:
    needs: build
    if: startsWith(github.ref, 'refs/tags/')
    runs-on: ubuntu-latest
    steps:
      - uses: softprops/action-gh-release@v1
        with:
          files: |
            src-tauri/target/release/bundle/**/*.dmg
            src-tauri/target/release/bundle/**/*.exe
```

### Code Signing (Security Critical)

**macOS Signing:**
```bash
# Developer ID Certificate (from Apple)
codesign -s "Developer ID Application: Camilo Martinez" \
  --options=runtime \
  --entitlements entitlements.plist \
  --timestamp \
  "Hyper Awareness.app"

# Verify signature
codesign -v "Hyper Awareness.app"
```

**Windows Signing:**
```batch
# Requires Sectigo or DigiCert certificate
signtool sign /f cert.pfx /p password /t http://timestamp.server \
  hyper-awareness.exe
```

**Why?**
- macOS Gatekeeper: Unsigned apps get "unknown developer" warning
- Windows SmartScreen: Unsigned apps trigger alerts
- User Trust: Signature proves the app is from us

---

## 7. Theoretical Knowledge Checklist (Mastery Goals for Camilo)

### Operating Systems Fundamentals

**Must Understand:**
- [ ] Process vs Thread (what's the difference?)
- [ ] Stack vs Heap (where does Rust data live?)
- [ ] Page Tables & Virtual Memory (why does app start slow?)
- [ ] System Calls (timer syscalls, file I/O syscalls)
- [ ] Interrupts & Event Loops (how does check-in trigger work?)
- [ ] Process Scheduling (why does CPU spike sometimes?)

**Resources:**
- *Operating Systems: Three Easy Pieces* (Remzi Arpaci-Dusseau)
- Linux man pages: `man 7 pthreads`, `man 2 write`
- YouTube: "How operating systems work" (MIT OpenCourseWare)

**Why It Matters for Hyper Awareness:**
- Understand memory usage: Know when app is bloated
- Optimize file I/O: Know syscall overhead
- Debug performance: Understand where latency comes from

---

### Networking (for v1.0+ Sync)

**Must Understand:**
- [ ] OSI 7-Layer Model (where does HTTP sit?)
- [ ] TCP/IP (connection reliability)
- [ ] DNS (how do we find user's server?)
- [ ] TLS/SSL (how do we encrypt sync data?)
- [ ] HTTP vs gRPC (trade-offs for sync?)
- [ ] REST design principles (designing sync API)

**Resources:**
- *Computer Networking: A Top-Down Approach* (Kurose & Ross)
- RFC 7230 (HTTP/1.1 specification)
- gRPC documentation (https://grpc.io/)

**Why It Matters:**
- Design sync protocol correctly (first time, not later)
- Understand conflict resolution constraints
- Know latency/bandwidth trade-offs

---

### Distributed Systems (for v1.0+)

**Must Understand:**
- [ ] CAP Theorem (Consistency, Availability, Partition tolerance)
- [ ] Eventually Consistent Systems (how do conflicts happen?)
- [ ] CRDTs (Conflict-free Replicated Data Types)
- [ ] Vector Clocks (causality & ordering)
- [ ] Consensus Algorithms (Raft, Paxos)
- [ ] Event Ordering (happens-before relation)

**Resources:**
- *Designing Data-Intensive Applications* (Martin Kleppmann) — **ESSENTIAL**
- Paper: "CRDTs: Conflict-free Replicated Data Types" (Shapiro et al.)
- Paper: "Raft: In Search of an Understandable Consensus Algorithm" (Ongaro & Ousterhout)
- YouTube: "Distributed Systems" course (6.824 MIT)

**Why It Matters:**
- Sync architecture must handle offline + multi-device
- Decide: Last-write-wins? CRDTs? Consensus?
- Avoid silent data loss (the nightmare scenario)

---

### DevOps & Systems Reliability

**Must Understand:**
- [ ] Containerization (Docker fundamentals)
- [ ] Container Orchestration (Kubernetes basics)
- [ ] Service Levels: SLI, SLO, SLA
- [ ] Monitoring & Observability (metrics, logs, traces)
- [ ] Incident Response (on-call procedures)
- [ ] CI/CD Best Practices (automated testing, canary releases)

**Resources:**
- *The Phoenix Project* (Gene Kim) — DevOps culture
- *Site Reliability Engineering* (Google, free online)
- Docker documentation
- Kubernetes documentation (conceptual overview first)

**Why It Matters:**
- Build infrastructure that doesn't wake you up at 3 AM
- Monitor in production (know when data syncing fails)
- Automate release process (reduce human error)

---

### Performance Engineering

**Must Understand:**
- [ ] Profiling & Benchmarking (flamegraph, perf)
- [ ] Latency Analysis (p99, p95, jitter)
- [ ] Algorithmic Complexity (Big-O analysis)
- [ ] Memory Profiling (heap allocation patterns)
- [ ] Cache Efficiency (L1/L2/L3, page faults)
- [ ] Concurrency Performance (locks, contention)

**Resources:**
- *Designing for Performance* (Cody P. Littley)
- *BPF Performance Tools* (Brendan Gregg)
- flamegraph: https://www.brendangregg.com/flamegraphs.html
- perf: https://perf.wiki.kernel.org/

**Why It Matters:**
- Hit <50MB idle memory target (know every allocation)
- Debug latency spikes (profile, not guess)
- Battery drain (CPU efficiency matters)

---

### Rust Systems Programming

**Must Understand:**
- [ ] Ownership & Borrowing (memory safety without GC)
- [ ] Lifetime System (explicit resource management)
- [ ] Trait System (generic programming, interface design)
- [ ] Error Handling (Result<T, E> idiomatic patterns)
- [ ] Concurrency (channels, Mutex, Arc)
- [ ] Async/Await (futures, tokio runtime)
- [ ] Unsafe Rust (when/why it's necessary)
- [ ] FFI (calling C/Objective-C, calling Rust from C)

**Resources:**
- *The Rust Programming Language* (Steve Klabnik & Carol Nichols)
- *Rust for Rustaceans* (Jon Gjengset)
- Tokio tutorial: https://tokio.rs/

**Why It Matters:**
- Prevent entire classes of bugs (memory safety)
- Write performant async code (v1.0+ sync)
- Understand compile-time guarantees

---

## 8. Lessons & Key Decisions Log

### Decision: Why JSONL vs Database?

**Considered Options:**
1. SQLite
2. PostgreSQL (early, rejected)
3. JSONL (chosen)

**Why JSONL?**
- ✅ Zero dependencies (no sqlite library)
- ✅ Privacy: Users can inspect files directly (cat, grep)
- ✅ Append-only: Atomic writes without locks
- ✅ Portable: Copy to new computer easily
- ✅ Analyzable: Feed to Python/R/Pandas

**Tradeoff:**
- ❌ No complex queries (no WHERE, JOINs)
- ❌ Slower for millions of rows (index lookup)

**Decision Logic:**
For MVP (v0.1), simplicity > queryability. We can migrate to SQLite in v1.0 if needed.

---

### Decision: Why Tauri vs Electron?

**Considered Options:**
1. Electron (most popular, but bloated)
2. Qt (C++, steeper learning curve)
3. Tauri (chosen)

**Why Tauri?**
- ✅ Small bundle (~3MB vs Electron's 120MB)
- ✅ Rust safety (no segfaults in backend)
- ✅ Native system integration (menu bar, calendar)
- ✅ Fast startup
- ✅ Low resource usage

**Tradeoff:**
- ❌ Smaller ecosystem than Electron
- ❌ Requires Rust knowledge
- ❌ Less mature (but stable enough for MVP)

**Decision Logic:**
Privacy-conscious users (our target) care about small app size, resource usage, and trust in code. Tauri excels there.

---

### Decision: Why Vanilla JS vs React/Vue?

**Considered Options:**
1. React (industry standard)
2. Vue (simpler learning curve)
3. Vanilla JS (chosen)

**Why Vanilla JS?**
- ✅ Zero dependencies (faster startup, smaller bundle)
- ✅ Direct DOM control (easier to debug)
- ✅ Lower complexity (for simple UI)
- ✅ Easier to learn Tauri (fewer moving parts)

**Tradeoff:**
- ❌ Manual DOM updates (more code)
- ❌ No component reusability (yet)
- ❌ Harder to scale (if UI becomes complex)

**Decision Logic:**
MVP UI is simple: timer, buttons, text inputs. Framework overhead isn't justified. Migrate to React in v1.0 if UI complexity warrants it.

---

### Lesson: Icon Template Mode Caused 2-Day Debug

**Problem:**
Menu bar icon was invisible after build.

**Root Cause:**
macOS requires template images (pure black on transparent) for menu bar. Icon was correctly set as template, but:
1. Icon had anti-aliasing (gray pixels)
2. Tauri's `set_icon_as_template(true)` was hiding it further

**Solution:**
Removed `set_icon_as_template(true)` and ensured icon was pure black + transparent.

**Lesson:**
- Platform-specific behavior is subtle
- Read Apple/Microsoft documentation carefully
- Test on actual platform (not just dev build)
- Icon formats matter (PNG metadata, bit depth)

---

### Lesson: IPC Serialization Adds Latency

**Problem:**
Check-in logging was taking 50ms instead of target 5ms.

**Root Cause:**
Each IPC call serialized/deserialized JSON. Profiling showed:
- Serialization: 30ms
- IPC overhead: 10ms
- File I/O: 10ms

**Solution:**
Batch multiple check-ins into single IPC call (future optimization).

**Lesson:**
- IPC is not free (design for minimal crossings)
- Serialization cost scales with data size
- Profile early (don't assume speed)

---

## 9. References & Learning Sources

### Essential Reading

| Resource                                            | Duration | Topic                                  | Why It Matters                                 |
| --------------------------------------------------- | -------- | -------------------------------------- | ---------------------------------------------- |
| *Designing Data-Intensive Applications* (Kleppmann) | 40 hrs   | Distributed systems, data architecture | Foundation for v1.0 sync design                |
| *Operating Systems: Three Easy Pieces*              | 30 hrs   | OS internals, concurrency, persistence | Deep understanding of system calls, scheduling |
| *The Rust Programming Language* (Klabnik)           | 20 hrs   | Rust fundamentals                      | Core skill for backend development             |
| *The Phoenix Project* (Gene Kim)                    | 6 hrs    | DevOps culture, automation             | Mindset for infrastructure                     |
| *Site Reliability Engineering* (Google)             | 15 hrs   | Monitoring, incident response, scaling | How to run production systems                  |

### Online Resources

| Resource                                                         | Format        | Topic                                    |
| ---------------------------------------------------------------- | ------------- | ---------------------------------------- |
| Tauri Docs (https://tauri.app/)                                  | Documentation | Tauri framework, IPC, system integration |
| Tokio Tutorial (https://tokio.rs/)                               | Interactive   | Async Rust, concurrency                  |
| Linux Man Pages                                                  | Reference     | System calls, file I/O, signals          |
| YouTube: 6.824 MIT (https://www.youtube.com/watch?v=cQP8WApzIQQ) | Video         | Distributed systems theory               |
| Brendan Gregg's Blog (https://www.brendangregg.com/)             | Blog          | Performance profiling, flame graphs      |

### Papers (Distributed Systems Core)

1. **"CRDTs: Conflict-free Replicated Data Types"** (Shapiro et al.)
   - Why: Design sync without consensus
   - Difficulty: Medium
   - Time: 2 hrs

2. **"Raft: In Search of an Understandable Consensus Algorithm"** (Ongaro & Ousterhout)
   - Why: Alternative to CRDTs (if you need strong consistency)
   - Difficulty: Medium
   - Time: 1.5 hrs

3. **"Bigtable: A Distributed Storage System for Structured Data"** (Google)
   - Why: Learn how Google handles distributed data
   - Difficulty: Hard
   - Time: 2 hrs

---

## 10. Current Architecture Status

### v0.1 Alpha (Current Sprint)

**Implemented:**
- ✅ Menu bar icon (fixed template mode issue)
- ✅ Basic timer UI
- ✅ Local settings storage (JSON)
- ✅ Check-in logging (JSONL)
- ✅ macOS system tray integration

**In Progress:**
- 🚧 Settings panel UI
- 🚧 Session history view
- 🚧 Windows platform support

**Not Started:**
- ⏳ Data export
- ⏳ Analytics/insights
- ⏳ Sync (v1.0+)

### Known Limitations

1. **No Encryption**: Settings/logs are plaintext (ok for MVP, adds for v0.2)
2. **Single User**: No multi-user profiles (ok for desktop, needed for web)
3. **No Cloud Sync**: Data stuck on one device (feature for v1.0+)
4. **Limited Analytics**: Basic aggregate stats only (v0.2)
5. **No Notifications**: OS notifications not wired (v0.2)

---

## 11. The Engineering Roadmap (Aligned with Mastery)

### By End of v0.1 (Week 6)
**You Will Master:**
- Tauri IPC patterns
- Rust command macros & type serialization
- System tray integration (macOS)
- File I/O error handling
- Local development workflow

**Ship:** Working timer + logging

---

### By End of v0.2 (Week 10)
**You Will Master:**
- Performance profiling (flamegraph, benchmarks)
- Cross-platform development (macOS + Windows)
- Testing strategies (unit + integration)
- Async Rust (tokio basics)
- GitHub Actions CI/CD

**Ship:** Multi-platform app + analytics

---

### By End of v1.0 (Week 14)
**You Will Master:**
- Distributed systems design (sync protocol)
- Conflict resolution (CRDTs or LWW)
- Code signing & secure release
- Monitoring & observability
- DevOps best practices

**Ship:** Production-ready app + optional sync

---

## Final Word

This document is your north star. Refer to it when making decisions. Update it as you learn. By shipping v1.0, you will have built not just an app, but a deep understanding of systems engineering from OS calls to distributed consensus.

**Question yourself often: "Does this decision align with the vision and principles?"**

Ship fast, learn continuously, and make Hyper Awareness the most reliable, privacy-respecting productivity tool ever built.
