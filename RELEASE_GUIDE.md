# 🚀 Release Guide - Hyper Awareness

## Quick Start - Your First Release

You have everything ready! Just need to push:

```bash
# Push all commits (4 unpushed commits)
git push

# Push the v1.0.0 tag
git push --tags
```

**What happens next:**
1. GitHub Actions starts building (takes ~5-10 minutes)
2. Builds macOS Universal binary (Intel + Apple Silicon)
3. Builds Windows installers (EXE + MSI)
4. Creates Release at: `github.com/camilojourney/Focusing-App/releases/v1.0.0`
5. Users can download!

**Download link to share:**
```
https://github.com/camilojourney/Focusing-App/releases/latest
```

---

## Files Ready Right Now

If you want to share the macOS version immediately (before GitHub Actions):

**Location:** `src-tauri/target/release/bundle/dmg/`
**File:** `Hyper Awareness_1.0.0_aarch64.dmg` (6.0 MB)
**For:** Apple Silicon Macs only

---

## Future Releases - The Pattern

### 1. Make Changes
```bash
# Edit your code
vim src-tauri/src/main.rs

# Test it
pnpm tauri dev
```

### 2. Commit
```bash
git add .
git commit -m "fix: Description of what you fixed"
```

### 3. Create Version Tag

**For bug fixes:**
```bash
git tag v1.0.1 -m "Bug fix: Tray icon visibility improved"
```

**For new features:**
```bash
git tag v1.1.0 -m "New feature: Dark mode support"
```

**For major changes:**
```bash
git tag v2.0.0 -m "Major: Complete redesign"
```

### 4. Push Everything
```bash
git push
git push --tags
```

### 5. Wait & Monitor
- Go to: `github.com/camilojourney/Focusing-App/actions`
- Watch the build progress (green = success, red = failed)
- Takes ~5-10 minutes

### 6. Share!
Tell users: "New version available at github.com/camilojourney/Focusing-App/releases/latest"

---

## What Gets Built Automatically

### macOS
- ✅ Universal DMG (Intel + Apple Silicon)
- ✅ Size: ~6-8 MB
- ✅ Works on macOS 10.13+

### Windows  
- ✅ NSIS Installer (.exe) - Recommended for users
- ✅ MSI Package - Recommended for enterprises
- ✅ Works on Windows 10/11

---

## Version Numbering Guide

Use [Semantic Versioning](https://semver.org):

```
v MAJOR . MINOR . PATCH
  │       │       │
  │       │       └─ Bug fixes (v1.0.1)
  │       └───────── New features (v1.1.0)  
  └───────────────── Breaking changes (v2.0.0)
```

**Examples:**
- Fixed tray icon bug → v1.0.0 → v1.0.1
- Added export feature → v1.0.1 → v1.1.0
- Complete UI redesign → v1.1.0 → v2.0.0

---

## Monitoring Builds

### During Build:
1. Go to Actions tab
2. Click the running workflow
3. See real-time logs
4. Download artifacts when done

### After Release:
1. Go to Releases tab
2. See download counts
3. View release notes
4. Test download links

---

## Troubleshooting

### Build Fails

**Check the logs:**
1. Actions tab → Failed workflow → Click it
2. Read error messages
3. Fix code → commit → push tag again

**Common issues:**
- Windows build fails: Check file paths (use `/` not `\`)
- macOS build fails: Check Rust targets installed
- Both fail: Syntax error in code

### Release Not Created

- Tag must start with `v` (e.g., `v1.0.0`)
- Must push the tag: `git push --tags`
- Check Actions tab for errors

### Users Can't Download

- Release might be draft (edit to publish)
- Files didn't upload (check Actions logs)
- GitHub might be slow (wait a few minutes)

---

## Manual Release (Backup)

If GitHub Actions fails, you can create release manually:

```bash
# Build locally
pnpm tauri build

# Go to github.com/your-repo/releases/new
# Upload files from:
# - src-tauri/target/release/bundle/dmg/*.dmg
```

---

## Tips

✅ **Test before tagging** - Always test with `pnpm tauri dev`
✅ **Use good commit messages** - Helps generate release notes
✅ **Keep CHANGELOG** - Document changes between versions
✅ **Beta testing** - Tag as `v1.0.0-beta.1` for pre-releases
✅ **Monitor downloads** - See what versions users prefer

---

## Current Status

- [x] GitHub Actions workflows created
- [x] DMG built locally (6.0 MB)
- [x] Tag v1.0.0 created
- [ ] Push commits (you need to do this)
- [ ] Push tag (you need to do this)
- [ ] First release live!

**Next step:** Run `git push && git push --tags`
