# ⚙️ Spec 003: Accountability Box

_Constitution: AGENTS.md@2025-11-07_

## 1. Feature Objective
Provide a structured reflection tool that prompts users to answer 5 deep questions about their work, fostering self-awareness and accountability. Responses are logged locally in JSONL format for future analysis and personal growth tracking.

## 2. File & Module Targets
- `src/index.html`
  - "Accountability Box" button in main UI
  - Modal with 5-question form
  - Form submission and validation
- `src-tauri/src/main.rs`
  - `#[tauri::command] fn log_accountability()` - Saves accountability responses to JSONL
  - Reuses existing `log_file_path()` helper for file location
- `~/Library/Application Support/com.focustime.app/accountability_box.jsonl`
  - Append-only JSONL file for accountability responses
  - One line per submission

## 3. Business & Technical Logic

### 3.1 The 5 Accountability Questions
These questions are designed to create deep self-reflection and track personal growth:

1. **"What did you accomplish today?"**
   - Purpose: Recognize achievements, build positive reinforcement
   - Field name: `q1_accomplished`
   - Type: Free-text (multi-line)

2. **"What challenges did you face?"**
   - Purpose: Identify obstacles, build problem-solving awareness
   - Field name: `q2_challenges`
   - Type: Free-text (multi-line)

3. **"What will you do tomorrow?"**
   - Purpose: Plan ahead, create intentionality
   - Field name: `q3_tomorrow`
   - Type: Free-text (multi-line)

4. **"How focused were you? (1-10)"**
   - Purpose: Quantify focus quality, track trends
   - Field name: `q4_focus_rating`
   - Type: Text input (validates 1-10 or "N/10" format)

5. **"What did you learn?"**
   - Purpose: Capture insights, build learning habit
   - Field name: `q5_learned`
   - Type: Free-text (multi-line)

### 3.2 User Interaction Flow
1. **Opening Accountability Box**
   - User clicks "Accountability Box" button in main UI
   - Modal opens with 5-question form
   - Form is empty (no pre-filled values)
   - Session continues running in background

2. **Filling Out Form**
   - User types responses in textareas
   - All fields optional (user can skip any question)
   - No character limits (encourage detailed reflection)
   - Focus rating validates format (1-10 or "5/10" style)

3. **Submitting Responses**
   - User clicks "Save Responses" button
   - Validation: At least one field must be filled (prevent empty submissions)
   - Data serialized to JSON with timestamp
   - IPC call to `log_accountability(json_data)`
   - Modal closes automatically after successful save
   - User sees brief confirmation message

4. **Canceling**
   - User can close modal without saving (X button or Escape key)
   - No data saved
   - Session continues normally

### 3.3 Data Persistence
1. **JSONL Format**
   - Each submission = 1 line in `accountability_box.jsonl`
   - Append-only (never modify existing lines)
   - Same pattern as `focus_log.jsonl`

2. **File Location**
   - macOS: `~/Library/Application Support/com.focustime.app/accountability_box.jsonl`
   - Windows: `%APPDATA%\com.focustime.app\accountability_box.jsonl`
   - Linux: `~/.config/focus-time/accountability_box.jsonl`

3. **Atomic Writes**
   - Use Rust `OpenOptions::new().create(true).append(true)`
   - Write entire JSON line + newline in one syscall
   - Prevents file corruption from concurrent writes

4. **Error Handling**
   - If file write fails, show error message to user
   - Don't lose user's typed responses (keep modal open)
   - Log error to console for debugging

## 4. Data Contracts

### AccountabilityEntry (JSONL line)
```json
{
  "timestamp": "2025-11-13T22:30:00.123Z",
  "q1_accomplished": "Completed the tray icon fix and implemented accountability feature",
  "q2_challenges": "Had to debug icon loading issues with Tauri",
  "q3_tomorrow": "Add data visualization for the accountability responses",
  "q4_focus_rating": "8/10 - stayed mostly on task with good flow",
  "q5_learned": "Learned how to create PNG files programmatically in Python"
}
```

**Field Specifications:**
- `timestamp`: ISO 8601 format, UTC timezone
- `q1_accomplished`: String (can be empty `""`)
- `q2_challenges`: String (can be empty)
- `q3_tomorrow`: String (can be empty)
- `q4_focus_rating`: String (can be empty, validates 1-10 or "N/10" format if filled)
- `q5_learned`: String (can be empty)

### Command: `log_accountability`
**Request:**
```javascript
await invoke('log_accountability', {
  logLine: JSON.stringify({
    timestamp: new Date().toISOString(),
    q1_accomplished: "...",
    q2_challenges: "...",
    q3_tomorrow: "...",
    q4_focus_rating: "8/10",
    q5_learned: "..."
  })
});
```

**Response (Success):**
```javascript
Ok(())  // void success
```

**Response (Error):**
```javascript
Err("Failed to write accountability log: <reason>")
```

## 5. UI/UX Design

### Modal Appearance
- **Size**: Medium modal (500px width, auto height)
- **Position**: Centered on screen
- **Background**: Semi-transparent overlay (blocks interaction with main UI)
- **Close**: X button top-right, Escape key closes
- **Styling**: Consistent with main app theme (calm, minimal)

### Form Layout
```
┌─────────────────────────────────────────┐
│  Accountability Box               [X]   │
├─────────────────────────────────────────┤
│                                         │
│  What did you accomplish today?         │
│  ┌─────────────────────────────────┐   │
│  │                                 │   │
│  │ [Multi-line textarea]           │   │
│  │                                 │   │
│  └─────────────────────────────────┘   │
│                                         │
│  What challenges did you face?          │
│  ┌─────────────────────────────────┐   │
│  │ [Multi-line textarea]           │   │
│  └─────────────────────────────────┘   │
│                                         │
│  [Same for remaining 3 questions]       │
│                                         │
│  ┌──────────┐  ┌──────────┐            │
│  │  Cancel  │  │   Save   │            │
│  └──────────┘  └──────────┘            │
└─────────────────────────────────────────┘
```

### Validation Feedback
- If all fields empty: Show error "Please answer at least one question"
- If focus rating invalid (not 1-10 or "N/10"): Show warning but allow save
- Success: Brief toast message "Responses saved!" then modal closes

## 6. Future Analysis & Insights (Post-v1.0)

The accountability data enables powerful self-analysis:

### Potential Analyses
1. **Trend Tracking**
   - Focus rating over time (line chart)
   - Common challenges (word cloud, frequency analysis)
   - Learning accumulation (knowledge graph)

2. **Pattern Recognition**
   - Correlation between focus rating and check-in data
   - Day-of-week patterns (productive days vs. challenging days)
   - Accomplishment velocity (tasks completed per session)

3. **Insights Generation**
   - Most common challenges → suggest solutions
   - Learning themes → recommend resources
   - Tomorrow plans → track completion rate

### Python Analysis Example
```python
import json
from datetime import datetime

# Load accountability data
with open('accountability_box.jsonl', 'r') as f:
    entries = [json.loads(line) for line in f]

# Extract focus ratings
ratings = [
    int(e['q4_focus_rating'].split('/')[0])
    for e in entries
    if e['q4_focus_rating'] and '/' in e['q4_focus_rating']
]

# Calculate average focus
avg_focus = sum(ratings) / len(ratings)
print(f"Average focus rating: {avg_focus:.1f}/10")

# Identify learning themes
learnings = [e['q5_learned'] for e in entries if e['q5_learned']]
# NLP analysis, keyword extraction, etc.
```

## 7. Acceptance Checklist
- [ ] "Accountability Box" button visible in main UI
- [ ] Button opens modal with 5 questions
- [ ] All textareas accept multi-line input
- [ ] Validation prevents empty submissions
- [ ] Focus rating validates format (1-10 or "N/10")
- [ ] Save button writes to `accountability_box.jsonl`
- [ ] JSONL file contains valid JSON on each line
- [ ] Timestamp is ISO 8601 UTC format
- [ ] Cancel button closes modal without saving
- [ ] Escape key closes modal
- [ ] Modal doesn't interfere with running session timer
- [ ] File write errors display user-friendly message
- [ ] Multiple submissions append correctly (no overwrites)
- [ ] Manual testing: Submit 3 responses, verify JSONL file has 3 lines

## 8. Privacy & Security
- ✅ All data stored locally (never sent to cloud)
- ✅ User controls data location (Tauri app config directory)
- ✅ JSONL human-readable (user can inspect with text editor)
- ✅ No encryption (user can add later if desired)
- ✅ No telemetry or analytics on responses
- ✅ User can delete `accountability_box.jsonl` anytime

## 9. Future Enhancements (Post-v1.0)
- [ ] Custom question sets (user defines their own questions)
- [ ] Scheduled prompts (daily reminder to fill accountability box)
- [ ] In-app visualization of accountability data
- [ ] Export accountability data to PDF/CSV
- [ ] Search/filter past accountability entries
- [ ] Link accountability entries to specific focus sessions
- [ ] AI-powered insights from accountability text (local LLM only)
