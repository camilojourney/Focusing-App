# macOS Versioning Strategy - Professional Approach

## Question: Do You Need Separate Builds for Each macOS Version?

**Short Answer:** No, you typically build ONE app that works across multiple macOS versions.

## How It Works in Practice

### The Professional Approach

```json
// In tauri.conf.json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "minimumSystemVersion": "10.13"  // High Sierra (2017)
      }
    }
  }
}
```

**This means:**
- ✅ Your app runs on macOS 10.13 (High Sierra) and newer
- ✅ Works on Big Sur, Monterey, Ventura, Sonoma automatically
- ✅ ONE binary for all versions
- ✅ Apple handles backward compatibility

### macOS Version Support Strategy

| Strategy | Use Case | Example |
|----------|----------|---------|
| **Conservative** | Maximum compatibility | `10.13` (2017) - 95%+ users |
| **Balanced** | Modern features, good reach | `11.0` (2020) - 90%+ users |
| **Aggressive** | Latest APIs only | `13.0` (2022) - 75%+ users |

### Real-World Example

```toml
# Cargo.toml for Hyper Awareness
[package]
name = "hyper-awareness"

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.24"  # macOS-specific APIs

[build-dependencies]
tauri-build = "1.5"
```

```rust
// Conditional compilation for different macOS versions
#[cfg(target_os = "macos")]
fn setup_macos_features(app: &mut App) {
    // This code only runs on macOS

    // Use feature that requires macOS 11.0+
    #[cfg(macos_version = "11.0")]
    {
        // Use Big Sur+ features
    }

    // Fallback for older versions
    #[cfg(not(macos_version = "11.0"))]
    {
        // Use compatible alternative
    }
}
```

## Architecture Support

### Universal Binary (Recommended)

Build ONE app that works on both Intel and Apple Silicon:

```bash
# Tauri automatically builds Universal binaries by default
npm run build
```

This creates:
- ✅ Runs on Intel Macs (x86_64)
- ✅ Runs on Apple Silicon (arm64/aarch64)
- ✅ macOS automatically picks the right architecture

### Architecture-Specific (Rare)

Only needed for very specific cases:

```bash
# Build for specific architecture
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

cargo build --target x86_64-apple-darwin    # Intel only
cargo build --target aarch64-apple-darwin   # Apple Silicon only
```

## Feature Detection Pattern

Professional apps detect features at runtime, not build time:

```rust
use std::process::Command;

fn macos_version() -> Option<(u32, u32)> {
    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()?;

    let version = String::from_utf8_lossy(&output.stdout);
    // Parse version like "13.5.2" -> (13, 5)
    // ...
}

fn main() {
    if let Some((major, minor)) = macos_version() {
        if major >= 13 {
            // Use Ventura+ features
            enable_stage_manager_integration();
        } else if major >= 11 {
            // Use Big Sur+ features
            enable_menu_bar_extras();
        } else {
            // Use older API
            enable_legacy_status_bar();
        }
    }
}
```

## Distribution Strategy

### App Store

```json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "minimumSystemVersion": "10.15",  // Catalina minimum for App Store
        "entitlements": "entitlements.plist",
        "provisioningProfile": "profile.provisionprofile"
      }
    }
  }
}
```

**Requirements:**
- Must notarize with Apple
- Code signing required
- Sandbox restrictions apply

### Direct Distribution (Our Case)

```json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "minimumSystemVersion": "10.13",  // Support older versions
        "exceptionDomain": ""  // Allow network access
      }
    }
  }
}
```

**Advantages:**
- ✅ More flexibility
- ✅ Support older macOS versions
- ✅ No App Store review
- ✅ Direct updates possible

## Backward Compatibility Checklist

### APIs to Avoid (Need Newer macOS)

| API | Minimum Version | Alternative |
|-----|-----------------|-------------|
| Stage Manager | 13.0+ | Standard window management |
| Menu Bar Extras | 13.0+ | System Tray (works everywhere) |
| SwiftUI in Rust | 11.0+ | Use Cocoa/Objective-C |
| Universal Control | 12.3+ | Not applicable for most apps |

### Safe APIs (Work Everywhere)

- ✅ System Tray (NSStatusBar) - 10.0+
- ✅ Notifications (NSUserNotification) - 10.8+
- ✅ Window Management - 10.0+
- ✅ File System - 10.0+
- ✅ Network - 10.0+

## Testing Strategy

### Professional Testing Approach

```bash
# 1. Test on your development machine (e.g., Sonoma)
npm run dev

# 2. Build release version
npm run build

# 3. Test on Virtual Machines (optional but recommended)
# - Download older macOS from Apple
# - Run in Parallels/VMware
# - Test on macOS 11, 12, 13, 14

# 4. Beta testers with different macOS versions
# - Distribute .dmg to friends/colleagues
# - Get feedback on compatibility
```

### Quick Compatibility Check

```rust
// Add version check on startup
fn check_compatibility() -> Result<(), String> {
    let (major, _) = macos_version()
        .ok_or("Could not detect macOS version")?;

    if major < 10 {
        return Err(format!(
            "Hyper Awareness requires macOS 10.13 or later. \
             You have macOS {}.x", major
        ));
    }

    Ok(())
}
```

## Practical Example: Hyper Awareness

### Our Current Setup

```json
// tauri.conf.json
{
  "bundle": {
    "macOS": {
      "minimumSystemVersion": "10.13"
    }
  }
}
```

**This means:**
- ✅ Works on macOS 10.13 (High Sierra, 2017) through macOS 14 (Sonoma, 2023)
- ✅ ~95% of Mac users can run it
- ✅ ONE build for all versions
- ✅ No special version detection needed

### When to Version-Specific Build

**Almost Never!** Only if:
1. You need cutting-edge APIs ONLY in macOS 14+
2. You're building system-level tools
3. You need specific kernel features

**For a productivity app like Hyper Awareness:** ONE build is perfect.

## Summary

### ✅ Do This (Recommended)

1. Set `minimumSystemVersion` to oldest version you want to support
2. Build ONE universal binary
3. Let macOS handle compatibility
4. Use runtime feature detection if needed
5. Test on oldest supported version

### ❌ Don't Do This

1. ~~Build separate versions for each macOS release~~
2. ~~Use version-specific APIs without fallbacks~~
3. ~~Ignore backward compatibility~~
4. ~~Hard-code version checks~~

## Resources

- [Apple Platform Deployment](https://developer.apple.com/support/deployment/)
- [Tauri macOS Guide](https://tauri.app/v1/guides/building/macos)
- [macOS Version Stats](https://gs.statcounter.com/macos-version-market-share)
- Check user's macOS version: `sw_vers -productVersion`
