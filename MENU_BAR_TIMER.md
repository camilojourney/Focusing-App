# Menu Bar Timer - How It Works

## Summary

✅ **The app is already configured to start minimized and show the countdown in the menu bar!**

## What You'll See

### When App Starts:
1. **No window appears** (starts hidden)
2. **Menu bar shows**: `🧠 15:00`
3. **Click the icon** to show/hide the main window

### When Session is Running:
The menu bar updates **every second**:
```
🧠 15:00  → Start
🧠 14:59
🧠 14:58
🧠 14:57
...
🧠 00:05
🧠 00:04
🧠 00:03
🧠 00:02
🧠 00:01
🧠 00:00  → Check-in triggers!
```

### During Check-in (Writing Time):
```
🧠 ✍️ 20s
🧠 ✍️ 19s
🧠 ✍️ 18s
...
🧠 ✍️ 1s
🧠 15:00  → Session resumes
```

## Implementation Details

### Frontend (index.html)

**Timer Updates Every Second:**
```javascript
checkInInterval = setInterval(() => {
    checkInTimeRemaining--;  // 900 → 899 → 898 ...
    updateDisplay();         // ← Updates menu bar!

    if (checkInTimeRemaining <= 0) {
        triggerCheckIn();
    }
}, 1000);
```

**Update Display Function:**
```javascript
async function updateDisplay() {
    let trayText = '';

    if (isWriting) {
        trayText = `✍️ ${writeTimeRemaining}s`;
    } else {
        trayText = formatTime(checkInTimeRemaining);  // "15:00"
    }

    // Update menu bar
    await invoke('update_tray_timer', { timerText: trayText });
}
```

### Backend (main.rs)

**Tray Update Command:**
```rust
#[tauri::command]
fn update_tray_timer(app: AppHandle, timer_text: String) -> Result<(), String> {
    let tray = app.tray_handle().get_item("timer");
    tray.set_title(&format!("🧠 {}", timer_text))
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

**Initial Menu:**
```rust
let timer_item = CustomMenuItem::new("timer", "🧠 15:00").disabled();
let show = CustomMenuItem::new("show", "Show Timer");
let settings_item = CustomMenuItem::new("settings", "Settings");
let quit = CustomMenuItem::new("quit", "Quit");
```

### Configuration (tauri.conf.json)

**Window Starts Hidden:**
```json
{
  "windows": [{
    "visible": false,      // ← Starts minimized
    "skipTaskbar": true,   // ← Not in dock
    "alwaysOnTop": true    // ← Appears on top when shown
  }]
}
```

## How to Use

### Starting the App

**Method 1: npm (Development)**
```bash
npm run dev
```

**Method 2: Built App (Production)**
```bash
npm run build
# Then open the .app from target/release/bundle/
```

### What Happens:
1. App launches (no window visible)
2. Menu bar icon appears: `🧠 15:00`
3. Click icon → Show main window
4. Enter goal, click "Start Session"
5. Click icon again → Hide window
6. **Timer continues in menu bar!**

### Menu Bar Interactions

**Left Click:**
- Show window (if hidden)
- Hide window (if visible)

**Right Click (or Ctrl+Click):**
```
🧠 14:35         ← Current timer (disabled, just shows time)
───────
Show Timer      ← Show main window
Settings        ← Open settings
───────
Quit            ← Exit app
```

## Workflow Example

### Focused Work Session:

```
1. Launch app
   → Menu bar: 🧠 15:00

2. Click icon → Window appears
   → Enter goal: "Write thesis chapter"
   → Click "Start Session"

3. Click icon again → Window hides
   → Menu bar starts counting: 🧠 14:59 ... 14:58 ...

4. Work for 15 minutes while glancing at menu bar

5. Timer hits 00:00
   → Desktop switches
   → Window appears with check-in screen
   → Menu bar: 🧠 ✍️ 20s

6. Click status button (e.g., "✅ On Task")
   → Window can be hidden again
   → Timer resets: 🧠 15:00
   → Continue working
```

## Benefits

### ✅ Non-Intrusive
- Window stays hidden while working
- Glance at menu bar for time remaining
- No need to keep window open

### ✅ Always Visible
- Menu bar always shows current countdown
- Can't forget about it
- Easy to track progress

### ✅ Minimal Distraction
- Just a timer in menu bar
- No notifications (except at check-in)
- You control when to show window

## Customization

### Change Menu Bar Icon

Edit `src-tauri/src/main.rs`:
```rust
tray.set_title(&format!("⏱️ {}", timer_text))  // Different emoji
tray.set_title(&format!("{}", timer_text))     // No emoji
tray.set_title(&format!("Focus: {}", timer_text))  // Text prefix
```

### Change Timer Format

Edit `src/index.html`:
```javascript
function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;

    // Current format: "15:00"
    return `${mins}:${secs.toString().padStart(2, '0')}`;

    // Alternative: "15m 00s"
    // return `${mins}m ${secs.toString().padStart(2, '0')}s`;

    // Alternative: "15.0" (minutes with decimal)
    // return (seconds / 60).toFixed(1);
}
```

### Add Notification Sound

Edit `src/index.html` in `triggerCheckIn()`:
```javascript
async function triggerCheckIn() {
    pauseSession();

    // Add a beep!
    const audio = new Audio('data:audio/wav;base64,...');  // System beep
    audio.play();

    // Rest of function...
}
```

## Troubleshooting

### Menu Bar Timer Not Updating

**Check Console:**
```javascript
// In updateDisplay(), the timer update happens here:
await invoke('update_tray_timer', { timerText: trayText });
```

If you see errors in console, the backend command might have failed.

**Verify Backend:**
```rust
// Ensure this command is registered in main.rs:
.invoke_handler(tauri::generate_handler![
    get_settings,
    save_settings,
    switch_desktop,
    open_settings,
    update_tray_timer,  // ← Must be here!
    log_check_in
])
```

### Menu Bar Icon Not Appearing

**Check Tauri Config:**
```json
{
  "systemTray": {
    "iconPath": "icons/tray_icon.png",  // ← File must exist
    "iconAsTemplate": true,
    "menuOnLeftClick": false
  }
}
```

**Verify Icon Exists:**
```bash
ls -la src-tauri/icons/
# Should see tray_icon.png
```

### Timer Shows Wrong Time

**Check Initial Values:**
```javascript
// In resetSession():
checkInTimeRemaining = settings.checkInInterval * 60;

// If settings.checkInInterval = 15
// Then checkInTimeRemaining = 900 seconds (15:00)
```

## macOS Menu Bar Tips

### Positioning
- Menu bar icons appear right-to-left
- Drag to reposition (macOS allows this)
- Keep it near the clock for easy viewing

### Dark Mode
The icon automatically adapts to dark/light mode because:
```json
"iconAsTemplate": true  // Uses system theme
```

### Menu Bar Spacing
If your menu bar is crowded, you can:
1. Hide other menu bar apps
2. Use Bartender app to organize
3. Use Hidden Bar (free alternative)

## Performance

### CPU Usage
- Timer updates every second
- Minimal CPU impact (~0.1%)
- No battery drain concern

### Memory
- Entire app: ~50MB RAM
- Menu bar updates: negligible

## Comparison

### Without Menu Bar Timer:
```
❌ Must open window to check time
❌ Easy to forget about session
❌ Loses focus switching to app
```

### With Menu Bar Timer:
```
✅ Always visible countdown
✅ No context switching needed
✅ Work stays in flow state
✅ Glance = instant awareness
```

## Advanced: System Notifications

If you want a notification instead of (or in addition to) the check-in screen:

**Install Tauri Notification Plugin:**
```bash
cargo add tauri-plugin-notification
```

**In main.rs:**
```rust
use tauri::api::notification::Notification;

#[tauri::command]
fn trigger_check_in_notification(app: AppHandle) {
    Notification::new(&app.config().tauri.bundle.identifier)
        .title("Time to Check In!")
        .body("15 minutes have passed. What are you working on?")
        .show()
        .unwrap();
}
```

---

## Summary

✅ **Everything is already configured!**

When you run the app:
1. It starts hidden (no window)
2. Menu bar shows `🧠 15:00`
3. When you start a session, it counts down every second
4. You can work with the window hidden
5. Timer is always visible in menu bar
6. At 0:00, check-in triggers

**No additional setup needed - just run it!**

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run the app
npm run dev

# The menu bar timer will start working immediately!
```
