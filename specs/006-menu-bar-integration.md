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
