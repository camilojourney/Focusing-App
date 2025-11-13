# ⚙️ Spec 001: Session Review Panel

_Constitution: AGENTS.md@2025-11-07_

## 1. Feature Objective
Give users a lightweight way to review the current focus session without leaving the main window. The review panel will list all check-ins captured since the timer started (status, note, timestamp) and surface a short summary so users can realign quickly.

## 2. File & Module Targets
- `src/index.html`
  - Add the “Review” trigger button/icon and the review drawer markup.
  - Embed the frontend script that fetches log data and renders cards.
- `src/js/sessionReview.js` (new)
  - Holds DOM helpers, serialization, and invoke calls for this feature only.
- `src-tauri/src/logs.rs` (new)
  - Encapsulates read-only access to the JSONL log file (`focus_log.jsonl`).
- `src-tauri/src/main.rs`
  - Register a new `#[tauri::command] fn list_session_entries(...)`.
  - Wire the module into `tauri::Builder::invoke_handler`.

## 3. Business & Technical Logic
1. **Access scope**
   - The review summarizes only the active session. A “session” starts when the timer transitions from idle → running and resets when the user clicks “Reset Session.”
   - Frontend stores the session `startTime` in memory and in `localStorage.currentSession.startTime` so the drawer survives window focus loss.
2. **Command: `list_session_entries`**
   - Parameters: `{ start_time_iso: String }`.
   - Reads `focus_log.jsonl`, deserializes each line into `LogEntry` (`serde::Deserialize`).
   - Filters entries where `entry.timestamp >= start_time_iso`.
   - Returns `Vec<SessionEntry>` sorted ascending by timestamp.
   - On failure (file missing or malformed line) return `Err("Unable to read session log: <detail>")`.
3. **Rust module (`logs.rs`)**
   - Owns `pub fn read_since(app: &AppHandle, start: DateTime<Utc>) -> Result<Vec<SessionEntry>, String>`.
   - Reuse existing `log_file_path` helper via a public function exported from `main.rs` or move it into `logs.rs`.
   - Use `BufRead::lines` to avoid loading the entire file into memory.
   - Parse timestamps with `chrono`; reject lines missing required fields.
4. **Frontend behavior**
   - “Review” chip toggles a drawer that slides from the bottom of `.app-shell`. Drawer width matches shell (max 360px). No separate window.
   - On open, call `invoke("list_session_entries", { startTimeIso })`.
   - Render each entry as a row: status (emoji + label), note (if any), and human-readable time (e.g., `09:45 AM`). Keep list virtualized via simple `DocumentFragment` (no new library).
   - Show summary pill at top: `On Task vs. Distraction count` + `Breaks`.
   - If no data, display `"No check-ins yet. Stay mindful ✨"`.
5. **State updates**
   - When `log_check_in` succeeds (already invoked elsewhere), emit a `window.dispatchEvent(new CustomEvent("ft:checkin-created"))`.
   - The review module listens for that event and refreshes the drawer if it is open.
6. **Settings interactions**
   - No new persistent settings. Respect existing styling and keep controls keyboard accessible (`tabindex=0`, `aria-expanded`).

## 4. Data Contracts
### LogEntry (existing JSONL line)
```json
{
  "timestamp": "2025-11-07T15:32:10.123Z",
  "status": "on_task",
  "note": "Worked on thesis chapter",
  "goal": "Finish chapter 4"
}
```

### SessionEntry (command response)
```json
{
  "timestamp": "2025-11-07T15:32:10.123Z",
  "status": "on_task",
  "statusLabel": "✅ On Task",
  "note": "Worked on thesis chapter"
}
```
- `statusLabel` is derived in Rust; frontend should not stringify status names.

## 5. Acceptance Checklist
- Review button appears in the main timer UI and is reachable via keyboard.
- Opening the drawer while logs exist shows at least the last three check-ins with correct status labels and notes.
- Removing `focus_log.jsonl` gracefully surfaces the empty state (no crashes, an error toast appears).
- Check-ins recorded after the drawer is open appear automatically within 1 second.
- Closing the drawer does not stop or pause the running session timer.
