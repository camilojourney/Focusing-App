# Professional Debugging Guide

## Debugging the System Tray Issue

### Problem
- **Dev mode**: System tray works perfectly ✅
- **Production build**: System tray icon doesn't appear ❌

### Root Cause Analysis

The issue was using `LSUIElement` in Info.plist, which can prevent the system tray from initializing properly in some cases.

### Professional Solution

Instead of modifying Info.plist, use Tauri's `set_activation_policy` API:

```rust
.setup(|app| {
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    Ok(())
})
```

**Benefits:**
- ✅ More reliable than LSUIElement
- ✅ Programmatic control
- ✅ Works consistently in debug and release builds
- ✅ Standard Tauri approach

### Debugging Tools Professionals Use

#### 1. Console.app (macOS)
```bash
# Open macOS Console
open /Applications/Utilities/Console.app
```
Filter for your app name and watch for errors/warnings.

#### 2. System Logs
```bash
# Watch logs in real-time
log stream --predicate 'process == "Hyper Awareness"' --level info

# Show recent logs
log show --predicate 'process == "Hyper Awareness"' --last 5m --info
```

#### 3. Rust Logging
Add to `Cargo.toml`:
```toml
[dependencies]
env_logger = "0.10"
log = "0.4"
```

Add to `main.rs`:
```rust
fn main() {
    env_logger::init();
    log::info!("Application starting...");
    log::debug!("System tray initialized");
    // ...
}
```

Run with logging:
```bash
RUST_LOG=debug npm run dev
```

#### 4. Check Binary Contents
```bash
# See what's embedded in the binary
strings "target/release/bundle/macos/Hyper Awareness.app/Contents/MacOS/Hyper Awareness" | less

# Check for PNG data
strings "target/release/bundle/macos/Hyper Awareness.app/Contents/MacOS/Hyper Awareness" | grep -i png
```

#### 5. Compare Debug vs Release
```bash
# File sizes
ls -lh target/debug/hyper-awareness
ls -lh target/release/bundle/macos/Hyper\ Awareness.app/Contents/MacOS/Hyper\ Awareness

# Dependencies
otool -L target/debug/hyper-awareness
otool -L target/release/bundle/macos/Hyper\ Awareness.app/Contents/MacOS/Hyper\ Awareness
```

#### 6. Code Signing Issues
```bash
# Check code signature
codesign -dvvv "/Applications/Hyper Awareness.app"

# Verify
codesign --verify --verbose "/Applications/Hyper Awareness.app"

# Check entitlements
codesign -d --entitlements - "/Applications/Hyper Awareness.app"
```

### Systematic Debugging Process

1. **Reproduce the issue** - Confirm it happens consistently
2. **Check logs** - Use Console.app and `log stream`
3. **Compare working vs broken** - Debug vs Release builds
4. **Isolate the problem** - Comment out code to find the issue
5. **Test hypothesis** - Make targeted changes
6. **Document solution** - Write it down for future reference

### Common Tauri Issues

| Issue | Symptom | Solution |
|-------|---------|----------|
| Icon not loading | Gray icon or no icon | Check icon file path and format |
| Tray not appearing | No menu bar icon | Use `ActivationPolicy::Accessory` |
| Window not showing | App runs but no UI | Check `visible: false` in config |
| IPC errors | Commands don't work | Check `withGlobalTauri: true` |
| Build fails | Compilation errors | Check Rust version and dependencies |

### Best Practices

1. **Always test production builds** before releasing
2. **Use proper logging** in production
3. **Check Console.app** for macOS-specific issues
4. **Keep dev and prod configs similar** to avoid surprises
5. **Document workarounds** for team knowledge

## Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Tauri Discord](https://discord.com/invite/tauri) - Very helpful community
- [GitHub Issues](https://github.com/tauri-apps/tauri/issues) - Check existing issues
- macOS Console.app - Built-in debugging tool
