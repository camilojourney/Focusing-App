# ⚙️ Spec 002: Calendar Integration

_Constitution: AGENTS.md@2025-11-07_

## 1. Feature Objective
Integrate with macOS Calendar (via EventKit) to display the user's current calendar event during focus sessions. This provides context for check-ins and helps users understand if they're on task relative to their scheduled commitments.

## 2. File & Module Targets
- `src-tauri/src/calendar.rs` (primary module)
  - Handles EventKit integration
  - Manages calendar permission requests
  - Queries current calendar events
- `src-tauri/src/main.rs`
  - Exposes `#[tauri::command] fn get_current_event()`
  - Exposes `#[tauri::command] fn request_calendar_permission()`
- `src/index.html`
  - Displays current calendar event in UI
  - Calls calendar commands via Tauri IPC
- `src-tauri/entitlements.plist`
  - Declares `com.apple.security.personal-information.calendars` entitlement for macOS sandbox

## 3. Business & Technical Logic

### 3.1 Permission Management
1. **Initial Permission Check**
   - On app launch, check if calendar permission has been granted
   - Permission states: NotDetermined, Authorized, Denied, Restricted
   - Store permission state to avoid repeated prompts

2. **Permission Request Flow**
   - User clicks "Request Calendar Permission" button (if needed)
   - App calls `request_calendar_permission()` command
   - Rust backend uses EventKit `EKEventStore::requestAccessToEntityType`
   - System shows native macOS permission dialog
   - Return permission result to frontend

3. **Permission Persistence**
   - macOS remembers permission grant/denial in System Settings
   - User can revoke permission via System Settings > Privacy & Security > Calendars
   - App gracefully handles denied/revoked permissions

### 3.2 Event Querying
1. **Current Event Detection**
   - Query calendar for events where: `now >= event.startDate AND now <= event.endDate`
   - Search across all enabled calendars in user's Calendar app
   - Use EventKit predicate: `predicateForEventsWithStartDate:endDate:calendars:`
   - Time window: current moment ± 1 minute (to handle timing edge cases)

2. **Event Data Extraction**
   - Extract: `title`, `startDate`, `endDate`, `calendar.title`, `location` (optional)
   - Format times using user's locale
   - Handle all-day events differently (show date instead of time)

3. **Multiple Overlapping Events**
   - If multiple events match current time, return the first scheduled event (earliest start time)
   - Rationale: Users typically prioritize earlier-scheduled commitments

4. **No Current Event**
   - Return `None` or empty result
   - Frontend displays: "No current event" or similar placeholder

### 3.3 UI Display
1. **Event Display Format**
   - Show in check-in modal: "Current Event: [Title] ([Start Time] - [End Time])"
   - Example: "Current Event: Team Standup (10:00 AM - 10:30 AM)"
   - If no event: Display nothing or "No scheduled event"

2. **Update Frequency**
   - Query calendar only when check-in modal opens (not continuously)
   - Rationale: Minimizes battery impact, calendar rarely changes mid-session

3. **Error Handling**
   - If permission denied: Show message "Calendar access denied. Enable in System Settings."
   - If calendar query fails: Silently fail, show "Unable to fetch calendar event"
   - Never block check-in flow due to calendar errors

## 4. Data Contracts

### CalendarEvent (Rust struct)
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct CalendarEvent {
    pub title: String,
    pub start_date: String,      // ISO 8601 format
    pub end_date: String,         // ISO 8601 format
    pub calendar_name: String,
    pub location: Option<String>,
}
```

### Command: `get_current_event`
**Request:**
```javascript
const event = await invoke('get_current_event');
```

**Response (Success):**
```json
{
  "title": "Team Standup",
  "start_date": "2025-11-13T10:00:00-08:00",
  "end_date": "2025-11-13T10:30:00-08:00",
  "calendar_name": "Work",
  "location": "Zoom"
}
```

**Response (No Event):**
```json
null
```

**Response (Error):**
```javascript
Err("Calendar permission denied")
```

### Command: `request_calendar_permission`
**Request:**
```javascript
const granted = await invoke('request_calendar_permission');
```

**Response:**
```json
true   // Permission granted
false  // Permission denied or error
```

## 5. Platform-Specific Behavior

### macOS
- ✅ Full EventKit integration
- ✅ System Settings integration for permissions
- ✅ Supports all calendar types (iCloud, Google, Exchange, etc.)

### Windows/Linux
- ⏳ Not implemented (EventKit is macOS-only)
- Future: Could integrate with Outlook/Google Calendar APIs
- Current behavior: Calendar commands return empty/error, UI gracefully hides calendar section

## 6. Implementation Notes

### Performance Considerations
1. **Lazy Calendar Access**
   - Only query calendar when user opens check-in modal
   - Don't poll calendar continuously
   - Cache EventStore instance to avoid repeated initialization

2. **Async Execution**
   - Calendar queries run on background thread (EventKit is thread-safe)
   - Use `tauri::async_runtime::spawn_blocking` for Objective-C calls
   - Never block main UI thread

3. **Memory Management**
   - Release EventKit objects properly (Objective-C memory management)
   - Don't cache event results (calendar can change)
   - EventStore is lightweight, create once per app launch

### Security & Privacy
1. **Entitlements**
   - Declare calendar entitlement in `entitlements.plist`
   - Required for macOS sandbox (App Store compliance)

2. **User Control**
   - User must explicitly grant permission
   - Permission can be revoked anytime in System Settings
   - App must handle revocation gracefully

3. **Data Usage**
   - Only read event data, never write/modify
   - Only access current event, not entire calendar history
   - Data stays in-memory, never logged to disk (privacy-first)

## 7. Acceptance Checklist
- [ ] Calendar permission request dialog appears when `request_calendar_permission()` called
- [ ] Permission grant/denial correctly reflected in UI
- [ ] Current event displays correctly during check-in when event is scheduled
- [ ] No event displays gracefully when calendar is empty
- [ ] Permission denial doesn't block check-in flow
- [ ] Calendar query completes within 500ms
- [ ] Multiple overlapping events handled (first event shown)
- [ ] All-day events display correctly
- [ ] App builds with correct entitlements (`com.apple.security.personal-information.calendars`)
- [ ] Calendar integration gracefully disabled on Windows/Linux builds

## 8. Future Enhancements (Post-v1.0)
- [ ] Show next upcoming event (not just current)
- [ ] Filter calendars (work vs. personal)
- [ ] Deeper integration: suggest session goals based on calendar events
- [ ] Windows/Linux calendar integration (Google Calendar API, Outlook)
- [ ] Calendar write capability (create "Focus Session" events)
