# ⚙️ Spec 006: Menu Bar Integration

_Constitution: AGENTS.md@2025-11-07_

## 1. Feature Objective
Provide a native macOS menu bar (system tray) experience that displays the focus timer in real-time, adapts to system theme (dark/light mode), and offers quick access to app functions via tray menu. The menu bar presence keeps the app always accessible without cluttering the Dock.

## 2. File & Module Targets
- `src-tauri/src/main.rs`
  - `SystemTray` initialization
  - `#[tauri::command] fn update_tray_timer(time_text: String)` - Updates menu bar display
  - Tray menu event handlers
- `src-tauri/icons/18x18.png`
  - Menu bar icon (template image)
  - Pure black on transparent background
  - macOS automatically inverts for dark/light mode
- `src-tauri/tauri.conf.json`
  - `systemTray` configuration
  - Icon path, template mode settings
- `src/index.html`
  - JavaScript timer that calls `update_tray_timer()` every second

## 3. Business & Technical Logic

### 3.1 Menu Bar Icon Design

#### Template Image Requirements (macOS)
macOS menu bar icons must be "template images" to adapt to system theme:

**Technical Specs:**
- **Format**: PNG (24-bit or 32-bit with alpha)
- **Size**: 18×18 pixels (exactly, no scaling)
- **Color**: Pure black (#000000) for visible pixels
- **Background**: Fully transparent (alpha = 0)
- **Anti-aliasing**: NONE (no gray pixels)
- **Pixels**: Only black (255,255,255,255) or transparent (0,0,0,0)

**Why No Anti-Aliasing?**
- macOS applies its own anti-aliasing when rendering
- Gray pixels cause incorrect rendering in dark mode
- Template mode inverts colors: black → white (dark mode), black → black (light mode)

**Lessons Learned:**
- ❌ Initial attempt used anti-aliased icon → invisible in menu bar
- ❌ Setting `iconAsTemplate: true` with anti-aliased icon → still invisible
- ✅ Solution: Pure black/transparent PNG + `iconAsTemplate: true` → perfect

#### Icon Creation Script (Python)
```python
from PIL import Image, ImageDraw

# Create 18x18 image with transparency
img = Image.new('RGBA', (18, 18), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Draw brain outline (simple representation)
# Use pure black (0, 0, 0, 255) - no anti-aliasing
draw.ellipse([2, 2, 16, 16], fill=(0, 0, 0, 255))
draw.ellipse([6, 6, 12, 12], fill=(0, 0, 0, 0))  # Cut out center

# Save without any compression or optimization
img.save('18x18.png', 'PNG')
```

### 3.2 Tray Menu Structure

#### Menu Items
```
┌────────────────────────────┐
│  🧠 00:15:30              │  ← Timer display (updated every 1s)
├────────────────────────────┤
│  Show                      │  ← Open main window
│  Settings                  │  ← Open settings window
│  ───────────────────────── │
│  Quit                      │  ← Exit app
└────────────────────────────┘
```

**Menu Item Behaviors:**
1. **Timer Display** (read-only)
   - Shows current timer: session time remaining OR check-in countdown
   - Format: `HH:MM:SS` (hours:minutes:seconds)
   - Example: `12:00:00` → `11:45:30` → `00:05:00`
   - Icon: 🧠 brain emoji (or custom icon)
   - Not clickable (display only)

2. **Show**
   - Brings main window to front
   - If window closed: reopens it
   - If window minimized: restores it
   - macOS behavior: switches to app's Desktop space

3. **Settings**
   - Opens settings window (`settings.html`)
   - If already open: brings to front
   - Independent window (can be open alongside main window)

4. **Quit**
   - Gracefully shuts down app
   - Saves any pending data
   - Doesn't prompt for confirmation (instant quit)
   - Standard macOS behavior (Cmd+Q also works)

### 3.3 Timer Update Mechanism

#### Update Frequency
- **Interval**: Every 1 second
- **Triggered by**: JavaScript timer in `index.html`
- **Command**: `update_tray_timer(time_text)`

#### Implementation
**Frontend (JavaScript):**
```javascript
async function updateTrayTimer() {
    const timeText = formatTime(sessionTimeRemaining);
    try {
        await invoke('update_tray_timer', { timeText });
    } catch (error) {
        console.error('Failed to update tray timer:', error);
        // Don't block UI on tray update failure
    }
}

// Call every second
setInterval(updateTrayTimer, 1000);

function formatTime(seconds) {
    const hours = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${pad(hours)}:${pad(mins)}:${pad(secs)}`;
}

function pad(num) {
    return num.toString().padStart(2, '0');
}
```

**Backend (Rust):**
```rust
#[tauri::command]
fn update_tray_timer(app: AppHandle, time_text: String) -> Result<(), String> {
    let tray_handle = app.tray_handle();

    // Update the timer menu item (first item in menu)
    tray_handle
        .get_item("timer")
        .set_title(&format!("🧠 {}", time_text))
        .map_err(|e| format!("Failed to update tray: {}", e))?;

    Ok(())
}
```

**Tray Initialization:**
```rust
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

fn create_tray_menu() -> SystemTray {
    let timer = CustomMenuItem::new("timer".to_string(), "🧠 00:00:00").disabled();
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(timer)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}
```

### 3.4 Tray Menu Event Handling

```rust
use tauri::SystemTrayEvent;

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "show" => {
                    if let Some(window) = app.get_window("main") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_window("settings") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    } else {
                        // Create settings window if doesn't exist
                        tauri::WindowBuilder::new(
                            app,
                            "settings",
                            tauri::WindowUrl::App("settings.html".into())
                        )
                        .title("Settings")
                        .inner_size(400.0, 500.0)
                        .resizable(false)
                        .build()
                        .unwrap();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        SystemTrayEvent::LeftClick { .. } => {
            // Optional: Show main window on tray icon click
            if let Some(window) = app.get_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
        _ => {}
    }
}
```

### 3.5 Theme Adaptation

#### Automatic Theme Detection
- macOS automatically inverts template images based on system theme
- **Light Mode**: Black icon stays black
- **Dark Mode**: Black icon becomes white
- **No code needed**: Template image + `iconAsTemplate: true` handles this

#### Configuration (tauri.conf.json)
```json
{
  "tauri": {
    "systemTray": {
      "iconPath": "icons/18x18.png",
      "iconAsTemplate": true,
      "menuOnLeftClick": false
    }
  }
}
```

**Field Explanations:**
- `iconPath`: Relative to `src-tauri/` directory
- `iconAsTemplate`: Enables macOS template mode (auto theme adaptation)
- `menuOnLeftClick`: false = left-click shows main window (not menu)

### 3.6 Cross-Platform Behavior

#### macOS
- ✅ Full menu bar integration
- ✅ Template image theme adaptation
- ✅ Native menu rendering
- ✅ Standard tray behavior

#### Windows
- ⚠️ System tray (not menu bar)
- ⚠️ No template mode (use colored icon)
- ✅ Tray menu works (same API)
- ⚠️ Different visual style (Windows system tray design)

#### Linux
- ⚠️ System tray support varies by desktop environment
- ✅ Works on GNOME, KDE, XFCE (with appindicator)
- ⚠️ May not work on Wayland (depends on compositor)
- ⚠️ No template mode (use SVG for scaling)

**Current Focus**: macOS only (MVP). Windows/Linux support in v0.2+.

## 4. Data Contracts

### Command: `update_tray_timer`
**Request:**
```javascript
await invoke('update_tray_timer', {
  timeText: "12:34:56"
});
```

**Response (Success):**
```javascript
Ok(())  // void success
```

**Response (Error):**
```javascript
Err("Failed to update tray: Tray not initialized")
```

## 5. Performance Considerations

### Update Frequency Impact
- **Updates/minute**: 60 (once per second)
- **IPC overhead**: ~0.5ms per update
- **Tray API overhead**: ~0.5ms per update
- **Total CPU**: <1% (negligible)

### Battery Impact
- **Timer loop**: Runs in JavaScript (web engine)
- **IPC calls**: 60/minute (minimal network overhead)
- **macOS tray update**: Native API (optimized)
- **Estimated battery drain**: <0.5% per hour

### Memory Footprint
- **Tray menu**: ~10 KB (menu items, strings)
- **Icon**: 18×18 PNG ≈ 1 KB
- **Total overhead**: <50 KB (negligible)

## 6. Error Handling

### Common Errors
1. **Tray Initialization Failure**
   - Cause: System doesn't support tray (rare)
   - Mitigation: Fall back to Dock icon only
   - User Impact: App still works, no tray

2. **Timer Update Failure**
   - Cause: Tray handle invalid (app shutting down)
   - Mitigation: Silently fail, don't crash
   - User Impact: Timer stops updating (minor)

3. **Icon Not Found**
   - Cause: `18x18.png` missing from build
   - Mitigation: Build fails early (Tauri validation)
   - User Impact: Can't build app (dev-time error)

4. **Template Mode Not Working**
   - Cause: Icon has anti-aliasing or wrong format
   - Mitigation: Use icon creation script, validate
   - User Impact: Icon invisible or wrong color

### Error Recovery
```rust
#[tauri::command]
fn update_tray_timer(app: AppHandle, time_text: String) -> Result<(), String> {
    let tray_handle = match app.try_tray_handle() {
        Some(handle) => handle,
        None => {
            eprintln!("Tray not initialized, skipping update");
            return Ok(());  // Fail silently, don't block UI
        }
    };

    match tray_handle.get_item("timer").set_title(&format!("🧠 {}", time_text)) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to update tray timer: {}", e);
            Ok(())  // Fail silently
        }
    }
}
```

## 7. Testing Strategy

### Manual Testing Checklist
- [ ] Icon visible in menu bar after app launch
- [ ] Icon color correct in light mode (black/dark)
- [ ] Icon color correct in dark mode (white/light)
- [ ] Timer updates every second (check with stopwatch)
- [ ] Timer format correct (HH:MM:SS)
- [ ] "Show" menu item opens main window
- [ ] "Settings" menu item opens settings window
- [ ] "Quit" menu item exits app
- [ ] Left-click on icon shows main window
- [ ] Right-click on icon shows menu
- [ ] Timer continues updating while window hidden
- [ ] Menu bar icon persists across Desktop switches

### Automated Testing (Future)
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_time_formatting() {
        assert_eq!(format_time(3661), "01:01:01");
        assert_eq!(format_time(0), "00:00:00");
        assert_eq!(format_time(43200), "12:00:00");
    }

    #[test]
    fn test_tray_menu_structure() {
        let menu = create_tray_menu();
        // Verify menu has 4 items + 2 separators
        // (Requires Tauri testing infrastructure)
    }
}
```

## 8. Acceptance Checklist
- [ ] Menu bar icon appears on app launch
- [ ] Icon is theme-aware (black in light mode, white in dark mode)
- [ ] Timer displays in tray menu (read-only item)
- [ ] Timer updates every 1 second with correct format (HH:MM:SS)
- [ ] "Show" menu item opens/focuses main window
- [ ] "Settings" menu item opens settings window
- [ ] "Quit" menu item exits app gracefully
- [ ] Left-click on tray icon shows main window
- [ ] Right-click (or any click on some macOS versions) shows menu
- [ ] Icon persists when main window hidden
- [ ] App doesn't appear in Dock (menu bar app only)
- [ ] Memory usage <50 MB with tray active
- [ ] CPU usage <1% with timer updating
- [ ] No console errors when updating timer
- [ ] Manual testing: Switch system theme, icon adapts immediately

## 9. Debugging Tools

### Verify Template Image
```bash
# Check PNG properties
file src-tauri/icons/18x18.png
# Should output: PNG image data, 18 x 18, 8-bit/color RGBA

# Check for anti-aliasing (look for gray pixels)
python3 << EOF
from PIL import Image
img = Image.open('src-tauri/icons/18x18.png')
pixels = img.load()
for y in range(18):
    for x in range(18):
        r, g, b, a = pixels[x, y]
        if a > 0 and (r != 0 or g != 0 or b != 0):
            print(f"Warning: Non-black pixel at ({x}, {y}): {pixels[x, y]}")
EOF
```

### Monitor Tray Updates
```javascript
// Add logging to update function
async function updateTrayTimer() {
    const timeText = formatTime(sessionTimeRemaining);
    console.log(`Updating tray: ${timeText}`);
    await invoke('update_tray_timer', { timeText });
}
```

### macOS System Tray Debugging
```bash
# Restart Dock (refreshes menu bar)
killall Dock

# Check menu bar item count
# System Preferences > Control Center > Menu Bar Items
```

## 10. Future Enhancements (Post-v1.0)
- [ ] Custom tray menu items per session mode
- [ ] Right-click quick actions (start session, pause, reset)
- [ ] Tray notification badges (unread check-ins, etc.)
- [ ] Multiple tray icons for different states (focused, paused, distracted)
- [ ] Windows/Linux tray support with platform-specific icons
- [ ] Tray tooltip (hover shows detailed session info)
- [ ] Configurable tray menu (user customizes menu items)
- [ ] Tray animation (subtle pulse during check-ins)
- [ ] macOS menu bar extra (more detailed info in tray menu)

## 11. Lessons Learned

### Icon Template Mode Debug (2-Day Journey)
**Problem**: Menu bar icon invisible after build.

**Attempts:**
1. ❌ Tried increasing icon size → Still invisible
2. ❌ Tried removing `iconAsTemplate: true` → Icon showed but wrong color
3. ❌ Tried different PNG formats → Still invisible
4. ✅ **Solution**: Created pure black/transparent PNG with NO anti-aliasing

**Root Cause**:
- Anti-aliased icon had gray pixels (e.g., RGB 128,128,128)
- macOS template mode expects ONLY black (0,0,0) or transparent (alpha=0)
- Gray pixels rendered incorrectly → invisible in menu bar

**Key Insight**:
- macOS documentation says "template images use only black and transparent"
- This is LITERAL: no shades of gray allowed
- Always verify PNG has only 2 colors: black (#000000) and transparent

**Prevention**:
- Use Python script to generate icon programmatically
- Never use Photoshop/GIMP anti-aliasing for macOS tray icons
- Validate PNG with pixel checker before building


## Menu bar behavior
1. When I click the right mouse button on the menu bar icon, it shows the tray menu with timer, Show, Settings, and Quit options.
2. When I click the left mouse button on the menu bar icon, it opens the main application window.
3. When the window is open, and I click the left mouse button on the menu bar icon, it should close the window.

## 12. Tauri 2 Implementation Notes (2025-11-17)

### Architecture Overview: How Everything Connects

```
┌─────────────────────────────────────────────────────────┐
│                    macOS Menu Bar                        │
│  ┌──────────┐                                           │
│  │  🧠 15:00 │  ← Tray Icon with Timer Title           │
│  └──────────┘                                           │
└───────────────────────────────┬─────────────────────────┘
                                │
                    ┌───────────┴───────────┐
                    │   Click Events        │
                    └───────────┬───────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
   Left Click              Right Click          Double Click
        │                       │                       │
        ▼                       ▼                       ▼
  Toggle Window           Show Menu               Quit App
        │                       │
        │                   ┌───┴────┐
        │                   │ • Show  │
        │                   │ • Settings
        │                   │ • Quit  │
        │                   └────────┘
        │
        ▼
┌─────────────────────────────────────┐
│     Main Window (index.html)         │
│  ┌─────────────────────────────┐   │
│  │  Timer: 15:00                │   │
│  │  [Start Session] [Reset]    │   │ ← Buttons need focus!
│  │  [Settings] [Review]         │   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
```

### The Focus Problem & Solution

**Why Buttons Don't Work (Tauri 2 Specific Issue):**

In Tauri 2, macOS apps have an "activation policy" that controls how they behave:

1. **`ActivationPolicy::Accessory`** (Menu bar only)
   - ✅ App stays in menu bar only (no Dock icon)
   - ❌ **Windows cannot become "key window"** (can't receive clicks!)
   - ❌ Buttons, inputs, any interaction = broken
   - Use case: Purely passive display apps (clocks, indicators)

2. **`ActivationPolicy::Regular`** (Normal app)
   - ✅ Windows can become "key window" (clicks work!)
   - ✅ Buttons, inputs, all interactions work
   - ⚠️ App appears in Dock by default
   - Use case: Interactive apps that need user input

3. **`ActivationPolicy::Regular` + `skipTaskbar: false`**
   - ✅ Windows get focus properly
   - ✅ All interactions work
   - ⚠️ Shows in Dock (trade-off for functionality)
   - **Current solution for Tauri 2**

### Why We Changed Configuration

**Original (Broken):**
```rust
// main.rs line ~305
app.set_activation_policy(tauri::ActivationPolicy::Accessory);
```
```json
// tauri.conf.json
"skipTaskbar": true
```
**Result:** Menu bar only app, but buttons don't work! 🐛

**Current (Working):**
```rust
// main.rs line ~305
app.set_activation_policy(tauri::ActivationPolicy::Regular);
```
```json
// tauri.conf.json
"skipTaskbar": false  // Temporarily disabled for functionality
```
**Result:** App works fully, shows in Dock temporarily 🎯

### File Connection Map

Here's how different files work together:

```
src-tauri/src/main.rs
├─ Line 17-20: Import Tauri APIs (Menu, TrayIcon, Window)
├─ Line 40-80: Settings management (loads check-in intervals)
├─ Line 130-160: Window positioning logic (positions below tray icon)
├─ Line 280-290: Menu creation (Show, Settings, Quit)
├─ Line 305: **ActivationPolicy** (controls focus capability)
├─ Line 320-335: Tray icon setup (.title() for timer display)
└─ Line 340-380: Click handlers (left/right/double-click logic)
        │
        └──> Calls window.show()/hide()
                    │
                    ▼
src/index.html
├─ Line 1-100: CSS styling (glassmorphic design)
├─ Line 400-500: Button HTML (Start, Reset, Settings)
├─ Line 700-800: DOM references (getElementById for buttons)
├─ Line 1000-1100: Timer logic (countdown, updates tray)
├─ Line 1200-1300: Session management (start/pause/reset)
└─ Line 1400-1500: Event listeners (button.addEventListener)
        │
        └──> Calls invoke('tauri_command')
                    │
                    ▼
        Back to main.rs Tauri commands
```

### Critical Dependencies

**For Buttons to Work:**
1. ✅ `ActivationPolicy::Regular` (allows window focus)
2. ✅ `withGlobalTauri: true` (enables Tauri APIs in webview)
3. ✅ `skipTaskbar: false` (allows proper focus in Tauri 2)
4. ✅ `window.show() + window.set_focus()` (activates window)

**For Menu Bar Timer:**
1. ✅ `TrayIconBuilder::with_id("main")` (creates tray)
2. ✅ `.title(&initial_time)` (sets timer text)
3. ✅ `invoke('update_tray_timer')` (updates every second)
4. ✅ `tray.set_title(Some(&timer_text))` (updates display)

**For Click Events:**
1. ✅ `.on_tray_icon_event()` (handles tray clicks)
2. ✅ `TrayIconEvent::Click` (detects left/right/double)
3. ✅ `MouseButton::Left/Right` (distinguishes buttons)
4. ✅ `MouseButtonState::Up` (prevents duplicate events)

### Known Limitations (Tauri 2)

1. **Dock Icon Trade-off**
   - To get working buttons, we need `ActivationPolicy::Regular`
   - With Regular policy, `skipTaskbar: true` doesn't hide Dock icon reliably
   - **Workaround**: Accept Dock icon, or explore Tauri 2 native hiding APIs

2. **Double-Click Events**
   - Click events sometimes fire twice (both Down and Up states)
   - **Fix**: Only handle `MouseButtonState::Up`
   - Still seeing duplicates? May be Tauri 2 event bubbling issue

3. **Window Focus Timing**
   - Window needs explicit `set_focus()` after `show()`
   - macOS sometimes doesn't focus window immediately
   - **Fix**: Call both `.show()` and `.set_focus()` sequentially

---

## 13. macOS Sequoia Compatibility (November 2025 Update)

### Issue: Invisible Tray Icon on macOS Sequoia 15.x

**Problem**: Tray icon created successfully (logs show "✅ Tray icon BUILT successfully") but icon is invisible in menu bar.

**Root Cause**: Template mode (`icon_as_template(true)`) with RGBA colored icon (44x44 PNG with full color).

**Technical Explanation**:
- Template mode expects **alpha-mask only** (black + transparent pixels)
- macOS Sequoia renders colored PNGs in template mode as **completely transparent**
- The stricter rendering in Sequoia doesn't auto-convert colored icons like previous versions

**Solution Applied** (November 18, 2025):
```rust
// src-tauri/src/main.rs:383
.icon_as_template(false) // CRITICAL FIX: Disable template mode for macOS Sequoia
```

**Trade-offs**:
- ✅ Icon is now visible on all macOS versions
- ✅ Full color icon displays correctly
- ❌ Icon will NOT auto-adapt to light/dark mode
- ❌ Icon stays same color regardless of menu bar theme

**Alternative Solution** (For Future):
Create two separate icons:
1. Template icon (black + transparent for template mode)
2. Colored icon (for non-template mode)

Switch based on macOS version detection:
```rust
let use_template = if macos_version >= 15.0 { false } else { true };
TrayIconBuilder::with_id("main")
    .icon_as_template(use_template)
    ...
```

**Related Files**:
- `src-tauri/icons/tray-44x44.png` — Current RGBA colored icon (2778 bytes)
- `src-tauri/src/main.rs:369-387` — Tray icon builder configuration

**Testing on Sequoia**:
```bash
# Verify icon file format
file src-tauri/icons/tray-44x44.png
# Output: PNG image data, 44 x 44, 8-bit/color RGBA, non-interlaced

# Check if icon appears in menu bar
ps aux | grep focus-time
# App should be running with visible icon in menu bar

# Monitor tray events
tail -f /tmp/tauri-dev-logs.txt | grep "Tray"
```

**Lessons Learned**:
1. Always test on latest macOS version before release
2. Template mode requirements are stricter in newer macOS versions
3. RGBA icons require `icon_as_template(false)` on Sequoia
4. Consider creating platform-specific icon variants
5. Document OS-specific rendering differences

---

## 14. Window Positioning Architecture (November 2025)

### Current Implementation

**Three Positioning Modes**:

1. **Centered Positioning** (`position_window_centered()`)
   - Used for: Check-in interruptions every 15 minutes
   - Behavior: Centers window on primary monitor
   - Purpose: Consistent, predictable location for quick responses
   - Implementation: `src-tauri/src/main.rs:240-265`

2. **Tray-Relative Positioning** (`position_window_at_top()`)
   - Used for: User clicks tray icon to show window
   - Behavior: Positions window near tray icon location
   - Purpose: Spatial proximity to clicked element
   - Implementation: `src-tauri/src/main.rs:160-235`

3. **User Draggable**
   - Used for: Manual window repositioning by user
   - Behavior: Full window draggability with `-webkit-app-region: drag`
   - Purpose: User control during interaction
   - Implementation: `src/index.html:456-473` (drag-handle CSS)

**Auto-Hide Behavior**:
- Window hides after "Start Focus" pressed
- Window hides immediately after check-in response
- Window shows centered on every check-in trigger
- Window shows near tray when tray icon clicked

**Window Decorations**:
- Standard macOS controls enabled (`"decorations": true`)
- Red/yellow/green buttons for close/minimize/maximize
- Allows native macOS window management

---

## 15. Future Improvements

1. **Hide Dock Icon (Tauri 2 Native)**
   - Explore `NSApplication.setActivationPolicy()` via objc crate
   - Directly call macOS APIs to hide Dock icon with Regular policy
   - Would allow full functionality + menu bar only

2. **Focus Shield Alternative**
   - Use `NSPanel` instead of `NSWindow` for menu bar windows
   - Panels can accept focus without appearing in Dock
   - Requires custom Tauri window type

3. **Event Deduplication**
   - Track event timestamps to filter duplicate events
   - Ignore events within 50ms of each other
   - Already implemented for double-click (300ms window)

4. **Dynamic Icon Mode**
   - Detect macOS version at runtime
   - Use template mode for macOS < 15, colored for >= 15
   - Provides best experience on all OS versions

5. **Window Position Memory**
   - Remember last user-dragged position
   - Restore position on next tray click
   - Reset to tray-relative on check-in

### Testing Checklist (Tauri 2 Specific)

- [x] Tray icon appears in menu bar
- [x] Timer text shows next to icon (e.g., "15:00")
- [x] Left-click shows window at correct position
- [x] Left-click again hides window
- [x] Right-click shows menu with Show/Settings/Quit
- [x] **Buttons are clickable** (Start Session works!)
- [x] Settings window opens
- [x] Timer updates in menu bar every second
- [ ] Double-click closes app (not implemented)
- [x] Window stays on top (alwaysOnTop)
- [x] Transparent background works
- [x] DevTools opens with right-click → Inspect Element
- [x] Window hides on Start
- [x] Window hides after check-in response
- [x] Window centers on check-in trigger
- [x] Tray icon visible on macOS Sequoia

### Debug Commands

**Check current activation policy:**
```bash
# In DevTools Console (Cmd+Opt+I):
await window.__TAURI__.core.invoke('get_activation_policy')
```

**Monitor window focus:**
```javascript
// In DevTools Console:
window.addEventListener('focus', () => console.log('✅ Window focused'));
window.addEventListener('blur', () => console.log('❌ Window lost focus'));
```

**Check click event frequency:**
```rust
// In main.rs, add to click handler:
eprintln!("🔍 Click at {:?} - {:?}",
    SystemTime::now(),
    event
);
```

**Test auto-hide behavior:**
```javascript
// In browser console:
console.log('🔽 Testing hide on Start...');
// Press Start button, window should hide
console.log('🔽 Testing hide on check-in response...');
// Select check-in option, window should hide immediately
```
