# 💡 Copilot & AI Contributor Instructions

## Reading Order
1. **Start with `AGENTS.md`.** It defines the immutable tech stack (Tauri + vanilla JS) and workflow rules.
2. **Locate the relevant spec in `/specs`.** Specs are numbered (`001-session-review-panel.md`, etc.)—never implement beyond what they authorize.
3. **Only after understanding the spec** should you open the referenced files in `src/` or `src-tauri/src/`.

## Implementation Guardrails
- Use existing patterns from `src-tauri/src/main.rs` when adding new commands: `#[tauri::command]`, `Result<T, String>`, and error mapping via `map_err(|e| e.to_string())`.
- Keep frontend logic modular. New behaviour goes into a dedicated module (e.g., `src/js/sessionReview.js`) and is imported with `<script type="module">`.
- Persist user data only through the helpers defined in Rust; do not invent alternate storage locations or network sync.
- Respect the JSONL schema already written by `log_check_in`. Specs will mention when schema changes are allowed.

## ✅ Prefer These Examples
- `src/index.html:1` — Shows the production UI structure and CSS conventions for the glassmorphic shell.
- `src-tauri/src/main.rs:1` — Illustrates how to structure commands, settings persistence, and tray interactions.
- `docs/DOCUMENT_OF_TRUTH.md` — Provides historical decisions and naming context when specs reference past work.

## ❌ Avoid These Anti-Patterns
- `src/test.html:1` — Experimental file; no drag regions, blocking alerts, and ad-hoc scripting. Do not copy anything from here into production surfaces.
- `docs/ROADMAP.md` — Directional only, not a spec; never treat roadmap bullets as implementation orders.
- Any new third-party dependency not mentioned in `AGENTS.md` or an explicit spec.

## Workflow Notes
- When touching Rust + frontend in the same change, update the relevant spec’s “Implementation Notes” section once the feature ships.
- If you cannot find a spec for the task, stop and request one—do not improvise features.
- After coding, run `pnpm run dev` locally and perform the manual sanity loop (start timer → trigger check-in → open settings → quit). Report failures in your PR description.
