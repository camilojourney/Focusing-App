# 🧠 Project Constitution: Focus Time

This document captures the non-negotiable rules for the Focus Time menu bar application. Treat these guidelines as law—feature specs may extend them, but never contradict them.

## 0. Quick Start: Understanding the Project

### What This App Does
A macOS menu bar timer that helps you maintain focus through:
- **Timed sessions** (default 12 hours) with periodic check-ins (every 15 min)
- **Accountability prompts** where you write what you're doing
- **Session review** showing your focus patterns
- **Calendar integration** to pull current meeting as focus goal

### How It Works (30-Second Overview)

```
Menu Bar (🧠 15:00) → Click → Window Appears → [Start Session]
     ↓                                              ↓
Timer counts down                          Check-in every 15min
     ↓                                              ↓
Updates menu bar         →              "What are you doing now?"
     ↓                                              ↓
Logs to JSONL file      ←              Stores your response
     ↓
[Review] button → Shows session timeline with all check-ins
```

### Tech Stack at a Glance

- **Tauri 2** - Rust + JavaScript hybrid framework (like Electron, but faster)
- **Rust backend** - Handles system integration, file I/O, calendar access
- **Vanilla JS frontend** - Simple HTML/CSS/JS (no frameworks!)
- **macOS native APIs** - Direct integration with menu bar, calendar, etc.

### File Structure You Need to Know

```
src/
  ├── index.html          ← Main window (timer, buttons, logic)
  └── settings.html       ← Settings window

src-tauri/src/
  ├── main.rs            ← Tray icon, window management, Tauri commands
  ├── calendar.rs        ← macOS Calendar access (EventKit)
  └── logs.rs            ← JSONL logging and reading

specs/
  └── 006-menu-bar-integration.md  ← KEY SPEC: Read this to understand behavior
```

## 1. Mission & Persona
- Build a distraction-free macOS focus companion that feels premium, calm, and privacy-first.
- Keep tone confident and intentional. Favor clear copy over cutesy humor.

## 2. Immutable Tech Stack
- **Shell:** Tauri `2.x` targeting macOS menu bar apps (Rust 2021 + JavaScript frontend).
- **Frontend:** Vanilla HTML/CSS/JavaScript authored in `src/`. No React, Vue, bundlers, or CSS frameworks unless a spec explicitly allows an incremental library.
- **Backend:** Rust commands in `src-tauri/src/*.rs`, exposed via `tauri::command`. Use `Result<T, String>` for UI-safe error propagation.
- **Storage:** Local JSON or JSONL files inside Tauri's app config directory (see `src-tauri/src/main.rs:log_check_in`). Never write outside this sandbox or introduce network sync without executive approval.
- **Tooling:** `pnpm run dev` / `pnpm run build` (Tauri CLI) and `cargo` for Rust. Python 3.11+ scripts may live under project root for offline analysis only. Commit `pnpm-lock.yaml` and install with `pnpm install --frozen-lockfile` (see `.github/workflows/build.yml`). Generate that lockfile with pnpm 8 so it matches CI. Keep `@tauri-apps/api` on the same major.minor as the `tauri` crate in `src-tauri/Cargo.lock`; `scripts/check-tauri-versions.js` fails the build if they diverge.

### Tauri 2 Specific Requirements
- **Window Focus:** Use `ActivationPolicy::Regular` (not `Accessory`) for interactive windows. See `specs/006-menu-bar-integration.md` section 12 for why.
- **DevTools:** Requires `"withGlobalTauri": true` in `tauri.conf.json` to enable right-click inspect.
- **Menu Bar Apps:** Trade-off between `skipTaskbar` (no Dock icon) vs window focus (buttons work). Currently using `skipTaskbar: false` for functionality.
- **Event Handlers:** Use `get_webview_window()` instead of deprecated `get_window()` in Tauri 2.

## 3. Coding Guidelines
### Frontend (HTML/JS/CSS)
- Keep each window self-contained (`src/index.html`, `src/settings.html`). Inline `<script type="module">` blocks are acceptable; if code grows beyond ~150 lines, move logic to `src/js/<feature>.js` and import it.
- UI events must call Tauri commands through `@tauri-apps/api` helpers. Never bypass the trusted command set with `window.__TAURI__.invoke` strings generated at runtime.
- Style with modern CSS (flexbox, gradients) but avoid external fonts and heavyweight assets to keep bundle size minimal.
- Accessibility: ensure interactive controls have focus states and descriptive labels, even if the UI is minimalist.

### Rust (Tauri commands & services)
- Put shared helpers in modules (`src-tauri/src/calendar.rs`, future `storage.rs`, etc.) and expose clean functions.
- Every command must guard against I/O errors (`map_err(|e| e.to_string())?`) and log actionable error messages.
- Keep long-running tasks off the main thread; prefer `tauri::async_runtime::spawn` for blocking I/O when necessary.
- OS integrations (EventKit, AppleScript, desktop switching) must stay behind feature-gated helpers so the rest of the app can compile cross-platform for tests.

### Scripts & Data Analysis
- Python utilities (e.g., `analyze_focus_data.py`) should read from exported JSONL logs only. Never mutate production data or call third-party APIs.

## 4. Documentation & Specs
- **Specs live in `/specs`.** Use zero-padded IDs (`/specs/001-session-review.md`) and keep each file laser-focused on a single feature/change.
- Each spec must contain: Feature Objective, File Locations, Business/Technical Logic (numbered), Data Contracts or UI states, and Acceptance/Test Notes.
- Reference this constitution from every spec (`_Constitution: AGENTS.md@v1_`) so contributors know which version of the ruleset applied.
- Update `docs/` for narrative artifacts (roadmaps, lessons learned) but never bury requirements there—use specs.

## 5. Directory Responsibilities
- `src/` – UI windows surfaced by Tauri. `index.html` is the live session controller; `settings.html` drives preferences; keep any experiments quarantined (e.g., `test.html`) and mark them as deprecated inside specs.
- `src-tauri/src/` – Rust backend (`main.rs`, `calendar.rs`, future modules). Commands declared here must be registered in `tauri::Builder::invoke_handler`.
- `docs/` – Strategy docs (`MVP.md`, `ROADMAP.md`, `DOCUMENT_OF_TRUTH.md`). Treat as background context, not source-of-truth for implementation.
- `specs/` – Authoritative feature instructions (see section 4).
- `.github/` – Automation and AI guidance (Copilot instructions, workflows).

## 6. Quality Gates
- Manual sanity pass before committing: `pnpm run dev`, start a 2-minute session, trigger at least one check-in, open settings, and quit via tray menu.
- Run `cargo fmt && cargo clippy --no-deps` on Rust changes; fail the build if warnings surface.
- Keep JSONL/file I/O covered with unit tests when logic gets complex (e.g., parsing, aggregation). Use `#[cfg(test)]` modules in Rust files.
- Changes touching focus data format must include migration or compatibility notes in the relevant spec.

## 7. Forbidden Moves
- No background network syncing, telemetry, or analytics pings.
- Do not introduce stateful globals in the frontend—persist through settings files or session storage abstractions managed by the backend.
- Never ship experimental files (like `src/test.html`) as part of the production menu bar UI. Specs should label any throwaway artifacts so AI assistants ignore them.
- Avoid speculative dependencies. If a problem can be solved with the standard library (Rust) or platform APIs (Tauri), do so.

_Last updated: 2026-08-13. When the stack or rules change materially, bump this note and describe the delta in `docs/DOCUMENT_OF_TRUTH.md`._

## Maintaining this file

Keep this file for knowledge useful to almost every future agent session in this project.
Do not repeat what the codebase already shows; point to the authoritative file or command instead.
Prefer rewriting or pruning existing entries over appending new ones.
When updating this file, preserve this bar for all agents and keep entries concise.
