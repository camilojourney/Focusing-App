# Spec 007: Hyper Awareness Recovery

_Constitution: AGENTS.md@v1_

## 1. Feature Objective

Preserve a local active focus session across an app restart or long sleep interruption, and keep JSONL check-in history append-safe when a final record was interrupted.

## 2. File Locations

- `src-tauri/src/session_state.rs` - versioned active-session state, atomic save, restart reconciliation, and synthetic-data regression tests.
- `src-tauri/src/logs.rs` - validated JSONL append boundary, metadata-only diagnostics, and malformed-tail regression test.
- `src-tauri/src/main.rs` - trusted Tauri commands for state persistence, recovery, and diagnostics.
- `src/main.js` - session snapshot, explicit restart recovery UI, and periodic state persistence.
- `src/settings.html` - configurable 20-minute check-ins and 20-second response reminders as defaults.

## 3. Business and Technical Logic

1. The active session is stored only in the application configuration directory as `active_session.json` with `version: 1`.
2. State records the phase, goal, timer remainders, session settings, check-in counters, and focus-shield state. It is atomically replaced only after serialization and version validation succeed; on Unix, the containing directory is synced after rename so the replacement is durable.
3. On startup, an `active` or `writing` state is reconciled to `interrupted`, retained without resetting counters or remainders, and shown as an explicit resume decision. A previously user-paused state remains paused.
4. Reset is the only user action that clears durable active-session state.
5. Check-ins are parsed and normalized by Rust before append. The writer examines the final byte and writes a newline boundary before a new record when an interrupted trailing record lacks one. It never rewrites or deletes historical bytes.
6. The diagnostics command reports only valid-record count, malformed-record count, and unterminated-tail status. A record counts as valid only when both its JSON shape and RFC3339 timestamp parse. It does not return activity text.
7. Existing JSONL records remain compatible. Invalid historical records remain in place and are counted, while valid records before and after them remain readable. A future state schema version requires an explicit migration instead of a silent reset.
8. The app remains local-first. The Review panel reads the journal only when the user opens it; any JSONL export is a separate manual action outside the app. It adds no network transfer, telemetry, automatic external analysis, or storage migration.

## 4. Data Contracts and UI States

### Active session state

```json
{
  "version": 1,
  "phase": "interrupted",
  "sessionGoal": "...",
  "sessionTimeRemaining": 42000,
  "checkInTimeRemaining": 1200,
  "checkInsCompleted": 3
}
```

`phase` is one of `active`, `writing`, `paused`, or `interrupted`. Only `active` and `writing` become `interrupted` at process restart.

### Diagnostics response

```json
{
  "validRecords": 12,
  "malformedRecords": 1,
  "hasUnterminatedTail": false
}
```

The UI presents this metadata without exposing journal content.

## 5. Acceptance and Evidence Mapping

| Acceptance criterion | Evidence-backed implementation and test |
| --- | --- |
| Interrupted runs are recovered accurately or explicitly represented | The established sleep-gap path calls `pauseSession`; saved state is reconciled to `interrupted` after restart. `restart_recovery_marks_active_session_interrupted_without_resetting_it` verifies preserved synthetic state. |
| Activity history survives a truncated final record | The writer restores only the newline boundary, then appends a normalized record. `append_after_unterminated_tail_keeps_prior_valid_records_and_new_record` verifies valid synthetic records on both sides remain readable. |
| Cadence and reminder defaults are configurable | Existing settings controls remain user-configurable. Defaults are 20 minutes and 20 seconds in Rust and settings UI; saved user settings are not overwritten. `tests/timer-state.test.mjs` proves an active timer retains its remainders while a user configuration update takes effect. |
| Timer transitions retain correct remaining time | `tests/timer-state.test.mjs` proves pause capture rounds remaining time up and resume derives fresh deadlines from those preserved values. |
| History remains exportable and local-first | JSONL remains the append-only journal. Review opens it only after a user action, and any export is manual outside the app; no network code is added. |
| Recoverable issues are visible without exposing activity content | `get_persistence_diagnostics` returns counts and tail state only; `diagnostics_count_invalid_timestamps_as_malformed_records` proves invalid timestamps are reported as malformed without exposing content. |
| Practical installation and accurate purpose | README documents frozen installs, manual bundle update, local-data preservation, and no health or performance guarantee. |
| Focused checks | Run `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `pnpm install --frozen-lockfile`, `pnpm run test:timer`, and `pnpm run build`; conduct an isolated-profile lifecycle check when buildable. |
