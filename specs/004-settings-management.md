# ⚙️ Spec 004: Settings Management

_Constitution: AGENTS.md@2025-11-07_

## 1. Feature Objective
Provide persistent, user-configurable settings for session duration, check-in interval, and write time. Settings are stored locally in JSON format and survive app restarts, enabling users to customize their focus workflow.

## 2. File & Module Targets
- `src/settings.html`
  - Settings UI with form inputs
  - Load/save settings via Tauri IPC
- `src-tauri/src/main.rs`
  - `#[tauri::command] fn get_settings()` - Load settings from disk
  - `#[tauri::command] fn save_settings(settings: Settings)` - Persist settings to disk
  - `Settings` struct definition
- `~/Library/Application Support/com.focustime.app/settings.json`
  - JSON file storing user preferences
  - Loaded on app launch, written on save

## 3. Business & Technical Logic

### 3.1 Settings Data Model
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub session_duration: u32,    // Total session time in minutes
    pub check_in_interval: u32,   // Minutes between check-ins
    pub write_time: u32,          // Reflection period in seconds
}
```

**Default Values:**
- `session_duration`: 720 minutes (12 hours)
- `check_in_interval`: 15 minutes
- `write_time`: 20 seconds

**Rationale for Defaults:**
- 12 hours: Full workday coverage, accommodates long deep work sessions
- 15 minutes: Optimal interval for maintaining awareness without excessive interruption
- 20 seconds: Enough time for brief reflection, not too long to break flow

### 3.2 Settings Lifecycle

#### 3.2.1 App Launch (First Time)
1. Check if `settings.json` exists
2. If not found: Create file with default settings
3. Load settings into memory
4. Frontend requests settings via `get_settings()`
5. UI initializes with loaded settings

#### 3.2.2 App Launch (Existing User)
1. Read `settings.json` from disk
2. Deserialize JSON → `Settings` struct
3. Validate settings (bounds checking)
4. If invalid: Use defaults, log warning
5. Return settings to frontend

#### 3.2.3 User Opens Settings Window
1. User clicks "Settings" button or menu item
2. `settings.html` window opens
3. Frontend calls `get_settings()`
4. Form populates with current values
5. User can edit values

#### 3.2.4 User Saves Settings
1. User clicks "Save Settings" button
2. Frontend validates inputs (client-side)
3. Frontend calls `save_settings(new_settings)`
4. Rust backend validates settings (server-side)
5. Serialize `Settings` → JSON
6. Write atomically to `settings.json`
7. Return success/error to frontend
8. Frontend shows confirmation message

### 3.3 Validation Rules

#### Session Duration
- **Minimum**: 30 minutes (prevents absurdly short sessions)
- **Maximum**: 1440 minutes (24 hours)
- **Default**: 720 minutes (12 hours)
- **UI Input**: Number field, step=30

#### Check-in Interval
- **Minimum**: 5 minutes (prevents excessive interruptions)
- **Maximum**: 120 minutes (2 hours)
- **Default**: 15 minutes
- **UI Input**: Number field, step=5
- **Constraint**: Must be ≤ session_duration

#### Write Time
- **Minimum**: 10 seconds (allows at least brief reflection)
- **Maximum**: 300 seconds (5 minutes)
- **Default**: 20 seconds
- **UI Input**: Number field, step=5

### 3.4 File I/O Implementation

#### Read Path (`get_settings`)
```rust
fn get_settings(app: AppHandle) -> Result<Settings, String> {
    let settings_path = app.path_resolver()
        .app_config_dir()
        .ok_or("Failed to resolve app config dir")?
        .join("settings.json");

    if !settings_path.exists() {
        // First launch: create default settings
        let defaults = Settings::default();
        save_settings(app, defaults.clone())?;
        return Ok(defaults);
    }

    // Read file
    let contents = std::fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings: {}", e))?;

    // Parse JSON
    let settings: Settings = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse settings: {}", e))?;

    // Validate
    validate_settings(&settings)?;

    Ok(settings)
}
```

#### Write Path (`save_settings`)
```rust
fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    // Validate first
    validate_settings(&settings)?;

    let settings_path = app.path_resolver()
        .app_config_dir()
        .ok_or("Failed to resolve app config dir")?
        .join("settings.json");

    // Ensure directory exists
    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config dir: {}", e))?;
    }

    // Serialize to pretty JSON (human-readable)
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    // Atomic write (write to temp file, then rename)
    let temp_path = settings_path.with_extension("json.tmp");
    std::fs::write(&temp_path, json)
        .map_err(|e| format!("Failed to write settings: {}", e))?;
    std::fs::rename(&temp_path, &settings_path)
        .map_err(|e| format!("Failed to finalize settings: {}", e))?;

    Ok(())
}
```

#### Validation (`validate_settings`)
```rust
fn validate_settings(settings: &Settings) -> Result<(), String> {
    if settings.session_duration < 30 || settings.session_duration > 1440 {
        return Err("Session duration must be between 30 and 1440 minutes".to_string());
    }
    if settings.check_in_interval < 5 || settings.check_in_interval > 120 {
        return Err("Check-in interval must be between 5 and 120 minutes".to_string());
    }
    if settings.write_time < 10 || settings.write_time > 300 {
        return Err("Write time must be between 10 and 300 seconds".to_string());
    }
    if settings.check_in_interval > settings.session_duration {
        return Err("Check-in interval cannot exceed session duration".to_string());
    }
    Ok(())
}
```

### 3.5 Settings Window UI

#### Layout
```
┌────────────────────────────────────┐
│  Settings                    [X]   │
├────────────────────────────────────┤
│                                    │
│  Session Duration (minutes)        │
│  ┌──────────────┐                  │
│  │     720      │ ◀ ▶             │
│  └──────────────┘                  │
│  How long should a focus session   │
│  last? (30-1440 minutes)           │
│                                    │
│  Check-in Interval (minutes)       │
│  ┌──────────────┐                  │
│  │      15      │ ◀ ▶             │
│  └──────────────┘                  │
│  How often should you check in?    │
│  (5-120 minutes)                   │
│                                    │
│  Write Time (seconds)              │
│  ┌──────────────┐                  │
│  │      20      │ ◀ ▶             │
│  └──────────────┘                  │
│  Reflection period after check-in  │
│  (10-300 seconds)                  │
│                                    │
│  ┌──────────┐  ┌──────────┐       │
│  │  Cancel  │  │   Save   │       │
│  └──────────┘  └──────────┘       │
└────────────────────────────────────┘
```

#### Interaction Flow
1. User adjusts values using number inputs or stepper controls
2. Client-side validation on input (show red border if invalid)
3. "Save" button disabled if validation fails
4. On save: Call `save_settings()`, show success/error message
5. On cancel: Close window without saving

## 4. Data Contracts

### Settings JSON File
```json
{
  "session_duration": 720,
  "check_in_interval": 15,
  "write_time": 20
}
```

**File Location:**
- macOS: `~/Library/Application Support/com.focustime.app/settings.json`
- Windows: `%APPDATA%\com.focustime.app\settings.json`
- Linux: `~/.config/focus-time/settings.json`

### Command: `get_settings`
**Request:**
```javascript
const settings = await invoke('get_settings');
```

**Response (Success):**
```json
{
  "session_duration": 720,
  "check_in_interval": 15,
  "write_time": 20
}
```

**Response (Error):**
```javascript
Err("Failed to read settings: Permission denied")
```

### Command: `save_settings`
**Request:**
```javascript
await invoke('save_settings', {
  settings: {
    session_duration: 720,
    check_in_interval: 15,
    write_time: 20
  }
});
```

**Response (Success):**
```javascript
Ok(())  // void success
```

**Response (Error):**
```javascript
Err("Session duration must be between 30 and 1440 minutes")
```

## 5. Error Handling

### Common Errors
1. **File Permission Error**
   - Cause: User lacks write permission to app config directory
   - Mitigation: Show error message, guide user to check permissions
   - Fallback: Use in-memory defaults, warn settings won't persist

2. **Disk Full**
   - Cause: No disk space to write settings.json
   - Mitigation: Show error, suggest freeing disk space
   - Fallback: Keep existing settings, retry on next save

3. **JSON Parse Error**
   - Cause: Corrupted settings.json (manual edit, disk corruption)
   - Mitigation: Delete corrupted file, use defaults, notify user
   - Log: Save corrupted file as `settings.json.bak` for debugging

4. **Validation Error**
   - Cause: Invalid values (manual file edit, UI bug)
   - Mitigation: Show validation error, prevent save
   - Frontend: Highlight invalid field in red

### Error Recovery Strategy
```rust
fn load_settings_with_recovery(app: AppHandle) -> Settings {
    match get_settings(app.clone()) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Failed to load settings: {}", e);
            // Try to backup corrupted file
            let _ = backup_corrupted_settings(&app);
            // Use defaults
            let defaults = Settings::default();
            // Try to save defaults (may fail, but worth trying)
            let _ = save_settings(app, defaults.clone());
            defaults
        }
    }
}
```

## 6. Performance Considerations

### Read Performance
- Settings read once per app launch
- Cached in memory (no repeated disk reads)
- File size: ~100 bytes (negligible I/O time)
- Expected latency: <5ms

### Write Performance
- Settings written only when user clicks "Save"
- Atomic write prevents corruption
- File size: ~100 bytes (instant write)
- Expected latency: <10ms

### Memory Footprint
- Settings struct: ~12 bytes (3 × u32)
- JSON string: ~100 bytes
- Total overhead: <1 KB (negligible)

## 7. Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings_valid() {
        let settings = Settings::default();
        assert!(validate_settings(&settings).is_ok());
    }

    #[test]
    fn test_invalid_session_duration() {
        let settings = Settings {
            session_duration: 10,  // Too short
            ..Default::default()
        };
        assert!(validate_settings(&settings).is_err());
    }

    #[test]
    fn test_check_in_exceeds_session() {
        let settings = Settings {
            session_duration: 60,
            check_in_interval: 90,  // Longer than session
            ..Default::default()
        };
        assert!(validate_settings(&settings).is_err());
    }
}
```

### Integration Tests
1. **Fresh Install Test**
   - Delete settings.json
   - Launch app
   - Verify defaults loaded
   - Verify settings.json created

2. **Save/Load Round-Trip Test**
   - Save custom settings
   - Restart app
   - Verify settings persisted correctly

3. **Validation Test**
   - Try to save invalid settings
   - Verify error message shown
   - Verify file not modified

## 8. Acceptance Checklist
- [ ] Settings window opens from main UI
- [ ] Form populates with current settings on open
- [ ] All inputs validate client-side (red border on invalid)
- [ ] "Save" button disabled when validation fails
- [ ] Save button calls `save_settings()` successfully
- [ ] Settings persist across app restarts
- [ ] Default settings created on first launch
- [ ] Validation prevents saving invalid settings
- [ ] Error messages user-friendly (not technical jargon)
- [ ] Atomic write prevents file corruption
- [ ] Cancel button closes window without saving
- [ ] Settings JSON human-readable (pretty-printed)
- [ ] Manual file edit with invalid JSON recovers gracefully
- [ ] Settings file location follows OS conventions

## 9. Future Enhancements (Post-v1.0)
- [ ] Presets: "Deep Work" (60 min), "Pomodoro" (25 min), etc.
- [ ] Export/import settings (share configs between devices)
- [ ] Advanced settings: sound notifications, desktop switch on/off
- [ ] Per-goal settings (different intervals for different goals)
- [ ] Settings history/undo
- [ ] Settings sync (optional cloud backup)
