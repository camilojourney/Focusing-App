# ROADMAP.md — Focus Time: Execution & Mastery Plan

**Last Updated**: November 13, 2025
**Ship Goal**: v1.0 by Q1 2026
**Owner**: Camilo Martinez
**Status**: ACTIVE
**Current Phase**: v0.1 Alpha COMPLETE ✅ | v0.2 Beta IN PROGRESS 🚧

---

## 1. Major Milestones (High-Level)

### v0.1 Alpha: "It Works" ✅ COMPLETE (Shipped: November 2025)
**Goal**: Prove core concept works. Build-test-ship cycle.

**What Shipped:**
- ✅ Menu bar timer visible and updating every 1 second
- ✅ User can set goal and track sessions
- ✅ Check-ins log to disk (JSONL format)
- ✅ Settings persistence (JSON file)
- ✅ macOS calendar integration (EventKit)
- ✅ Automatic desktop switching (AppleScript)
- ✅ Accountability Box with 5 reflection questions
- ✅ Adaptive menu bar icon (theme-aware)
- ✅ Privacy-first local storage

**Bonus Features Shipped:**
- ✅ Calendar permission management
- ✅ Multiple distraction status types
- ✅ Optional notes for each check-in
- ✅ Settings validation and error handling

**Success Metrics Achieved:**
- ✅ Build succeeds with zero compiler warnings
- ✅ App runs stable for extended sessions
- ✅ Data persists correctly across restarts
- ✅ Menu bar icon visibility issue resolved

---

### v0.2 Beta: "It's Useful" 🚧 IN PROGRESS (Target: December 2025)
**Goal**: Multi-platform, analytics, professional UX.

**What's Shipping:**
- 🚧 Session Review Panel (Spec 001) - IN PROGRESS
  - View current session check-ins in drawer
  - Summary statistics (on-task vs. distraction count)
  - Real-time updates as check-ins occur
- ⏳ Session History UI (past sessions)
- ⏳ Daily/Weekly Summary (aggregated analytics)
- ⏳ Data Export (CSV/JSON)
- ⏳ Performance Optimization (<50MB idle, <5ms logging)
- ⏳ Windows Support (feature parity with macOS)
- ⏳ GitHub Actions CI/CD (automated releases)

**What's Deferred:**
- Cloud sync (v1.0)
- Encryption at rest (v1.0)
- Mobile (v1.5+)

**Success Metrics (TBD):**
- 10+ external beta testers
- <1% data loss rate
- <5ms check-in logging latency
- Cross-platform feature parity (macOS + Windows)

---

### v1.0 Release: "Production Ready" (Target: Week 14)
**Goal**: Hardened, distributed-systems-ready, shipping with confidence.

**What Ships:**
- Personal server sync (opt-in, user-hosted)
- Encryption at rest (settings + logs)
- Comprehensive monitoring & error recovery
- Code-signed, versioned releases
- Full documentation + user guides

**Not Included:**
- Mobile apps (future)
- AI insights (future)
- Enterprise features (future)

**Success Metric:**
- 100+ active users
- 99.9% uptime SLO met
- Zero unplanned data loss incidents
- Sync works correctly across 3+ devices

---

## 2. Detailed Phase Breakdown

### Phase 0: Foundation (Week 1-2)

#### Deliverables

| Deliverable                 | Owner  | Status | Validation                 |
| --------------------------- | ------ | ------ | -------------------------- |
| Prototype timer logic (CLI) | Camilo | ✅      | Counts 12 hours accurately |
| JSONL schema finalized      | Camilo | ✅      | Documented, validated      |
| GitHub Actions setup        | Camilo | ⏳      | Deferred to v0.2           |
| Menu bar UI skeleton        | Camilo | ✅      | Icon appears, works        |

#### Learning Focus

**Rust Concepts:**
- `tokio::spawn()` for timer loop
- `channels` for event passing
- `File::create()`, `OpenOptions` for I/O

**Tauri Concepts:**
- Command macros (`#[tauri::command]`)
- App state management
- IPC serialization

**Specific Milestones:**
- Day 1-2: Timer loop works (no UI)
  - Write timer in pure Rust: increment every second
  - Validate: runs 12 hours without drift

- Day 3-4: IPC from JavaScript
  - JavaScript calls `get_timer_state()`
  - Rust returns `{elapsed_seconds: 1200}`

- Day 5-6: File I/O
  - Save check-in to JSONL
  - Verify file contains valid JSON

- Day 7-10: Menu bar icon
  - Configure `tauri.conf.json` system tray
  - Test icon appears in menu bar
  - Test tray menu items (Show, Quit)

---

### Phase 1: Alpha (Week 3-6)

#### Deliverables

| Deliverable                                          | Owner  | Status | Validation                     |
| ---------------------------------------------------- | ------ | ------ | ------------------------------ |
| Working session flow (goal → timer → check-in → log) | Camilo | ✅      | All implemented, working       |
| Settings UI + persistence                            | Camilo | ✅      | Fully functional               |
| Session history (basic list)                         | Camilo | 🚧      | Spec 001 in progress           |
| Test suite (unit + integration)                      | Camilo | ⏳      | Deferred to v0.2               |
| Signed macOS release                                 | Camilo | ⏳      | Deferred to v0.2               |

#### Learning Focus

**Systems Programming:**
- Race conditions: file writes during concurrent check-ins
- Debugging: use `dbg!()`, `log` crate for insight
- Error handling: graceful failures (not crashes)

**DevOps:**
- GitHub Actions: build, test, sign, release
- Semantic versioning: v0.1.0 → v0.1.1
- Release notes: what changed?

**Performance:**
- Profiling: flamegraph for startup time
- Memory: `ps` command to check idle usage
- Battery: run for 1 hour, measure CPU %

**Specific Milestones:**

**Week 3-4: UI + State**
- [ ] Timer display updates every 1 second
- [ ] Check-in button triggers modal
- [ ] User selects status (On Task / Distracted / Break)
- [ ] Data written to JSONL

**Week 5: Settings Panel**
- [ ] Settings modal opens from menu
- [ ] User adjusts session_duration, check_in_interval
- [ ] Changes persist across restarts
- [ ] Verify IPC round-trip latency <10ms

**Week 6: Testing + Release**
- [ ] Unit tests for timer logic
- [ ] Integration test: start session → log check-in → read JSONL
- [ ] Build passes CI (cargo check, cargo test, cargo clippy)
- [ ] Code-sign macOS build
- [ ] Release .dmg to GitHub

---

### Phase 2: Beta (Week 7-10)

#### Deliverables

| Deliverable                   | Owner  | Status | Validation                           |
| ----------------------------- | ------ | ------ | ------------------------------------ |
| Windows port (feature parity) | Camilo | ⏳      | Works on Windows 10+                 |
| Session history UI + stats    | Camilo | ⏳      | Display today/week/month summaries   |
| Data export (CSV)             | Camilo | ⏳      | Export, open in Excel, no corruption |
| Performance optimized         | Camilo | ⏳      | <50MB idle, <5ms log append          |
| CI/CD auto-signs releases     | Camilo | ⏳      | GitHub release auto-populated        |

#### Learning Focus

**Cross-Platform Development:**
- Windows API differences (system tray, file paths)
- Testing both OS simultaneously (Docker simulation)
- Debugging platform-specific issues

**Performance Engineering:**
- Profiling: where is CPU time going?
- Memory: heap profiling, allocation patterns
- Benchmarking: establish baseline latencies

**Distributed Systems (Prep for v1.0):**
- Data versioning: how to migrate JSONL schema in future?
- Conflict detection: what if sync fails midway?
- Idempotency: is logging the same check-in twice safe?

**Specific Milestones:**

**Week 7: Windows Port**
- [ ] Install Rust + Tauri on Windows VM
- [ ] Port menu bar → system tray (Windows equivalent)
- [ ] Test file I/O paths: `C:\Users\{user}\AppData\Roaming\`
- [ ] Build .exe + .msi installer
- [ ] Test on real Windows 10+ machine

**Week 8: Analytics**
- [ ] Read JSONL, aggregate by day
- [ ] Display: "Today: 68% On Task, 22% Distracted"
- [ ] Show session timeline (visual bar chart)
- [ ] Verify aggregation is correct (manual spot-check)

**Week 9: Performance Sprint**
- [ ] Profile with flamegraph: find hotspots
- [ ] Measure startup time: target <500ms
- [ ] Measure check-in latency: target <5ms
- [ ] Monitor memory: Idle <50MB

**Week 10: Release Infrastructure**
- [ ] GitHub Actions signs Windows + macOS
- [ ] Automated release notes generation
- [ ] Test update flow: download, install, verify signature

---

### Phase 3: Release (Week 11-14)

#### Deliverables

| Deliverable                              | Owner  | Status | Validation                           |
| ---------------------------------------- | ------ | ------ | ------------------------------------ |
| Personal server sync (protocol + client) | Camilo | ⏳      | Sync data to HTTP server             |
| Encryption at rest (AES-256)             | Camilo | ⏳      | No plaintext in files                |
| Monitoring + error recovery              | Camilo | ⏳      | App self-heals from crashes          |
| Production deployment                    | Camilo | ⏳      | v1.0 available for download          |
| Documentation + user guides              | Camilo | ⏳      | Users can troubleshoot independently |

#### Learning Focus

**Distributed Systems:**
- Sync protocol design: conflict resolution, eventual consistency
- CRDT fundamentals: design for offline-first
- Testing distributed scenarios: network failures, clock skew

**DevOps Maturity:**
- Monitoring: what metrics matter?
- Error recovery: automatic retry, exponential backoff
- Incident response: if users report data loss, what's the playbook?

**Security:**
- Encryption: choose algorithm, key derivation
- Key management: where to store password?
- Threat model: who are we protecting against?

**Specific Milestones:**

**Week 11: Sync Foundation**
- [ ] Design HTTP sync endpoint: `POST /api/v1/checkins`
- [ ] Build client: batch check-ins, exponential backoff on failure
- [ ] Test: simulate network failures, verify recovery
- [ ] Design conflict resolution: Last-Write-Wins (LWW) strategy

**Week 12: Encryption**
- [ ] Generate user encryption key (from password)
- [ ] Encrypt check-ins before sync
- [ ] Decrypt on read
- [ ] Test: encrypted data unreadable without key

**Week 13: Monitoring + Hardening**
- [ ] Add telemetry: app crashes, sync failures
- [ ] Error recovery: if sync fails 3 times, pause for 1 hour
- [ ] Automatic cleanup: delete old backups
- [ ] Test edge cases: out of disk space, corrupted JSONL, clock drift

**Week 14: Release + Docs**
- [ ] Final security audit (code review with peer)
- [ ] Write user guide: install, set up, privacy settings
- [ ] Write troubleshooting: "Why aren't my logs syncing?"
- [ ] Tag v1.0.0, build signed binaries, release

---

## 3. Technical Dependencies & Blockers

### External Dependencies (Must Resolve Before v0.1)

| Dependency       | Status      | Risk | Mitigation                   |
| ---------------- | ----------- | ---- | ---------------------------- |
| Tauri CLI v1.8.3 | ✅ Available | Low  | Already in use, stable       |
| Rust 1.70+       | ✅ Available | Low  | Standard toolchain           |
| macOS 10.13+ SDK | ✅ Available | Low  | Old enough, widely supported |
| Windows 10+ SDK  | ✅ Available | Low  | Standard Windows dev         |

### Internal Dependencies (Must Build)

| Dependency           | Phase   | Blocker? | Plan                           |
| -------------------- | ------- | -------- | ------------------------------ |
| Timer logic          | Phase 0 | YES      | Build first, test separately   |
| IPC bridge           | Phase 0 | YES      | Tauri handles; just use macros |
| File I/O (JSONL)     | Phase 0 | YES      | Use `std::fs`, `serde_json`    |
| Menu bar UI          | Phase 0 | NO       | Prototype with simple HTML     |
| Platform abstraction | Phase 2 | YES      | Design once in Phase 1         |
| Sync protocol        | Phase 3 | NO       | Design doc, implement Phase 3  |

### Skills Gaps (Must Learn)

| Skill                    | Current Level | Target | Timeline   |
| ------------------------ | ------------- | ------ | ---------- |
| Rust ownership/borrowing | ⭐⭐⭐           | ⭐⭐⭐⭐   | Week 1-4   |
| Tauri IPC                | ⭐⭐            | ⭐⭐⭐⭐   | Week 1-3   |
| Performance profiling    | ⭐⭐            | ⭐⭐⭐⭐   | Week 7-9   |
| Distributed systems      | ⭐⭐⭐           | ⭐⭐⭐⭐⭐  | Week 11-14 |
| DevOps/CI-CD             | ⭐⭐            | ⭐⭐⭐⭐   | Week 1-6   |

### Known Risks

| Risk                                    | Probability | Impact   | Mitigation                                |
| --------------------------------------- | ----------- | -------- | ----------------------------------------- |
| **Cross-platform bugs**                 | 40%         | HIGH     | Test on both OS weekly                    |
| **Data corruption** (concurrent writes) | 20%         | CRITICAL | Atomic writes, file locking tests         |
| **Performance regression**              | 30%         | MEDIUM   | Benchmark every commit                    |
| **Scope creep**                         | 60%         | MEDIUM   | Cut features ruthlessly                   |
| **User testing reveals UX flaws**       | 70%         | MEDIUM   | Iterate with feedback, not major refactor |

---

## 4. Validation Checkpoints

### v0.1 Validation (Week 6)

**Criteria:**
- [ ] Build passes `cargo check` (no warnings)
- [ ] All tests pass: `cargo test`
- [ ] Clippy: `cargo clippy -- -D warnings` (zero warnings)
- [ ] Startup time <500ms (measured with `time`)
- [ ] Idle memory <50MB (measured with `ps`)
- [ ] Data persists across app restart
- [ ] 3 beta testers report "no crashes"
- [ ] **Manual validation**: Researcher uses app for 2 hours, logs 8 check-ins, exports JSONL, verifies all entries present

**Failure Criteria:**
- ❌ Any data loss
- ❌ Crash during normal usage
- ❌ Memory > 100MB idle
- ❌ Compiler warnings or clippy warnings

---

### v0.2 Validation (Week 10)

**Criteria:**
- [ ] Windows + macOS builds both green
- [ ] Feature parity: same features on both OS
- [ ] Session history loads 30 sessions in <200ms
- [ ] Export to CSV opens in Excel without corruption
- [ ] Performance: log append <5ms, session load <200ms
- [ ] 10+ beta testers, 80% daily active after 1 week
- [ ] CI/CD pipeline auto-signs and releases

**Failure Criteria:**
- ❌ Data loss on Windows
- ❌ >100ms check-in latency
- ❌ Tester churn >20% (people stop using)

---

### v1.0 Validation (Week 14)

**Criteria:**
- [ ] Sync works: desktop → server → desktop (data round-trip)
- [ ] Conflict resolution works: edit on 2 devices, correct winner emerges
- [ ] Encryption works: settings unreadable without password
- [ ] 99.9% uptime: measure for 1 week (calculated from logs)
- [ ] Error recovery: app recovers from sync failures autonomously
- [ ] Code-signed: no security warnings on macOS/Windows
- [ ] 100+ active users, zero unplanned data loss incidents
- [ ] Documentation complete: user guide + troubleshooting

**Failure Criteria:**
- ❌ Silent data loss (worst-case scenario)
- ❌ Sync corruption (two versions conflict unresolved)
- ❌ <99.9% uptime
- ❌ Users unable to recover from failures

---

## 5. Learning Milestones (Aligned with Ship Dates)

### Checkpoint: End of Week 2 (Foundation Complete)

**Skills Acquired:**
- Rust timer loops, file I/O patterns
- Tauri command system
- Basic IPC understanding

**Proof:**
- Timer code compiles, runs 12 hours without drift
- Data saves to JSONL, readable

**Self-Assessment:**
- [ ] I understand ownership & borrowing deeply
- [ ] I can write Rust without looking at docs for basic ops
- [ ] I understand IPC serialization

---

### Checkpoint: End of Week 6 (v0.1 Shipped)

**Skills Acquired:**
- Full Tauri IPC pipeline
- UI/backend integration
- File system error handling
- Basic testing strategies
- GitHub Actions CI

**Proof:**
- App works end-to-end
- Tests pass (70%+ coverage)
- Build is automated

**Self-Assessment:**
- [ ] I can debug async issues
- [ ] I understand race conditions in file I/O
- [ ] I can write tests for concurrency

---

### Checkpoint: End of Week 10 (v0.2 Shipped)

**Skills Acquired:**
- Performance profiling (flamegraph, perf)
- Cross-platform debugging
- Advanced testing (integration, edge cases)
- Release automation

**Proof:**
- Performance targets met (<50MB, <5ms logging)
- Windows build equivalent to macOS
- Automated releases

**Self-Assessment:**
- [ ] I can optimize code based on profiles
- [ ] I understand OS-level differences (macOS vs Windows)
- [ ] I can design test strategies for concurrency

---

### Checkpoint: End of Week 14 (v1.0 Shipped)

**Skills Acquired:**
- Distributed systems design (sync protocol)
- Conflict resolution strategies (CRDTs, LWW)
- Encryption & security practices
- DevOps maturity (monitoring, recovery)
- Production hardening

**Proof:**
- Sync works correctly across devices
- Encryption is unbreakable
- App recovers from failures
- Monitoring in place
- Documentation complete

**Self-Assessment:**
- [ ] I understand CAP theorem and its implications
- [ ] I can design a sync protocol from scratch
- [ ] I know how to monitor and debug production systems
- [ ] I've shipped software that doesn't lose user data

---

## 6. Engineering Progress Table

### Macro View (Quarters)

| Phase        | Deliverable                | Learning Focus               | Owner  | Start | End | Status |
| ------------ | -------------------------- | ---------------------------- | ------ | ----- | --- | ------ |
| v0.1 Alpha   | Timer + Menu Bar           | Tauri IPC, Rust fundamentals | Camilo | W1    | W6  | 🚧      |
| v0.2 Beta    | Multi-platform + Analytics | Performance, cross-platform  | Camilo | W7    | W10 | ⏳      |
| v1.0 Release | Sync + Encryption          | Distributed systems, DevOps  | Camilo | W11   | W14 | ⏳      |

### Micro View (Weekly Sprints)

#### Week 1

| Day     | Deliverable          | Learning                 | Validation                 |
| ------- | -------------------- | ------------------------ | -------------------------- |
| Mon-Tue | Timer loop (Rust)    | Ownership, loops         | Counts 12 hours accurately |
| Wed-Thu | JSONL logging        | File I/O, error handling | Writes valid JSON          |
| Fri     | GitHub Actions setup | CI basics                | Build passes on commit     |

#### Week 2

| Day     | Deliverable            | Learning                      | Validation                     |
| ------- | ---------------------- | ----------------------------- | ------------------------------ |
| Mon-Tue | IPC bridge (Rust ↔ JS) | Command macros, serialization | JS gets timer state            |
| Wed-Thu | Menu bar skeleton      | Tauri system tray             | Icon appears in menu           |
| Fri     | Integration test       | Test strategy                 | Test passes: goal → log → read |

#### Week 3-4

| Day     | Deliverable             | Learning             | Validation                      |
| ------- | ----------------------- | -------------------- | ------------------------------- |
| Ongoing | UI refinement           | Frontend performance | <100ms latency for button click |
| Ongoing | Settings persistence    | IPC round-trip       | Settings survive restart        |
| Ongoing | Session history (basic) | JSONL querying       | Load 30 entries, display        |

*(Detailed sprints for W5-14 follow same pattern: Daily deliverables, learning goal, validation method)*

---

## 7. Rollback & Risk Management

### If We Discover Data Corruption (Panic Button)

**Immediate Actions:**
1. Pause app deployment
2. Investigate root cause (file locking? concurrent writes?)
3. Identify affected users
4. Provide recovery tool (restore from backup)

**Longterm:**
- Switch to database (SQLite) if JSONL proves unreliable
- Add checksums to JSONL entries
- Implement transaction log

---

### If Performance Targets Aren't Met

**If Startup > 500ms:**
- Profile with flamegraph
- Lazy-load settings (don't read all logs on start)
- Cache computed results

**If Idle > 50MB:**
- Profile heap
- Find large allocations
- Use `Arc<Mutex<>>` to avoid cloning

**If Log Append > 5ms:**
- Batch writes (buffer 10 check-ins, write once)
- Measure syscall time (might be OS limitation)

---

### If Windows Port Fails

**Fallback:** Delay Windows to v0.3, ship v0.2 macOS-only, collect feedback

---

## 8. Success Measures (Final Scorecard)

### By Ship Date (W14)

**Quantitative:**
- ✅ 0 data loss incidents
- ✅ 99.9% uptime measured over 1 week
- ✅ <50MB idle memory
- ✅ <5ms log append latency
- ✅ 100+ active users

**Qualitative:**
- ✅ Users say "This respects my privacy" (survey)
- ✅ Users say "I understand my productivity better" (interviews)
- ✅ Code is maintainable (peer review approval)
- ✅ Documentation is complete (no support friction)

### For Camilo (Personal Mastery)

**Technical Mastery Checklist:**
- ✅ Can write async Rust without docs
- ✅ Can profile code and identify bottlenecks
- ✅ Can design sync protocol from scratch
- ✅ Can set up CI/CD pipeline independently
- ✅ Understand distributed systems trade-offs (CAP, CRDTs, consensus)
- ✅ Shipped production software that doesn't lose user data

**Mindset:**
- ✅ Think like a systems engineer (understand *why*, not just *how*)
- ✅ Embrace trade-offs (no perfect solution)
- ✅ Measure before optimizing (avoid premature optimization)
- ✅ Learn from failures (debug carefully, document decisions)

---

## Next Steps (Starting Now)

### Immediate (This Week)

- [ ] Review MVP.md + DOCUMENT_OF_TRUTH.md thoroughly
- [ ] Set up dev environment (Rust, Tauri, macOS SDK)
- [ ] Write timer loop prototype (no UI, just Rust)
- [ ] Validate: timer runs 12 hours, accurate to ±1 second

### Short-term (Next 2 Weeks)

- [ ] Get timer + file I/O working
- [ ] Connect to menu bar (UI skeleton)
- [ ] 3 beta testers start using
- [ ] First GitHub release (v0.1.0-alpha)

### Medium-term (Weeks 3-10)

- [ ] Iterate on feedback
- [ ] Ship v0.2 (multi-platform)
- [ ] Performance optimization sprint

### Long-term (Weeks 11-14)

- [ ] Implement sync + encryption
- [ ] Production hardening
- [ ] Ship v1.0

---

## Final Reminders

**Ship > Perfection**
You don't need to predict every edge case. Build v0.1, test with users, iterate.

**Learn as You Build**
Each phase teaches you something new. Tauri → Performance → Distributed Systems. This is intentional.

**Measure Everything**
Don't guess about performance. Profile. Don't assume users understand your UI. Test with real people.

**Own Your Decisions**
Every trade-off is deliberate (IPC vs shared memory, JSONL vs SQLite, CRDTs vs LWW). Document why.

---

**By v1.0, you will have shipped a production-grade app AND mastered the engineering skills that separate junior engineers from senior architects.**

Let's build something remarkable. 🚀
