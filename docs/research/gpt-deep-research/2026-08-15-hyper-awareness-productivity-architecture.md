---
source: "ChatGPT Deep Research"
captured_at: "2026-08-15T23:00:05.572105+00:00"
evidence_status: "Unreviewed source report. Validate claims before adding them to canonical vault notes."
---

# Hyper Awareness: Evidence-Backed Architecture Decision for the Next Release

## Executive recommendation

**Decision:** specify the next release around a **small Rust-owned session/reminder state machine backed by transactional SQLite, while keeping reflection local and deterministic and preserving JSON/JSONL as a migration/export format**. This is best described as **B, narrowed by C and E**:

- **B:** explicit durable session/reminder state transitions and recovery semantics;
- **C:** SQLite as the established local persistence component, plus Tauri/native notifications strictly as a delivery mechanism;
- **E:** deterministic local review plus explicit Markdown/JSON/copy export;
- **not D:** no direct cloud-AI integration in this release.

This should **not** become a textbook event-sourcing system. At Hyper Awareness's single-user scale, making every screen rebuild itself by replaying an immutable event stream would add complexity without evidence of product value. Instead, write each consequential state transition and its small audit event in **one SQLite transaction**, keep materialized session/reminder rows as the normal recovery/read model, and deliberately keep private note text out of the append-only event ledger so deletion remains meaningful. SQLite provides atomic transactions and established crash-recovery mechanisms; it also gives the app uniqueness constraints and application-controlled schema versioning that would otherwise have to be rebuilt around JSONL. citeturn16view8turn16view10

**Do not describe “20 minutes / 20 seconds” as an evidence-proven optimum.** The evidence is mixed. A 2022 meta-analysis found small beneficial effects of microbreaks on vigor and fatigue but no statistically significant overall performance improvement; benefits to performance were concentrated in less cognitively demanding work. One 2023 prospective 20-20-20 study found reduced digital-eye-strain and dry-eye symptoms over two weeks, whereas a separate 2023 controlled experiment found no benefit of 20-second scheduled breaks for symptoms, reading speed, or accuracy. A 2025 experiment again suggested frequent or individualized breaks can reduce some eye-strain symptoms. The defensible product position is therefore: **20 minutes and 20 seconds are convenient starting defaults that users can freely change or disable, not a health prescription or productivity guarantee.** citeturn16view0turn19search0turn20search0turn20search2

The same caution applies to active self-report check-ins. Ecological momentary assessment research establishes that repeated in-the-moment self-report is feasible and can achieve fairly high compliance, but it does **not** establish that asking knowledge workers “what are you working on?” every 20 minutes improves focus or productivity. Meta-analytic EMA designs commonly use roughly five to six assessments per day rather than a fixed 20-minute workday cadence, and burden/disruption is an acknowledged design consideration. Hyper Awareness should therefore treat the check-in as a **user-chosen awareness instrument**, not as an intervention whose effectiveness is asserted by the product. citeturn21search0turn21search10turn21search11

**Overall confidence: high-moderate, approximately 0.82.** Confidence is **high, approximately 0.88**, that an explicit state machine plus transactional local database is safer for restart/recovery and future history evolution than adding increasingly elaborate recovery semantics to JSONL. Confidence is only **moderate, approximately 0.62**, that 20/20 is the right default cadence for a broad user base because the eye-break literature is contradictory and there is no direct evidence that a 20-minute focus self-report cadence is optimal.

The architectural recommendation is intentionally bounded. It does **not** imply that the currently observed “stuck session” is caused by JSONL. That root cause requires local reproduction. Replacing persistence before reproducing the defect could add migration risk while failing to fix the actual bug.

### The strongest case for the status quo

Option A is substantially stronger than it might first appear. JSONL is human-inspectable, easy to back up and export, has few moving parts, and already matches Hyper Awareness's local-only philosophy. If local fault testing shows that records are not being corrupted or duplicated and the stuck-session defect is simply an in-memory/UI state bug, a targeted fix plus explicit startup recovery may be the safest next release. Rebuilding persistence solely because SQLite is “more robust” would violate the project's simplicity criterion if the additional robustness is not actually needed.

**The recommendation should reverse from B-lite to A** if all of the following are demonstrated locally: repeated forced termination never leaves ambiguous history except a safely discardable/truncatable final JSONL record; session reconstruction is deterministic; duplicate reminders can be prevented with a small identifier/idempotency addition; schema evolution requirements remain modest; and the planned reflection UI does not need reliable relational querying.

Conversely, evidence for moving even more strongly toward B would be: repeated ambiguous recovery states, cross-record updates that cannot safely be made atomic in the existing format, increasingly complex deduplication/replay code, multiple history schema migrations, or review features whose JSONL reconstruction logic becomes a material maintenance burden.

The report therefore authorizes **a specification and reversible validation prototype only**, not a broad refactor, new network behavior, or health/productivity claims.

## Evidence on breaks, check-ins, and interruption

### What microbreak research supports

Albulescu et al.'s 2022 systematic review and meta-analysis is the strongest broad evidence located for ordinary microbreaks. Across the included studies, microbreaks produced statistically significant but small improvements in vigor and fatigue, while the overall performance effect was not statistically significant. Performance improvements appeared in less cognitively demanding tasks, and longer breaks were associated with greater performance benefits. The study defined microbreaks as breaks under ten minutes, so it does not specifically validate twenty seconds every twenty minutes. citeturn16view0

That distinction matters for Hyper Awareness. The research supports a modest statement such as **“brief breaks can help some aspects of subjective well-being or fatigue”**. It does not support **“a 20-second interruption every 20 minutes makes you more productive.”** For intensely cognitive work—software development, writing, analysis, design—the meta-analysis is particularly poor evidence for a productivity claim because the performance effect was weakest where cognitive demands were higher. citeturn16view0

The updated 2025 Cochrane review of work-break interventions is even more cautious for musculoskeletal outcomes. It found nine studies in the updated review, judged the evidence very uncertain, and found insufficient good-quality evidence for reliable conclusions about benefits and harms of alternative work-break frequencies. Higher-frequency breaks may make little or no difference to musculoskeletal discomfort, although the certainty is very low. No eligible trials compared different break durations. citeturn19search2

**Product implication:** do not merge several weakly supported ideas—eye rest, musculoskeletal recovery, self-awareness, and productivity—into one claim. They have different evidence bases.

### What the 20-20-20 evidence actually says

The direct 20-20-20 literature is contradictory rather than uniformly positive.

Talens-Estarelles and colleagues reported in 2023 that using reminders for the 20-20-20 rule reduced digital-eye-strain and dry-eye symptoms over a two-week intervention, although two weeks was insufficient to produce substantial changes in several objective binocular-vision or dry-eye measures. This supports the possibility that the practice can improve **subjective symptoms** for some screen users. citeturn19search0turn20search4

Johnson and Rosenfield's 2023 controlled study reached a different conclusion. Thirty young participants completed a demanding 40-minute tablet-reading task with 20-second breaks every 5, 10, 20, or 40 minutes. Symptoms increased after the task in all conditions, but scheduled-break frequency had no significant effect on symptoms, reading speed, or accuracy. The authors explicitly concluded that their results did not support 20-second scheduled breaks as a therapeutic intervention for digital eye strain. citeturn20search0turn20search5

A 2025 experiment by Redondo and colleagues compared four break schedules during a 40-minute reading task and reported benefits for individualized and/or more frequent breaks on some digital-eye-strain outcomes. That result strengthens the general case for letting users take suitable breaks but weakens the case for treating exactly 20 minutes and exactly 20 seconds as uniquely correct. citeturn20search2

The appropriate synthesis is therefore:

| Proposition | Evidence status | Product consequence |
|---|---|---|
| Brief breaks may reduce fatigue or increase vigor | **Supported, modest effect** | Reasonable product rationale, without promising performance gains. citeturn16view0 |
| Looking away periodically may reduce some subjective digital-eye-strain symptoms | **Plausible / partially supported** | A look-away reminder is defensible as an optional wellness prompt. citeturn19search0turn20search2 |
| Exactly 20 seconds every 20 minutes is optimal | **Not supported** | Treat 20/20 as an editable default, never a scientific optimum. citeturn20search0turn20search5 |
| 20/20 improves knowledge-work productivity | **Unsupported** | Do not make this product claim. citeturn16view0turn20search0 |
| More frequent work breaks reliably prevent musculoskeletal problems | **Very uncertain** | Do not market the reminder as an ergonomic treatment. citeturn19search2 |

### Active check-ins are measurement, not proven treatment

The closest established research paradigm to Hyper Awareness's “what are you working on right now?” prompt is experience sampling/ecological momentary assessment. A large meta-analysis reported an average design of about six assessments per day over seven days, with average compliance around 79%; another systematic review/meta-analysis estimated compliance around 82% and found substantial protocol heterogeneity. These studies support momentary self-report as a workable data-collection technique, but they do not demonstrate that being repeatedly queried itself makes participants work better. citeturn21search0turn21search11

That distinction should appear in the product specification. Hyper Awareness can truthfully say that check-ins create a contemporaneous personal record rather than relying only on retrospective memory. It should **not** infer that more prompts are better or silently increase cadence to maximize “awareness.” EMA literature explicitly treats prompt intensity and questionnaire length as participant-burden variables, and newer work continues to investigate the relationship between prompt timing/frequency and compliance. citeturn21search2turn21search6turn21search10

A 20-minute workday cadence can produce roughly 18–24 prompts over six to eight hours of active use, far above the median prompt frequency in much EMA research. That numerical comparison does not prove that Hyper Awareness is too frequent—the prompt here may consist of only one short field—but it is sufficient reason to make skip, defer, quiet modes, per-session disablement, and custom cadence first-class rather than secondary settings. The calculation is an engineering/product inference from the proposed cadence and the EMA evidence. citeturn21search0turn21search11

### Interruption is a real countervailing cost

Reminders are not free. Stothart, Mitchum, and Yehnert found that merely receiving a phone notification significantly disrupted performance on an attention-demanding task even when participants did not interact with the device. The experiment was about phone notifications rather than desktop productivity prompts, so its exact effect size should not be transferred to Hyper Awareness, but the causal direction is directly relevant: an alert intended to improve focus can itself consume attention. citeturn22search0

Microsoft's current Windows notification design guidance independently reaches the product-design version of the same conclusion: notifications should be valuable rather than noisy, excessive interruption frustrates users and leads them to disable the channel, and quiet/suppressed delivery is an appropriate option. citeturn16view12turn23view5

This creates the key behavioral design principle for the release:

> **The reminder should offer an opportunity to notice, not demand proof of compliance.**

A check-in that steals focus, forces a modal, starts an unavoidable countdown, or penalizes skipping can negate the benefit it is meant to create. The application should optimize for **easy acknowledgment, easy postponement, and easy silence**, rather than maximizing completion rate.

## Architecture and ranked options

### Ranked comparison

The ranking below treats the user's criteria as ordered priorities rather than turning them into an arbitrary weighted sum. In particular, delivery speed does not outrank recovery correctness or privacy.

| Rank | Option | Restart / suspend / clock correctness | Privacy / control | Simplicity / debugging | Interruption / accessibility | Maintenance / cross-platform / migration | Decision |
|---|---|---|---|---|---|---|---|
| **First** | **B — explicit local state machine, narrowed to transactional SQLite plus small event ledger** | **Very high** once tested: transitions and projections can commit atomically; restart recovery is explicit | **Very high**; all data remains local | Higher initial complexity than A, but simpler invariants than custom JSONL recovery | Can model defer/skip/expire explicitly | Strong fit; SQLite is embedded and mature | **Recommend** |
| **Second** | **A — current timer + JSONL with targeted repairs** | Potentially high enough, but depends heavily on local failure results and custom recovery logic | **Very high** | **Highest** short-term simplicity | No inherent UX penalty | Best immediate reversibility; increasing schema/state complexity is the risk | **Best fallback / strongest challenger** |
| **Third** | **C — established components without explicit state semantics** | Medium: a database or notification library cannot define missed-prompt policy for the app | High | High at component level | Depends on product logic | Good as a building block, incomplete as architecture | **Adopt inside B, not alone** |
| **Fourth** | **E — deterministic local summaries/export alone** | Low as a crash-recovery architecture | **Highest** | High | High | Excellent as reflection layer | **Include as feature boundary, not persistence solution** |
| **Fifth** | **D — direct cloud AI review** | Does not solve local session correctness | **Lowest** of the options | Adds network, provider, error, consent, security and policy states | Adds another user decision surface | Higher long-term dependency and migration risk | **Reject for this release** |

The reason B wins is not “databases are always better.” It wins because the upcoming requirements create **cross-record invariants**: one active logical session, one logical reminder per ordinal, at most one stored check-in per reminder, durable recovery state, schema evolution, and deletion semantics. SQLite can enforce several of these properties transactionally rather than requiring the application to infer them from several append files after every abnormal exit. SQLite describes its commits as atomic—either all changes in a transaction occur or none do—and exposes an application-controlled `user_version` field useful for schema migration. citeturn16view8turn16view10

### Use SQLite, but do not over-engineer SQLite

For this workload there is no compelling reason to begin with WAL mode. SQLite's WAL documentation lists real advantages—better reader/writer concurrency, sequential I/O, and fewer `fsync` operations—but also additional WAL/checkpoint behavior and companion-file considerations. Hyper Awareness writes very small records at low frequency and is not a multi-user database server. The simpler default should therefore be ordinary transactional SQLite first, with WAL treated as a performance option only if local measurements identify meaningful read/write contention. This is an engineering inference from SQLite's documented trade-offs, not a claim that WAL is unsafe. citeturn16view9turn23view3

Likewise, do not replace JSON files with Tauri's key-value Store plugin and call the durability problem solved. The Store plugin is explicitly a persistent file-backed key/value store that can save/load across restarts, which makes it reasonable for uncomplicated preferences. It does not by itself supply the relational uniqueness and multi-entity transaction semantics useful for sessions, reminders, check-ins, and migrations. citeturn17view6

The Tauri SQL plugin supports SQLite through `sqlx` and is a legitimate component, but its current public API is oriented toward frontend SQL access. As of August 15, 2026, the plugin repository still has an open issue requesting/supporting proper transaction handling across multiple frontend operations. For the correctness-critical state machine, the safer architectural boundary is therefore **Rust owns the transaction**, regardless of which Rust SQLite binding is ultimately selected. JavaScript should request semantic operations such as `resolve_checkin`, not orchestrate `BEGIN`/multiple statements/`COMMIT` itself. That conclusion is engineering inference from the current plugin interface and open issue rather than evidence of a SQLite defect. citeturn16view6turn16view7turn23view1

### Timer and notification architecture

The scheduler should have **one source of truth: the Rust state machine**. Native notifications are delivery attempts, not state.

Rust's `Instant` is designed around an OS monotonic clock on supported tier-one platforms, whereas `SystemTime` is explicitly non-monotonic and can move backward. That makes the appropriate split straightforward: use an `Instant`-based deadline while the process is alive; persist UTC wall timestamps for human history/audit; and after a restart or detected discontinuity, reconcile logical state instead of pretending a persisted wall timestamp is a monotonic timer. citeturn17view1turn17view2

Do **not** assume that monotonic-clock behavior over laptop sleep is identical on every platform. Microsoft's own timer APIs distinguish clocks that include sleep/hibernation from one that counts only working-state time, illustrating why sleep accounting is a semantic choice rather than something a cross-platform application should infer accidentally from whichever low-level clock happens to back a runtime. Hyper Awareness should have explicit wake/restart reconciliation tests rather than relying on undocumented assumptions about a timer continuing “correctly” while a laptop lid is closed. citeturn17view12

If Tokio interval timers are involved, its default missed-tick behavior is particularly relevant: `Burst` fires missed ticks rapidly until caught up. A focus application should **never** let that default translate into three or four overdue check-ins appearing after wake. Use one-shot next-deadline scheduling or explicitly choose skip semantics. Tokio's test utilities can advance simulated time, making this policy testable without twenty-minute wall-clock test runs. citeturn17view3turn17view4

Native notification APIs should be treated as best-effort channels. Tauri's notification plugin exposes native notifications and requires permission checks. Apple explicitly says notification interactions may be disruptive, permission must be obtained, authorization can change later, and current settings should be checked before scheduling. Windows guidance similarly allows notification suppression, and Windows currently does not support App SDK notifications from elevated apps. Thus an OS notification API succeeding is not equivalent to “the user saw the reminder.” citeturn17view0turn18view1turn23view6

**Recommended delivery hierarchy:**

1. Maintain reminder eligibility and state in Rust.
2. When due, attempt the configured channel: native notification, quiet notification, tray/menu-bar change, or foreground check-in surface.
3. Record a **delivery attempt**, not an assumed delivery.
4. Never advance important durable state solely because the OS API accepted a notification request.
5. When the user actually answers, skips, defers, or opens the check-in, record that explicit action.
6. On resume/restart, collapse all stale prompts into one recovery decision rather than replaying a notification backlog.

A single-instance guard should also be part of the desktop reliability boundary so two launches cannot independently schedule the same logical reminder. Tauri provides a Single Instance plugin for desktop operation; its Linux behavior uses DBus and has packaging-specific requirements under Snap/Flatpak, so those packages require corresponding tests if Hyper Awareness ships them. citeturn17view5turn23view2

## State machine, data contract, and failure recovery

### Recommended state machine

This is an **engineering specification inferred from the evidence above**, not a claim that Tauri or SQLite requires these exact states.

```text
SESSION

IDLE
  └─ start ───────────────> RUNNING

RUNNING
  ├─ pause ───────────────> PAUSED
  ├─ normal end ──────────> ENDED
  ├─ app/wake gap that makes state ambiguous
  │                        > INTERRUPTED
  └─ reminder becomes due > RUNNING + REMINDER:DUE

PAUSED
  ├─ resume ──────────────> RUNNING
  └─ end ─────────────────> ENDED

INTERRUPTED
  ├─ user chooses resume ─> RUNNING
  └─ user chooses end ────> ENDED


REMINDER

SCHEDULED
  └─ deadline reached ────> DUE

DUE
  ├─ delivery attempt ────> DUE          [attempt is not receipt]
  ├─ user opens/responds ─> PRESENTED
  ├─ user skips ──────────> SKIPPED
  ├─ user defers ─────────> DEFERRED
  └─ becomes stale ───────> EXPIRED

DEFERRED
  └─ deferred deadline ───> DUE

PRESENTED
  ├─ check-in saved ──────> ANSWERED
  ├─ skip ────────────────> SKIPPED
  └─ defer ───────────────> DEFERRED
```

**No transition exists from “app was asleep for an hour” to “fire every missed check-in.”** That is intentional. The interruption literature makes burst reminders behaviorally undesirable, and Tokio's documented missed-tick behavior demonstrates that catch-up behavior must be deliberately overridden rather than left implicit. citeturn22search0turn17view3

For an unclean exit, the app should not silently decide that the user remained focused during the entire offline period. On next launch, an unfinished prior session becomes **interrupted**. The user can resume from now or end the old session. If resumed, the active check-in cadence begins afresh or from a clearly specified recovery policy; it should not manufacture check-ins that were never presented. Windows explicitly tells desktop applications that they need their own persistence mechanism for crash/unexpected-termination recovery, and Modern Standby can suspend user-mode processes despite the ordinary desktop lifecycle otherwise continuing until close. citeturn16view11

### Minimal local data contract

The database should remain deliberately small. User preferences can initially stay in the existing configuration mechanism; the session snapshots the relevant values so later setting changes cannot rewrite history.

| Entity | Minimal fields | Important invariant |
|---|---|---|
| `app_run` | `run_id`, `started_utc`, `clean_exit_utc?`, `app_version` | Previous row without a clean exit is evidence of abnormal termination, not proof of its cause. |
| `session` | `session_id`, `status`, `started_utc`, `ended_utc?`, `checkin_interval_ms`, `break_duration_ms`, `write_window_ms`, `data_policy`, `schema_version` | A running session's policy is a snapshot, not retrospectively changed by preferences. |
| `reminder` | `reminder_id`, `session_id`, `ordinal`, `due_utc`, `state`, `resolved_utc?`, `defer_count` | Unique `(session_id, ordinal)` prevents a logical reminder being created twice. |
| `delivery_attempt` | `attempt_id`, `reminder_id`, `channel`, `attempted_utc`, `result_class` | Attempt ≠ user receipt. |
| `checkin` | `reminder_id`, `captured_utc`, `note_text?`, optional user labels | At most one current check-in record per reminder unless the UX explicitly supports editing/versioning. |
| `event` | `sequence`, `event_id`, `entity_id`, `event_type`, `occurred_utc`, `minimal_payload`, `schema_version` | Append-only **metadata** ledger; do not copy private note prose into it. |

The event row and corresponding current-state mutation should occur in the **same transaction**. For example, answering a reminder should atomically set the reminder to `ANSWERED`, save/update its `checkin`, and append a `checkin_recorded` event. A crash should therefore expose either the old state or the new state, rather than an event claiming success while the check-in row is missing. That is exactly the kind of all-or-none update SQLite transactions are designed to provide. citeturn16view8

This is intentionally **not pure event sourcing**. Normal startup reads the current session/reminder tables; replay exists as a diagnostic/export capability, not as the only way to reconstruct application state. That sacrifices some theoretical purity for simpler debugging and recovery.

The privacy consequence is important: an immutable event stream must not become an accidental permanent archive of deleted notes. An event may say `checkin_recorded` with a reminder ID and timestamp; it should not contain the user's actual note unless the product explicitly adopts an immutable-note retention policy—which this report does not recommend.

SQLite's application-controlled `user_version` field provides an established place for schema-version coordination. Its integrity-check facilities can be used on a diagnostic/recovery path if corruption is suspected, but a full integrity scan does not need to become an every-launch ritual for such a small local app unless local evidence justifies it. citeturn16view10

### Failure and recovery matrix

| Failure or edge condition | Required behavior | Why |
|---|---|---|
| **Process killed mid-session** | On next start, detect previous unclean `app_run`; put prior running session into `INTERRUPTED`; offer Resume or End. | Do not infer activity during an interval the application could not observe. Windows says desktop apps must implement their own recovery state after unexpected termination. citeturn16view11 |
| **Crash during a check-in write** | Event, reminder state, and check-in either all commit or none commit. | SQLite atomic transaction boundary. citeturn16view8 |
| **Laptop sleeps for less than one cadence** | On wake, reconcile current reminder once; no duplicate notification. | Sleep can suspend application execution; clock/timer behavior cannot be assumed uniformly. citeturn16view11turn17view12 |
| **Laptop sleeps for several cadences** | Expire/collapse missed reminders; never fire catch-up bursts; resume with at most one relevant interaction. | Tokio's default interval catch-up is Burst unless changed; multiple alerts create avoidable interruption. citeturn17view3turn22search0 |
| **Wall clock moves forward/back** | Live deadline remains monotonic; persisted UTC remains audit metadata. If discontinuity makes history ambiguous, record a clock-discontinuity/recovery event rather than manufacturing elapsed time. | `SystemTime` is explicitly non-monotonic; `Instant` is intended for monotonic interval measurement. citeturn17view1turn17view2 |
| **Timezone / DST changes** | UTC ordering remains unchanged; only local display changes. Test formatting and review grouping separately. | Engineering inference from use of UTC persistence. |
| **Notification permission denied or later disabled** | Continue state machine; fall back to tray/in-app channel; surface permission state without nagging. | Apple says authorization can change and should be checked; Tauri requires permission handling. citeturn18view1turn17view0 |
| **Windows Focus / quiet delivery** | Respect OS suppression; do not bypass it with a forced foreground modal. | Windows explicitly recommends suppression to reduce interruption. citeturn16view12 |
| **Windows app elevated** | Treat native notification as unavailable; use another configured channel. | Current Windows App SDK notification documentation says elevated apps cannot send/receive app notifications. citeturn23view6 |
| **Two app instances launch** | Only one scheduler owns the active session. | Tauri Single Instance exists for this purpose; package behavior must be verified on supported targets. citeturn17view5 |
| **Disk full / permission failure** | Do not claim “saved”; keep user-visible error distinct from reminder resolution; retry must be idempotent. | Engineering requirement derived from transactional correctness. |
| **Database corruption suspected** | Preserve original file; stop destructive writes; run diagnostic integrity checks on a copy or recovery path; offer deterministic export/salvage where possible. | SQLite provides integrity checking; recovery should not destroy the only evidence. citeturn16view10 |
| **Migration interrupted** | Legacy source remains unchanged; SQLite migration transaction rolls back or remains resumable; never delete originals as part of first migration. | SQLite transaction semantics plus reversibility requirement. citeturn16view8 |
| **Partial final JSONL legacy record** | Import all unambiguous complete records; report the malformed tail; leave source file untouched. | Local migration policy; avoids silently rewriting historical evidence. |
| **User deletes a note** | Erase note text and derived deterministic/AI artifacts that contain it; event ledger retains at most a tombstone/reference. | Required to make deletion meaningful rather than cosmetic. |
| **Old OS notification remains after reminder resolution** | Remove or expire it where platform APIs permit; logical state is already resolved regardless. | Windows supports tag/group removal and expiration for stale notifications. citeturn17view14turn23view7 |

macOS can expose sleep/wake notifications, and Windows exposes suspend/resume power events; these can improve prompt reconciliation, but neither should be the only correctness mechanism. Apple's sleep/wake Q&A is an older archived document—last updated in 2014—so it should be treated as platform background rather than a current cross-platform abstraction contract. Generic lateness/reconciliation on the next observable event is still required. citeturn17view10turn17view11

## Privacy-safe reflection and accessible interaction

### The AI boundary for this release

The recommended reflection path is:

```text
local sessions/check-ins
        |
        v
deterministic local summary
        |
        +--> local review UI
        |
        +--> explicit Copy
        |
        +--> explicit Export: Markdown / JSON
                     |
                     v
          user decides what happens next
```

A deterministic review can provide useful structure without pretending to “understand” the user: sessions completed, elapsed periods, check-in timeline, skipped/deferred counts, user-entered labels, and the user's own notes in chronological form. These are transformations of locally stored data rather than inferred judgments about motivation, mental state, productivity, or intent.

That boundary fits NIST's general privacy-risk-management principle of designing products so privacy risks are identified and managed, and it is consistent with data-minimization/purpose-limitation principles found in frameworks such as the GDPR. The GDPR citation here is a design benchmark, not a determination that a particular deployment of Hyper Awareness is legally subject to EU law. citeturn16view14turn11search3

**Direct cloud AI review should be rejected in the next release** because it solves no requirement that cannot first be validated with local summaries/export, while introducing an entirely new set of consequential states: consent, payload selection, provider identity, credentials, retention policy, provider errors, deletion expectations, network security, model provenance, and changing third-party terms. NIST's AI Risk Management Framework and its Generative AI profile exist precisely because generative-AI use introduces risks requiring explicit governance rather than being merely another UI component. citeturn11search5turn11search9

A later direct-provider feature should clear all of these gates before specification:

| Gate | Required future behavior |
|---|---|
| **Per-use initiation** | No automatic, periodic, background, launch-time, or shutdown-time upload. |
| **Payload preview** | Show the exact selected date range/sessions and categories being sent before transmission. |
| **Minimal scope** | Default to selected sessions, not “entire history.” Calendar context, titles, URLs, or other context must be separately selectable. |
| **Provider identity** | Name the provider and model/service actually receiving the information. |
| **Current policy review** | Review then-current provider retention/training/deletion terms at implementation time; do not freeze 2026 assumptions into the architecture. |
| **Credentials** | Never place provider credentials in history/export/log records. |
| **Provenance** | Store locally which source sessions supported a reflection, request time, provider/model identifier when available, and whether the user edited the output. |
| **Deletion** | Deleting source content must make local derived copies discoverable for deletion; remote deletion limits must be disclosed truthfully. |
| **No coercion** | Local review/export remains usable without connecting an AI provider. |

A useful provenance record does **not** need to retain a second complete copy of every note. A local manifest can identify source session/check-in IDs, selected fields, request timestamp, provider/model metadata, and a local payload digest. This allows the app to answer “what was this reflection based on?” without turning its audit trail into another private-note archive.

Direct cloud integration should be rejected outright for a proposed provider or configuration when the app cannot show what is sent, cannot narrow the payload, cannot state the provider's relevant data-use terms, cannot respect deletion expectations, requires ongoing background transfer, conflicts with the user's local-only policy, or provides no clear benefit over explicit export. That is a product/privacy inference, not a statement about any unnamed provider's current policy.

SQLite does **not** itself mean “encrypted.” Moving plaintext JSON notes into an ordinary SQLite database improves transactional integrity, not confidentiality against someone who can read the user's files. At-rest encryption is a separate threat-model decision. The next release should avoid implying otherwise and should validate file locations and permissions on each supported OS. If the local threat model later requires application-level encryption, that should be a separate architecture decision with key recovery and data-loss consequences, not quietly bundled into this migration.

### Interaction model

The default setup should say something equivalent to:

> **Check in every 20 minutes**  
> A starting point. Change this anytime.
>
> **Look away for 20 seconds at each check-in**  
> Optional. Brief visual breaks may help some people with screen discomfort, but this timing is not a medical rule.

That wording reflects the mixed vision evidence rather than converting a mnemonic into a claim of clinical efficacy. citeturn19search0turn20search0turn20search2

Check-in and look-away behavior should be **separately configurable**, even when they share a default cadence. A user may value a visual break but dislike journaling, or may want focus notes without a fixed 20-second countdown. Separating the controls also makes future evidence changes reversible.

At minimum, the interaction should provide:

- **Answer now**, **Defer/Snooze**, **Skip**, and **Disable for this session** without moralized language.
- A blank check-in as a valid outcome; the app should not force users to manufacture text.
- Cadence presets plus a custom value and an off state.
- Independent control of sound/banner/tray/in-app delivery where technically possible.
- No forced foreground window by default.
- No catch-up queue after sleep or downtime.
- No streak penalty, “focus score,” red warning, or shame treatment for skipping unless a future evidence-backed feature explicitly justifies such mechanics.

These controls are not merely preference polish. WCAG 2.2's interruption criterion states that non-emergency interruptions should be postponable or suppressible, and its timing criteria favor user control over nonessential time limits. WCAG directly governs web content rather than every native OS surface, but a Tauri webview should use it as a concrete accessibility baseline. citeturn16view13

Keyboard operation should cover the entire in-app check-in without requiring a pointer: predictable Tab/Shift-Tab traversal, a visible focus indicator, an unambiguous primary action, and keyboard-accessible defer/skip. Status updates such as “check-in saved” should be conveyed without gratuitously moving keyboard or screen-reader focus. These patterns follow the broader accessibility aim of reducing unnecessary interruptions and retaining user control. citeturn16view13

For the 20-second look-away period specifically, do not require the user to watch an animated countdown—which defeats the instruction to look away. A user-selected subtle start cue, tray state, optional end cue, or silent completion indication is more coherent. The timer should be nonblocking: the user can continue work, dismiss it, or turn it off.

### Data-use controls

Because note content may be private, the release specification should distinguish at least three decisions instead of hiding them behind one “privacy” toggle:

**Prompting:** whether active check-ins occur.

**Local retention:** whether note text is stored, and whether session metadata may be kept even if note text is not.

**External disclosure:** currently none except an explicit user-triggered export/copy operation.

That lets a user choose, for example, “remind me but don't retain my written note,” without requiring them to disable the core awareness function. The `data_policy` snapshot on each session should record which retention rules were active, so future review/export logic does not guess.

Deletion should be available at the session and history level, and deterministic summaries should either be generated on demand or clearly linked to their source records so they can be regenerated after deletion. An append-only audit mechanism should never be used as a loophole to preserve deleted note prose.

## Reversible prototype, migration, and revisit triggers

### Smallest reversible prototype

The smallest useful validation is **not** “rewrite Hyper Awareness onto SQLite.” It is a testable slice that proves or disproves the architectural premise.

**Prototype boundary:**

1. Define a Rust transition engine for one active session and one reminder sequence, with clock and repository interfaces that can be replaced by test doubles.
2. Model `RUNNING`, `PAUSED`, `INTERRUPTED`, `ENDED` and reminder `SCHEDULED/DUE/DEFERRED/ANSWERED/SKIPPED/EXPIRED`.
3. Persist that test profile to a separate SQLite database using backend-owned transactions.
4. Record a minimal append-only metadata event alongside each durable state transition.
5. Implement restart detection with `app_run`.
6. Implement only one reminder-delivery path plus a simulated delivery path.
7. Use synthetic/test-profile data first. Do **not** migrate the owner's real history merely to validate the design.
8. Prove sleep/missed-tick/restart/idempotency behavior.
9. Only after those tests pass, prototype legacy JSON/JSONL import on copies of representative fixtures.

Tokio's test-time facilities allow timer advancement, so twenty-minute and multi-hour cases can be exercised deterministically rather than making the test suite wait in real time. citeturn17view4

This prototype is reversible because the existing production persistence path does not have to be removed, production history does not need to be altered, and the SQLite path can initially exist behind a developer/test configuration. That is preferable to dual-writing live JSONL and SQLite: dual write would create a new reconciliation problem—what happens when one write succeeds and the other fails—before the replacement has proved its value.

### Required local test matrix

| Dimension | Cases that must pass before broad refactor | Acceptance criterion |
|---|---|---|
| **Normal cadence** | 20m default; short synthetic intervals; custom intervals | Exactly one logical reminder per ordinal. |
| **Defer** | Defer once; repeated defer; defer across app foreground/background | Same logical reminder remains identifiable; no duplicate check-in. |
| **Skip** | Skip via app; skip via notification action if supported | Durable `SKIPPED`; stale OS notification removed where possible. |
| **App kill** | Kill before transaction; during write; immediately after commit; while prompt is open | Recovery yields old or committed new state, never half-state. |
| **Restart** | Restart before due; when overdue; several cadences overdue | No catch-up burst; unfinished session becomes explicit recovery state. |
| **Sleep/wake** | Sleep 1 min, 19 min, 21 min, 1 h, several hours | At most one relevant prompt after reconciliation. |
| **Clock forward** | +1 h, +24 h | No duplicate reminder; history records discontinuity if needed. |
| **Clock backward** | −1 h, DST-style local change | No refiring of an already resolved reminder. |
| **Timezone change** | Change timezone with active session | UTC history ordering remains stable; local display changes coherently. |
| **Notification permission** | Granted, denied initially, revoked later | State machine remains correct; alternative channel works or unavailability is visible. |
| **OS quiet/focus** | Focus/Do Not Disturb/suppressed delivery | App does not defeat user OS preference. |
| **Windows elevated mode** | Run elevated if supported | Notification absence does not lose reminder/session state. citeturn23view6 |
| **Multiple launch** | Launch second instance while session active | One scheduler only. citeturn17view5 |
| **Storage failure** | Read-only app-data dir; simulated disk-full/commit failure | UI never reports unsaved check-in as durable; retry is idempotent. |
| **Corruption** | Corrupt a disposable DB copy | App fails safely; original preserved; diagnostic path gives actionable status. |
| **Legacy import** | Valid JSONL, empty file, duplicate record, invalid final line, older schema | Deterministic import report; source unchanged. |
| **Migration interruption** | Terminate during migration | Re-run is safe; no partially declared successful migration. |
| **Configuration race** | Change cadence while reminder due/deferred | Existing logical reminder follows documented policy; next reminder uses new setting. |
| **Keyboard** | Full session/check-in flow without mouse | Every essential action reachable with stable focus. |
| **Assistive technology** | Current VoiceOver/Narrator on supported targets; Linux AT if shipping Linux | Prompt/status understandable without forced focus jumps. |
| **Scale/UI** | OS text scaling, zoom, reduced-motion/high-contrast equivalents | Check-in remains operable and controls remain visible. |

Windows power events explicitly distinguish suspend and automatic/user resume states, while current Windows lifecycle guidance notes Modern Standby can suspend user-mode processes. Those facts justify testing real hardware sleep in addition to synthetic clocks; simulation alone cannot establish platform behavior. citeturn17view11turn16view11

Test the same principle on each **currently supported shipping target**, rather than assuming Tauri erases OS differences. Tauri's notification and Store plugins are cross-platform abstractions, but permission models, notification suppression, process power behavior, and packaging are platform-specific. citeturn17view0turn17view6

### Compatibility and migration plan

The first SQLite-capable release should perform migration as **copy/import, not move/convert-in-place**.

Legacy JSON/JSONL remains untouched until the new database has been created, imported, validated, and opened successfully. An import manifest should record which source files were processed and the migration version. A malformed final line should appear in a migration report rather than being silently deleted from the old file.

A migration should be transactionally all-or-none at an appropriate unit—ideally an entire small history file if practical. `PRAGMA user_version` should identify the SQLite schema generation. citeturn16view8turn16view10

**Do not dual-write JSONL and SQLite indefinitely.** Long-lived dual-write looks reversible but makes correctness worse: the application must then handle SQLite success/JSONL failure, JSONL success/SQLite failure, divergent deletion, and two competing histories. Preserve reversibility through untouched originals plus explicit export, not through permanent duplicate state machines.

After migration:

```text
Legacy JSON / JSONL
    ├─ retained unchanged for migration safety
    └─ no longer mutated once SQLite is canonical

SQLite
    └─ canonical new local history

Explicit export
    ├─ Markdown for human review / copy
    └─ versioned JSON for portability
```

A downgrade after the user has created new SQLite-only sessions should **not** silently attempt to merge those records into the old application's JSONL format. The safer policy is to preserve old files for binary rollback and provide an explicit export for data portability. “Application downgrade” and “lossless automatic data-format downgrade” are different promises.

If local testing demonstrates that users or maintainers strongly depend on manually inspecting/editing JSONL, that is a genuine decision-changing result: portability may outweigh the database's convenience. In that case a strengthened Option A—versioned JSONL records, per-record identifiers, explicit tail-corruption handling, and a small atomic recovery snapshot—becomes more attractive.

### Revisit triggers

The architecture decision should be reopened when any of these occur:

| Trigger | Reconsider |
|---|---|
| Fault tests show JSONL already satisfies all required recovery invariants with substantially less code | **Move back toward A.** |
| Review UI develops measurable SQLite read/write contention | **Evaluate WAL**, rather than enabling it preemptively. citeturn16view9 |
| Product promises reminders while the app is completely terminated | **Evaluate native scheduled-notification APIs per platform**; the current process-owned scheduler is no longer sufficient. Apple supports locally scheduled notifications and Windows exposes app-notification facilities, but permission and platform semantics must be validated independently. citeturn17view7turn17view13 |
| Users consistently disable 20-minute prompts, defer them, or report interruption | **Change/default-test cadence**; do not defend 20 minutes as scientifically fixed. citeturn20search0turn22search0 |
| User research shows look-away prompts are valued but check-in questions are not, or vice versa | **Decouple their defaults further.** |
| A future provider-specific AI feature has demonstrated demand and a reviewed consent/retention/deletion design | **Reopen D**, provider by provider; do not create a generic background-AI channel. |
| Users require confidential local storage beyond normal OS account/file protection | **Open a separate encryption/key-management design**, not an ad hoc SQLite setting. |
| Multi-device sync or collaboration becomes a product requirement | **Reopen persistence architecture entirely**; this report assumes one user's local desktop history. |
| Linux packages such as Snap/Flatpak become supported/relevant | **Expand single-instance, DBus, notification, and filesystem-permission testing.** citeturn17view5 |

## Evidence register and packet

**Access date for every web source below: August 15, 2026.** “Official documentation” indicates a normative/current platform or library source; “systematic review/meta-analysis” is higher-level research synthesis; “controlled study” means direct experimental evidence; “engineering inference” indicates this report is combining source facts into an architecture recommendation rather than attributing that design directly to the source.

| Claim ID | Consequential claim and evidence class | Canonical source, publication/update date | Status, contradiction, local validation |
|---|---|---|---|
| **E-BREAK-01** | Microbreaks have small positive effects on vigor/fatigue; overall performance effect was nonsignificant. **Systematic review/meta-analysis.** | Albulescu et al., PLOS ONE, published **2022-08-31**. `https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0272460` citeturn16view0 | **Supported for general microbreaks, not 20/20 optimum.** Validate whether Hyper Awareness users perceive benefit/burden. |
| **E-EYE-01** | 20-20-20 reminders reduced subjective digital-eye-strain/dry-eye symptoms in one intervention. **Peer-reviewed prospective study.** | Talens-Estarelles et al., **2023**. `https://pubmed.ncbi.nlm.nih.gov/35963776/` citeturn19search0 | **Positive evidence**, contradicted in part by E-EYE-02. Do not make clinical claim from one small study. |
| **E-EYE-02** | A controlled 40-minute study found no significant benefit of 20-second scheduled breaks for symptoms, reading speed or accuracy. **Controlled study.** | Johnson & Rosenfield, published **2023-01-01**, DOI `10.1097/OPX.0000000000001971`. `https://pubmed.ncbi.nlm.nih.gov/36473088/` citeturn20search0turn20search5 | **Contradicts a strong 20/20 claim.** Supports configurable default. |
| **E-EYE-03** | 2025 evidence supports individualized/frequent breaks for some digital-eye-strain outcomes. **Controlled study.** | Redondo et al., **2025**. `https://pubmed.ncbi.nlm.nih.gov/40466853/` citeturn20search2 | **Supports breaks generally more than one fixed cadence.** |
| **E-WORK-01** | Evidence for work-break frequency preventing musculoskeletal symptoms is very uncertain. **Cochrane systematic review.** | Published **2025-10-08**. `https://www.cochrane.org/evidence/CD012886_work-break-interventions-preventing-musculoskeletal-symptoms-and-disorders-healthy-workers` citeturn19search2 | **Strong caution against ergonomic treatment claims.** |
| **E-CHECKIN-01** | EMA/momentary self-report is feasible, but protocols and burden vary; evidence does not establish a 20-minute productivity benefit. **Meta-analyses/systematic reviews.** | Wrzus et al., **2022/2023 publication record**; Williams et al., **2021**. `https://pmc.ncbi.nlm.nih.gov/articles/PMC9999286/`, `https://pmc.ncbi.nlm.nih.gov/articles/PMC7970161/` citeturn21search0turn21search10 | **Measurement feasibility supported; productivity effect unproven.** Measure skip/defer/disable locally. |
| **E-INTERRUPT-01** | Notifications can disrupt attention even without interaction. **Controlled experimental study.** | Stothart et al., **2015**. `https://pubmed.ncbi.nlm.nih.gov/26121498/` citeturn22search0 | **Relevant causal evidence but phone-to-desktop generalization is imperfect.** Validate interruption burden with actual Hyper Awareness UI. |
| **E-TIME-01** | Rust `Instant` is monotonic-oriented; `SystemTime` is not monotonic. **Official Rust documentation; rolling docs, page date not stated.** | `https://doc.rust-lang.org/std/time/struct.Instant.html`, `https://doc.rust-lang.org/std/time/struct.SystemTime.html` citeturn17view1turn17view2 | **Supported.** Sleep accounting across actual target hardware still needs tests. |
| **E-TIME-02** | Tokio interval's default missed-tick behavior is Burst; test time can be advanced. **Official crate documentation; rolling docs.** | `https://docs.rs/tokio/latest/tokio/time/enum.MissedTickBehavior.html`, `https://docs.rs/tokio/latest/tokio/time/fn.advance.html` citeturn17view3turn17view4 | **Supported.** Ensure implementation never inherits burst reminder behavior accidentally. |
| **E-STORE-01** | SQLite transactions provide atomic commit; `user_version` supports application schema versioning. **Official SQLite documentation.** | Atomic Commit page; PRAGMA page last updated **2026-06-04**. `https://www.sqlite.org/atomiccommit.html`, `https://sqlite.org/pragma.html` citeturn16view8turn23view4 | **Strong support for SQLite over custom cross-record JSONL transactions.** Must still fault-test chosen Rust integration/filesystem. |
| **E-STORE-02** | WAL improves concurrency/performance but adds WAL/checkpoint considerations. **Official SQLite documentation.** | Last updated **2026-04-13**. `https://www.sqlite.org/wal.html` citeturn16view9turn23view3 | **No evidence Hyper Awareness needs WAL.** Benchmark before adopting. |
| **E-TAURI-01** | Tauri has current native notification and single-instance components. **Official Tauri docs.** | Notification updated **2026-06-15**; Single Instance updated **2025-11-03**. `https://v2.tauri.app/plugin/notification/`, `https://v2.tauri.app/plugin/single-instance/` citeturn23view0turn23view2 | **Use as components, not logical source of truth.** Test packaging/permission behavior. |
| **E-TAURI-02** | Tauri SQL supports SQLite through `sqlx`; transaction-support issue remains open in plugin repository as observed **2026-08-15**. **Official docs + project issue.** | SQL docs updated **2025-11-04**. `https://v2.tauri.app/plugin/sql/`, `https://github.com/tauri-apps/plugins-workspace/issues/886` citeturn23view1turn16view7 | **Supports Rust-owned transaction boundary as an engineering inference.** Recheck issue/API at implementation time. |
| **E-WIN-01** | Modern Standby may suspend desktop processes; unexpected termination needs application state persistence. **Official Microsoft documentation.** | Current page, update date not exposed in retrieved content. `https://learn.microsoft.com/en-us/windows/apps/develop/launch/app-lifecycle` citeturn16view11 | **Supported.** Test on real supported Windows hardware. |
| **E-WIN-02** | Windows notification UX explicitly discourages noisy interruption; elevated apps currently cannot use App SDK notifications. **Official Microsoft documentation.** | UX guidance updated **2026-04-21**; overview updated **2026-07-15**. `https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-ux-guidance`, `https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/` citeturn23view5turn23view6 | **Supported.** Fallback channel required. |
| **E-APPLE-01** | Notification authorization is user-controlled, may change, and should be checked before scheduling. **Official Apple documentation.** | Current 2026 documentation; specific update date not stated. `https://developer.apple.com/documentation/usernotifications/asking-permission-to-use-notifications` citeturn18view1 | **Supported.** Test denied/revoked/quiet behavior on supported macOS releases. |
| **E-A11Y-01** | Non-emergency interruptions should be postponable/suppressible under WCAG 2.2 AAA guidance. **W3C Recommendation/accessibility standard.** | WCAG 2.2, **2023** Recommendation. `https://www.w3.org/WAI/WCAG22/quickref/` citeturn16view13 | **Directly applicable to webview UI; design principle for native notifications.** Test keyboard + screen reader locally. |
| **E-PRIV-01** | Privacy risk should be deliberately managed and data use minimized to the stated purpose. **NIST framework + regulatory design benchmark.** | NIST Privacy Framework current page; GDPR consolidated Article 5. `https://www.nist.gov/privacy-framework`, `https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=CELEX%3A02016R0679-20160504` citeturn16view14turn11search3 | **Supports local-first/minimal-transfer architecture.** Legal applicability is jurisdiction-specific and not determined here. |

The machine-readable packet below maps the principal decision claims to the same evidence. “Supported” does not mean a source directly prescribes the architecture; engineering-inference claims identify where the report has combined evidence with Hyper Awareness's stated constraints.

```yaml
research_id: HYPER-AWARENESS-RESEARCH-OPEN-7F3A
accessed: 2026-08-15
decision:
  recommendation: "B-lite + C(SQLite) + E; reject D for this release"
  confidence:
    overall: 0.82
    architecture: 0.88
    twenty_twenty_default: 0.62
  authorization: "research/specification only; no implementation or external AI authorized"

claims:
  - id: BREAK-DEFAULT
    claim: "20 min / 20 sec is defensible only as a configurable starting default, not an optimum or productivity/medical claim."
    evidence_status: mixed
    sources:
      - class: systematic_review_meta_analysis
        published: 2022-08-31
        url: "https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0272460"
      - class: peer_reviewed_intervention
        published: 2023
        url: "https://pubmed.ncbi.nlm.nih.gov/35963776/"
      - class: controlled_study
        published: 2023-01-01
        url: "https://pubmed.ncbi.nlm.nih.gov/36473088/"
      - class: controlled_study
        published: 2025
        url: "https://pubmed.ncbi.nlm.nih.gov/40466853/"
    contradiction: "Talens-Estarelles reports symptom improvement; Johnson/Rosenfield found no scheduled-break benefit in a short controlled task."
    local_check: "Measure skip/defer/disable rates and qualitative interruption burden; never infer health efficacy from usage."

  - id: CHECKIN-EFFECT
    claim: "Momentary self-report is a viable recording method, but evidence does not show that a 20-minute check-in cadence improves productivity."
    evidence_status: supported_for_measurement_not_intervention
    sources:
      - class: meta_analysis
        published: 2022
        url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9999286/"
      - class: systematic_review_meta_analysis
        published: 2021
        url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7970161/"
    contradiction: "High EMA compliance does not establish productivity benefit; protocol-frequency findings are heterogeneous."
    local_check: "Test several cadences and record only local aggregate prompt outcomes unless user explicitly opts into richer history."

  - id: INTERRUPTION-BURDEN
    claim: "Prompting itself can impose an attentional cost, so reminders require defer/skip/suppress controls."
    evidence_status: supported_with_domain_generalization
    sources:
      - class: controlled_experiment
        published: 2015
        url: "https://pubmed.ncbi.nlm.nih.gov/26121498/"
      - class: official_platform_guidance
        updated: 2026-04-21
        url: "https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-ux-guidance"
      - class: accessibility_standard
        published: 2023
        url: "https://www.w3.org/WAI/WCAG22/quickref/"
    contradiction: null
    local_check: "Keyboard/screen-reader test; no forced modal; verify quiet and session-disable behavior."

  - id: SCHEDULER
    claim: "Use a monotonic live deadline plus persisted wall-clock audit timestamps; reconcile after restart/wake and never burst missed prompts."
    evidence_status: engineering_inference_from_official_docs
    sources:
      - class: official_rust_docs
        updated: "rolling documentation"
        url: "https://doc.rust-lang.org/std/time/struct.Instant.html"
      - class: official_rust_docs
        updated: "rolling documentation"
        url: "https://doc.rust-lang.org/std/time/struct.SystemTime.html"
      - class: official_tokio_docs
        updated: "rolling documentation"
        url: "https://docs.rs/tokio/latest/tokio/time/enum.MissedTickBehavior.html"
      - class: official_windows_docs
        updated: "current at access"
        url: "https://learn.microsoft.com/en-us/windows/apps/develop/launch/app-lifecycle"
    contradiction: "Clock APIs differ in whether sleep is counted; therefore no universal sleep assumption is made."
    local_check: "Real sleep/wake + synthetic time tests on every supported desktop target."

  - id: PERSISTENCE
    claim: "Transactional SQLite with current-state tables plus a minimal append-only transition ledger is preferable to making JSONL the canonical recovery model."
    evidence_status: engineering_inference_strong
    sources:
      - class: official_sqlite_docs
        updated: "current at access"
        url: "https://www.sqlite.org/atomiccommit.html"
      - class: official_sqlite_docs
        updated: 2026-06-04
        url: "https://sqlite.org/pragma.html"
    contradiction: "JSONL remains simpler and more inspectable; recommendation reverses if fault testing proves its existing recovery invariants adequate."
    local_check: "Kill process around commit boundaries; disk-full; corruption copy; migration-interruption; idempotent retry."

  - id: SQLITE-MODE
    claim: "Do not enable WAL merely by default; begin with the simpler transactional configuration unless measurement demonstrates contention."
    evidence_status: engineering_inference
    sources:
      - class: official_sqlite_docs
        updated: 2026-04-13
        url: "https://www.sqlite.org/wal.html"
    contradiction: "WAL has documented performance/concurrency benefits."
    local_check: "Benchmark history review while check-ins are written; adopt WAL only for a demonstrated problem."

  - id: TAURI-BOUNDARY
    claim: "Keep correctness-critical DB transactions in Rust; use Tauri notification/single-instance components as adapters."
    evidence_status: engineering_inference
    sources:
      - class: official_tauri_docs
        updated: 2026-06-15
        url: "https://v2.tauri.app/plugin/notification/"
      - class: official_tauri_docs
        updated: 2025-11-03
        url: "https://v2.tauri.app/plugin/single-instance/"
      - class: official_tauri_docs
        updated: 2025-11-04
        url: "https://v2.tauri.app/plugin/sql/"
      - class: project_issue
        status_at_access: open
        url: "https://github.com/tauri-apps/plugins-workspace/issues/886"
    contradiction: "Tauri SQL is usable for SQLite access; the recommendation concerns where multi-step semantic transactions should be owned."
    local_check: "Recheck plugin transaction API/status at implementation time."

  - id: NOTIFICATION-TRUTH
    claim: "Notification delivery is best-effort and must not be the source of reminder state."
    evidence_status: engineering_inference_from_platform_constraints
    sources:
      - class: official_apple_docs
        updated: "current 2026 documentation"
        url: "https://developer.apple.com/documentation/usernotifications/asking-permission-to-use-notifications"
      - class: official_windows_docs
        updated: 2026-07-15
        url: "https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/"
      - class: official_windows_docs
        updated: 2026-04-21
        url: "https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/manage-app-notifications"
    contradiction: null
    local_check: "Denied/revoked permission, Focus/DND, elevated Windows, stale notification cleanup."

  - id: EVENT-PRIVACY
    claim: "Append-only events should not duplicate private note prose; mutable note storage is needed for meaningful deletion."
    evidence_status: privacy_engineering_inference
    sources:
      - class: privacy_framework
        updated: "current at access"
        url: "https://www.nist.gov/privacy-framework"
      - class: regulatory_design_benchmark
        url: "https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=CELEX%3A02016R0679-20160504"
    contradiction: "Pure immutable event sourcing offers stronger historical replay but conflicts with note-deletion minimization unless sensitive payloads are separately erasable."
    local_check: "Delete a note and verify no plaintext survives in events, summaries, exports, logs, or recovery artifacts."

  - id: AI-BOUNDARY
    claim: "Current release should use deterministic local summaries plus explicit export/copy and no direct provider integration."
    evidence_status: architecture_privacy_inference
    sources:
      - class: privacy_framework
        updated: "current at access"
        url: "https://www.nist.gov/privacy-framework"
      - class: ai_risk_framework
        updated: "current at access"
        url: "https://www.nist.gov/itl/ai-risk-management-framework"
    contradiction: "Cloud AI may later provide semantic reflection not achievable deterministically."
    local_check: "Before any provider is proposed: prove user demand, payload preview/minimization, consent, current retention/training policy review, deletion and provenance."

  - id: MIGRATION
    claim: "Import legacy JSON/JSONL without modifying originals; make SQLite canonical only after validated import; avoid permanent dual-write."
    evidence_status: engineering_inference
    sources:
      - class: official_sqlite_docs
        url: "https://www.sqlite.org/atomiccommit.html"
      - class: official_sqlite_docs
        updated: 2026-06-04
        url: "https://sqlite.org/pragma.html"
    contradiction: "Keeping JSONL canonical is safer if manual inspectability and existing recovery tests outweigh schema/transaction requirements."
    local_check: "Valid, duplicate, older-schema and malformed-tail fixtures; terminate every migration stage; verify re-run safety."

revisit:
  - "Local reproduction shows stuck-session failure is entirely UI/in-memory and JSONL remains unambiguous under fault tests -> prefer targeted Option A."
  - "Read/write contention is measured -> evaluate SQLite WAL."
  - "Product promises reminders while process is terminated -> evaluate per-OS scheduled notifications."
  - "Prompt defer/skip/disable rates show 20-minute default is burdensome -> change default without defending 20/20 as fixed science."
  - "Provider-specific AI demand and privacy requirements are proven -> reopen Option D under a separate decision."
  - "Local confidentiality threat model changes -> open a separate encryption/key-management decision."
research_close: HYPER-AWARENESS-RESEARCH-CLOSE-9C2D
```
