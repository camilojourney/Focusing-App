# macOS 26.0.1 System Tray Issue - Workaround Guide

## The Problem

**Your System:** macOS 26.0.1 (unreleased beta version)

**Issue:** System tray icon doesn't appear in production builds on macOS 14+ (Sonoma), 15+ (Sequoia), and 26+ (unreleased)

### Why It Happens

This is a **known Tauri framework bug** affecting newer macOS versions:

- **GitHub Issues:** #12060, #9480, #10912
- **Affected Versions:** macOS 14.4.1+ (Sonoma), macOS 15+ (Sequoia), macOS 26+
- **Status:** Active bug, no fix yet from Tauri team

### Symptoms

✅ **Dev mode works perfectly** (`npm run dev`)
❌ **Production build fails** (`npm run build`)

- Tray icon doesn't appear
- App runs but no menu bar presence
- Click events don't work on macOS 15+
- Ghost/duplicate icons on some systems

## What We Tried

### Attempt 1: LSUIElement in Info.plist
**Result:** ❌ Failed - Made it worse, app became completely invisible

### Attempt 2: ActivationPolicy::Accessory
**Result:** ❌ Failed - Professional approach but doesn't fix macOS 26 bug

### Attempt 3: Toggle iconAsTemplate
**Result:** ⚠️ Testing - Changed from `true` to `false`

### Attempt 4: Smaller Icon Size
**Result:** ⚠️ Testing - Changed from 32x32 to 18x18 (optimal for macOS menu bar)

## Current Configuration

```json
// tauri.conf.json
{
  "systemTray": {
    "iconPath": "icons/tray-icon.png",
    "iconAsTemplate": false  // Changed from true
  }
}
```

```rust
// main.rs
let system_tray = SystemTray::new()
    .with_icon(Icon::Raw(include_bytes!("../icons/18x18.png").to_vec()))  // Changed from 32x32
    .with_menu(tray_menu);
```

## Recommended Workaround

### Option 1: Use Dev Mode (Recommended for Now)

```bash
# This works perfectly!
npm run dev
```

**Pros:**
- ✅ Tray icon appears
- ✅ All buttons work
- ✅ Full functionality
- ✅ Hot reload for development

**Cons:**
- ❌ Not a "production" app
- ❌ Needs terminal running
- ❌ Can't distribute to others

### Option 2: Test on Older macOS

If possible, test on:
- **macOS 13 (Ventura)** - Known to work
- **macOS 12 (Monterey)** - Should work
- **macOS 11 (Big Sur)** - Should work

These versions don't have the Tauri tray bug.

### Option 3: Wait for Tauri Fix

Monitor these GitHub issues:
- https://github.com/tauri-apps/tauri/issues/12060
- https://github.com/tauri-apps/tauri/issues/9480
- https://github.com/tauri-apps/tauri/issues/10912

Join Tauri Discord for updates: https://discord.com/invite/tauri

### Option 4: Alternative Framework (Last Resort)

If Tauri continues to fail:
- **Electron** - More mature, larger bundle size
- **Native Swift/SwiftUI** - Best performance, macOS only
- **Flutter** - Cross-platform, good performance

## Testing Checklist

Try production build with these combinations:

- [ ] `iconAsTemplate: false` + 18x18 icon
- [ ] `iconAsTemplate: true` + 18x18 icon
- [ ] `iconAsTemplate: false` + 16x16 icon
- [ ] Different icon formats (try black/white template icon)
- [ ] Remove `ActivationPolicy::Accessory` temporarily
- [ ] Test on external monitor vs built-in display
- [ ] Test with/without dark mode

## How to Test

```bash
# 1. Kill all processes
killall "Hyper Awareness"
pkill -9 -f "tauri dev"

# 2. Build production version
npm run build

# 3. Install to Applications
rm -rf "/Applications/Hyper Awareness.app"
cp -R "src-tauri/target/release/bundle/macos/Hyper Awareness.app" /Applications/

# 4. Launch and check menu bar
open "/Applications/Hyper Awareness.app"

# Look for icon in menu bar (top-right)
# If not visible, bug still exists
```

## For End Users

Until this is fixed, distribute the app with these instructions:

### Installation

1. Download `Hyper Awareness.app`
2. Move to Applications folder
3. **First launch:** Right-click → Open (bypass Gatekeeper)

### If Icon Doesn't Appear

```bash
# Run in dev mode instead
git clone https://github.com/yourusername/Focusing-App.git
cd Focusing-App
npm install
npm run dev
```

## Technical Details

### Icon Sizes

macOS menu bar icon guidelines:
- **16x16** - Standard resolution
- **18x18** - Slightly larger, better visibility
- **32x32** - Too large for menu bar, causes issues
- **Template mode** - Black/transparent, adapts to light/dark mode

### What Tauri v1 vs v2 Does Differently

- **Tauri v1** (our version): Uses `tao` windowing library
- **Tauri v2**: Uses updated `tao` with better macOS support
- **The bug:** Affects both, but v2 has more active development

### System Requirements

```json
{
  "minimumSystemVersion": "10.13"  // Our setting
}
```

This means we SHOULD support macOS 10.13+, but the tray bug affects 14+.

## When Will This Be Fixed?

**Unknown.** The Tauri team is aware but:
- It's a complex macOS-specific issue
- Affects multiple macOS versions differently
- May require Apple to fix their APIs
- Could be resolved in Tauri v2 stable release

## Conclusion

**For now: Use `npm run dev`** for development and personal use.

**For distribution:** Wait for Tauri fix or consider alternative framework.

The app itself works perfectly - all buttons, all functionality. It's purely a system tray visibility bug in production builds on newer macOS versions.

---

**Last Updated:** 2025-10-20
**macOS Version:** 26.0.1
**Tauri Version:** 1.8.3
**Status:** Known bug, workaround available (dev mode)
