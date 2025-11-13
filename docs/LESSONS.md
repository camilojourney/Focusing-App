# Technical Lessons Learned

A comprehensive guide to the technical concepts, tools, and best practices used in building the Focus Time app with Tauri.

---

## 1. What is Tauri?

**Tauri** is a framework for building **native desktop applications** using web technologies (HTML, CSS, JavaScript) with a Rust backend.

### Key Advantages:
- **Small Bundle Size**: Apps are ~600KB vs Electron's ~120MB
- **Performance**: Rust backend is fast and memory-efficient
- **Security**: Strong type system, no Node.js vulnerabilities
- **Cross-Platform**: Build for macOS, Windows, and Linux from one codebase
- **Native Features**: Direct access to OS APIs (file system, notifications, system tray)

### Tauri vs Electron:
| Feature | Tauri | Electron |
|---------|-------|----------|
| Backend | Rust | Node.js |
| Frontend Renderer | OS WebView | Chromium (bundled) |
| Bundle Size | ~600KB - 3MB | ~120MB+ |
| Memory Usage | Lower | Higher |
| Startup Time | Faster | Slower |

---

## 2. Development vs Production Builds

### Development Mode (`pnpm run dev`)

**Purpose**: Fast iteration during development

**What Happens:**
1. Tauri CLI starts a dev server
2. Frontend code is served with hot-reload (changes update instantly)
3. Rust backend compiles in **debug mode** (faster compile, slower runtime)
4. File watching enabled - auto-recompiles on code changes
5. Source maps included for debugging
6. No optimization, full debug symbols

**Command:**
```bash
pnpm run dev
# Internally runs: tauri dev
```

**When to Use:**
- Writing code
- Testing features
- Debugging issues
- Rapid prototyping

### Production Build (`pnpm run build`)

**Purpose**: Optimized app for end users

**What Happens:**
1. Frontend code is minified and optimized
2. Rust backend compiles in **release mode** (slower compile, faster runtime)
3. Dead code elimination (tree-shaking)
4. Assets are bundled into the app
5. Creates platform-specific installer/bundle:
   - macOS: `.app` bundle, `.dmg` installer
   - Windows: `.exe`, `.msi`
   - Linux: `.deb`, `.AppImage`

**Command:**
```bash
pnpm run build
# Internally runs: tauri build
```

**Output Location:**
```
src-tauri/target/release/bundle/
├── macos/
│   └── Hyper Awareness.app    # Standalone app
├── dmg/
│   └── Hyper Awareness.dmg    # Installer
```

**When to Use:**
- Final testing before release
- Distributing to users
- Performance testing (release mode is much faster)

---

## 3. Project Architecture

### Directory Structure

```
Focusing-App/
├── src/                       # Frontend (HTML/CSS/JS)
│   ├── index.html            # Main UI
│   └── settings.html         # Settings UI
│
├── src-tauri/                # Backend (Rust)
│   ├── src/
│   │   ├── main.rs           # Entry point, Tauri commands
│   │   └── calendar.rs       # Calendar integration
│   │
│   ├── icons/                # App icons
│   │   └── 18x18.png         # Menu bar icon
│   │
│   ├── Cargo.toml            # Rust dependencies
│   ├── tauri.conf.json       # Tauri configuration
│   └── entitlements.plist    # macOS permissions
│
├── package.json              # Frontend dependencies, scripts
└── node_modules/             # npm packages
```

### Frontend (src/)

**Technologies:**
- Vanilla JavaScript (no frameworks)
- HTML5 + CSS3
- Tauri API for backend communication

**Responsibilities:**
- UI rendering and updates
- Timer logic (session, check-in, countdown)
- User input handling
- Calling Rust commands via Tauri IPC

**Example: Calling Rust from JavaScript**
```javascript
// Frontend calls Rust function
const settings = await window.__TAURI__.invoke('get_settings');
console.log(settings); // { session_duration: 720, ... }

// With parameters
await window.__TAURI__.invoke('log_check_in', {
  log_line: JSON.stringify(data)
});
```

### Backend (src-tauri/src/)

**Technologies:**
- Rust (systems programming language)
- Tauri framework
- Platform-specific APIs (Cocoa for macOS)

**Responsibilities:**
- System tray (menu bar icon)
- File I/O (settings, logs)
- OS integration (calendar, desktop switching)
- Window management
- Exposing commands to frontend

**Example: Rust Command**
```rust
#[tauri::command]
fn get_settings(app: AppHandle) -> Result<Settings, String> {
    // Load settings from disk
    load_settings(&app)
}
```

### Communication Flow (IPC)

```
Frontend (JavaScript)
    │
    │ invoke('get_settings')
    ▼
Tauri IPC Bridge
    │
    │ Serialize/Deserialize
    ▼
Backend (Rust)
    │
    │ Execute command
    ▼
Return Result → Frontend
```

**Key Concept**: Inter-Process Communication (IPC)
- Frontend and backend run in separate processes
- Communication via Tauri's IPC system
- Data serialized to JSON automatically
- Type-safe with Rust's type system

---

## 4. Key Configuration Files

### package.json

**Purpose**: Frontend configuration and scripts

**Key Sections:**
```json
{
  "scripts": {
    "dev": "tauri dev",      // Start dev server
    "build": "tauri build"   // Build production
  },
  "devDependencies": {
    "@tauri-apps/cli": "^1.8.3"  // Tauri CLI
  }
}
```

### src-tauri/Cargo.toml

**Purpose**: Rust dependencies and metadata

**Key Dependencies:**
```toml
[dependencies]
tauri = { version = "1.8.3", features = ["system-tray"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
objc = "0.2"  # For macOS APIs
cocoa = "0.24"  # For macOS Calendar
```

### src-tauri/tauri.conf.json

**Purpose**: Tauri app configuration

**Critical Settings:**
```json
{
  "package": {
    "productName": "Hyper Awareness",  // App name
    "version": "1.0.0"
  },
  "tauri": {
    "bundle": {
      "identifier": "com.focustime.app",  // Unique ID
      "resources": ["icons/18x18.png"]    // Bundle files
    },
    "systemTray": {
      "iconPath": "icons/18x18.png",
      "iconAsTemplate": true,  // Adaptive icon (macOS)
      "menuOnLeftClick": false
    },
    "windows": [...]  // Window configuration
  }
}
```

---

## 5. System Tray / Menu Bar Integration

### macOS Template Images

**Concept**: macOS uses "template images" for menu bar icons

**Requirements:**
1. **Pure black** (`#000000`) pixels for visible parts
2. **Transparent** (alpha = 0) for invisible parts
3. **No anti-aliasing** (no gray pixels)
4. **Small size**: 18x18 pixels recommended

**Why?**
- macOS automatically inverts colors based on theme
- White icon in dark mode
- Black icon in light mode

**Configuration:**
```json
"systemTray": {
  "iconPath": "icons/18x18.png",
  "iconAsTemplate": true  // CRITICAL for macOS
}
```

### System Tray in Rust

**Creating the Tray:**
```rust
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

let menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("show", "Show Timer"))
    .add_item(CustomMenuItem::new("quit", "Quit"));

let system_tray = SystemTray::new()
    .with_menu(menu)
    .with_tooltip("Hyper Awareness");
```

**Handling Clicks:**
```rust
.on_system_tray_event(|app, event| match event {
    SystemTrayEvent::LeftClick { .. } => {
        // Show window
    }
    SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
            "quit" => std::process::exit(0),
            _ => {}
        }
    }
    _ => {}
})
```

---

## 6. macOS-Specific Features

### Calendar Integration (EventKit)

**Concept**: Access user's calendar to show current events

**Steps:**
1. **Request Permission** (required by Apple)
2. **Access EventKit** via Objective-C bridge
3. **Query Current Events**

**Implementation:**
```rust
// calendar.rs
use objc::*;
use cocoa::base::id;

#[tauri::command]
fn get_current_event() -> Result<Option<String>, String> {
    unsafe {
        // Access EventKit via Objective-C runtime
        let event_store = msg_send![class!(EKEventStore), new];
        // Query events...
    }
}
```

**Permissions (entitlements.plist):**
```xml
<key>com.apple.security.personal-information.calendars</key>
<true/>
```

### Desktop Switching (AppleScript)

**Concept**: Automatically switch to Desktop 1 for clean reflection space

**Implementation:**
```rust
use std::process::Command;

fn switch_to_desktop_1() {
    Command::new("osascript")
        .arg("-e")
        .arg(r#"tell application "System Events" to key code 18 using control down"#)
        .output()
        .expect("Failed to switch desktop");
}
```

### Activation Policy

**Concept**: Control app's Dock icon behavior

**Options:**
- `Regular`: Normal app with Dock icon
- `Accessory`: Menu bar only (no Dock icon) ← **We use this**
- `Prohibited`: Background app

**Implementation:**
```rust
.setup(|app| {
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    Ok(())
})
```

---

## 7. Data Persistence

### Settings Storage

**Format**: JSON
**Location**: OS-specific config directory
- macOS: `~/Library/Application Support/com.focustime.app/`
- Linux: `~/.config/focus-time/`
- Windows: `%APPDATA%\com.focustime.app\`

**Implementation:**
```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Settings {
    session_duration: u32,
    check_in_interval: u32,
}

fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = app_config_dir()?;
    let json = serde_json::to_string_pretty(settings)?;
    fs::write(path, json)?;
    Ok(())
}
```

### Log Storage (JSONL)

**Format**: JSON Lines (one JSON object per line)
**Why?**: Easy to append, stream, and analyze

**Example:**
```jsonl
{"timestamp":"2025-10-25T10:30:00Z","status":"On Task"}
{"timestamp":"2025-10-25T10:45:00Z","status":"Social Media"}
```

**Benefits:**
- No need to parse entire file
- Append-only (fast writes)
- Easy to analyze with Python/R
- Works with streaming tools (grep, awk)

---

## 8. Rust Concepts for JavaScript Developers

### Ownership & Borrowing

**Problem Rust Solves**: Memory management without garbage collection

**Rules:**
1. Each value has one owner
2. When owner goes out of scope, value is dropped
3. Can borrow references (`&`) without taking ownership

**Example:**
```rust
fn process(s: String) {
    // s is owned here
} // s is dropped here

fn main() {
    let text = String::from("hello");
    process(text);
    // text is no longer valid here!
}
```

### Result Type (Error Handling)

**Concept**: No exceptions, explicit error handling

**Type:**
```rust
Result<T, E>  // Either Ok(value) or Err(error)
```

**Usage:**
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Handle result
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

### Option Type (Null Safety)

**Concept**: No null pointer exceptions

**Type:**
```rust
Option<T>  // Either Some(value) or None
```

**Usage:**
```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}
```

### Macros

**Concept**: Code generation at compile time

**Example:**
```rust
// tauri::command macro generates IPC boilerplate
#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Expands to ~50 lines of serialization code
```

---

## 9. Build & Deployment

### Development Workflow

```bash
# 1. Install dependencies (first time)
pnpm install

# 2. Start dev server
pnpm run dev

# 3. Make changes → Auto-reload

# 4. Test in dev mode
```

### Production Release

```bash
# 1. Build production app
pnpm run build

# 2. Output location
src-tauri/target/release/bundle/macos/

# 3. Test production build
open "src-tauri/target/release/bundle/macos/Hyper Awareness.app"

# 4. Create DMG for distribution
# (Automatically created during build)
```

### Code Signing (macOS)

**Why?**: macOS requires signed apps to avoid security warnings

**Steps:**
1. Get Apple Developer account ($99/year)
2. Create signing certificate
3. Configure in `tauri.conf.json`:
```json
"bundle": {
  "macOS": {
    "signingIdentity": "Developer ID Application: Your Name"
  }
}
```

### CI/CD Pipeline (GitHub Actions)

**Concept**: Automated builds on every commit

**Example workflow:**
```yaml
name: Build
on: [push]
jobs:
  build:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      - name: Install dependencies
        run: pnpm install
      - name: Build
        run: pnpm run build
      - name: Upload artifact
        uses: actions/upload-artifact@v2
```

---

## 10. Common Pitfalls & Solutions

### Problem: Icon Not Showing in Menu Bar

**Cause**: Icon not set as template, or wrong format

**Solution:**
1. Ensure `iconAsTemplate: true` in config
2. Icon must be pure black + transparent
3. Restart Dock: `killall Dock`

### Problem: Calendar Permission Not Working

**Cause**: Missing entitlements or user denied permission

**Solution:**
1. Add to `entitlements.plist`:
```xml
<key>com.apple.security.personal-information.calendars</key>
<true/>
```
2. Request permission in code
3. User must grant in System Settings

### Problem: Slow Compilation

**Cause**: Rust's compile times can be long

**Solutions:**
1. Use `cargo check` instead of full build during dev
2. Enable incremental compilation (default in dev)
3. Use `sccache` (compile cache)
4. Reduce dependencies

### Problem: Large Bundle Size

**Cause**: Including unnecessary dependencies

**Solutions:**
1. Use feature flags to disable unused features
2. Optimize `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"  # Optimize for size
lto = true       # Link-time optimization
strip = true     # Remove debug symbols
```

---

## 11. Testing Strategies

### Frontend Testing

**Manual Testing:**
- Use browser DevTools (in dev mode)
- Test UI interactions
- Check console for errors

**Automated Testing:**
```javascript
// Example with Jest
test('timer decrements correctly', () => {
  let time = 900;
  time -= 1;
  expect(time).toBe(899);
});
```

### Backend Testing

**Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default() {
        let s = Settings::default();
        assert_eq!(s.session_duration, 720);
    }
}
```

**Integration Tests:**
```rust
// tests/integration_test.rs
#[test]
fn test_save_load_settings() {
    let settings = Settings::default();
    save_settings(&settings).unwrap();
    let loaded = load_settings().unwrap();
    assert_eq!(settings.session_duration, loaded.session_duration);
}
```

### End-to-End Testing

**Tools:**
- Playwright (web testing)
- Tauri WebDriver (automation)

---

## 12. Performance Optimization

### Frontend

1. **Minimize DOM Manipulation**: Batch updates
2. **Use RequestAnimationFrame**: Smooth animations
3. **Debounce Events**: Avoid excessive function calls
4. **Lazy Load**: Load resources only when needed

### Backend

1. **Async I/O**: Use `tokio` for non-blocking operations
2. **Caching**: Store frequently accessed data
3. **Efficient Data Structures**: Use `HashMap` for lookups
4. **Profile**: Use `cargo flamegraph` to find bottlenecks

---

## 13. Security Best Practices

### Input Validation

**Always validate user input:**
```rust
#[tauri::command]
fn save_settings(settings: Settings) -> Result<(), String> {
    if settings.session_duration < 1 || settings.session_duration > 1440 {
        return Err("Invalid duration".to_string());
    }
    // Proceed...
}
```

### Secure IPC

**Tauri's allowlist system:**
```json
"tauri": {
  "allowlist": {
    "all": false,  // Deny all by default
    "fs": {
      "scope": ["$APPDATA/*"]  // Only allow app directory
    }
  }
}
```

### Secrets Management

**Never hardcode secrets:**
```rust
// ❌ Bad
const API_KEY = "secret123";

// ✅ Good
let api_key = std::env::var("API_KEY")?;
```

---

## 14. Debugging Tools

### Frontend

- **Browser DevTools**: `Cmd+Option+I` (in dev mode)
- **Console Logging**: `console.log()`
- **Tauri Console**: `window.__TAURI_INTERNALS__`

### Backend

- **Print Debugging**: `println!()`, `dbg!()`
- **Logging**: Use `log` crate
```rust
use log::{info, error};
info!("User clicked button");
error!("Failed to save: {}", err);
```
- **LLDB Debugger**: `rust-lldb target/debug/app`

### Build Issues

```bash
# Verbose output
cargo build --verbose

# Check without building
cargo check

# Clean and rebuild
cargo clean && cargo build
```

---

## 15. Resources & Next Steps

### Official Documentation
- **Tauri**: https://tauri.app/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Cargo Book**: https://doc.rust-lang.org/cargo/

### Community
- **Tauri Discord**: https://discord.gg/tauri
- **Rust Forum**: https://users.rust-lang.org/

### Learning Path

**Beginner:**
1. Learn Rust basics (ownership, types, errors)
2. Build simple Tauri app (Hello World)
3. Understand IPC (frontend ↔ backend)

**Intermediate:**
4. Add system tray
5. Implement file I/O
6. Handle window management

**Advanced:**
7. Platform-specific APIs (macOS, Windows)
8. Custom protocols
9. Plugin development

---

## 16. Key Takeaways

### For Explaining to Technical People:

**"What is this project?"**
> A native macOS menu bar app built with Tauri. Frontend uses vanilla JavaScript, backend is Rust. It's a focus timer with periodic check-ins, calendar integration, and local data logging.

**"Why Tauri?"**
> Small bundle size (~3MB vs Electron's 120MB), better performance with Rust, native system integration, and strong security model.

**"How does dev/prod work?"**
> Dev mode: hot-reload, debug builds, fast iteration. Production: optimized release builds, creates `.app` bundle for distribution.

**"What's special about macOS integration?"**
> Uses EventKit for calendar access, template images for adaptive menu bar icon, AppleScript for desktop switching, and proper entitlements for permissions.

**"How is data stored?"**
> Settings in JSON, logs in JSONL format, all stored in OS-specific config directories. No cloud, 100% local for privacy.

**"What was the hardest part?"**
> Menu bar icon initially didn't work because it wasn't a proper template image (needs pure black on transparent, no anti-aliasing). Fixed by setting `iconAsTemplate: true` and ensuring proper icon format.

---

**Remember**: Tauri is about building fast, secure, native desktop apps using web technologies for the UI and Rust for the backend. It's the modern alternative to Electron.
