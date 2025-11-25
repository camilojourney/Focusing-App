# Release Guide for Hyper Awareness

This guide explains how to create releases and distribute the app to users.

## 📦 Quick Release (Using GitHub Actions)

### 1. Prepare the Release

```bash
# Make sure all changes are committed
git add .
git commit -m "Your changes"

# Create a version tag
git tag -a v1.0.1 -m "Release v1.0.1: Bug fixes and improvements"

# Push everything
git push
git push --tags
```

### 2. GitHub Actions Automatically:

✅ Builds for macOS (Universal - Intel + Apple Silicon)
✅ Builds for Windows (MSI + NSIS installers)
✅ Creates GitHub Release
✅ Uploads all installers

**Time:** ~10-15 minutes

### 3. Monitor Progress

Go to: https://github.com/camilojourney/Focusing-App/actions

Watch the "Build and Release" workflow run.

### 4. Share Download Link

```
https://github.com/camilojourney/Focusing-App/releases/latest
```

Users can download:
- **macOS**: `Hyper-Awareness_VERSION_universal.dmg`
- **Windows**: `Hyper-Awareness_VERSION_x64-setup.exe`

---

## 🛠️ Manual Release (Backup Method)

### For macOS (Your Current Machine)

```bash
# Build the app
pnpm tauri build

# Sign the app (prevents "damaged" warning)
codesign --force --deep --sign - \
  "src-tauri/target/release/bundle/macos/Hyper Awareness.app"

# Installers are created at:
# - DMG: src-tauri/target/release/bundle/dmg/*.dmg
# - App: src-tauri/target/release/bundle/macos/*.app
```

### For Windows (Requires Windows Machine)

```bash
# On Windows machine:
git clone https://github.com/camilojourney/Focusing-App
cd Focusing-App
pnpm install
pnpm tauri build

# Installers created at:
# - MSI: src-tauri/target/release/bundle/msi/*.msi
# - EXE: src-tauri/target/release/bundle/nsis/*.exe
```

### Create GitHub Release Manually

```bash
# Using GitHub CLI
gh release create v1.0.1 \
  "src-tauri/target/release/bundle/dmg/*.dmg" \
  --title "Hyper Awareness v1.0.1" \
  --notes "Release notes here"
```

Or via GitHub web UI:
1. Go to: https://github.com/camilojourney/Focusing-App/releases/new
2. Choose tag: `v1.0.1`
3. Upload installers
4. Publish

---

## 📋 Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **v1.0.0** → First stable release
- **v1.0.1** → Bug fixes
- **v1.1.0** → New features (backward compatible)
- **v2.0.0** → Breaking changes

---

## ✅ Pre-Release Checklist

Before creating a release:

- [ ] Test the app locally
- [ ] Update version in `src-tauri/tauri.conf.json`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Update CHANGELOG.md (if you have one)
- [ ] Test builds work (`pnpm tauri build`)
- [ ] All tests pass
- [ ] No console errors

---

## 📝 Release Notes Template

```markdown
# 🎉 Hyper Awareness vX.Y.Z

## ✨ New Features
- Feature 1
- Feature 2

## 🐛 Bug Fixes
- Fix 1
- Fix 2

## 🔧 Improvements
- Improvement 1
- Improvement 2

## 📦 Downloads

**macOS (10.13+):**
- Universal (Intel + Apple Silicon): `Hyper-Awareness_X.Y.Z_universal.dmg`

**Windows (10+):**
- Installer: `Hyper-Awareness_X.Y.Z_x64-setup.exe`
- MSI: `Hyper-Awareness_X.Y.Z_x64_en-US.msi`

## 🔒 Security Notes

This release includes unsigned installers:
- **macOS**: Right-click → Open → Open Anyway
- **Windows**: Click "More info" → "Run anyway"

For trusted installers, we need an Apple Developer certificate ($99/year).
```

---

## 🚀 Distribution Channels

### GitHub Releases (Current)

✅ Free
✅ Works for both platforms
✅ Automatic with GitHub Actions
❌ Users see security warnings (unsigned)

**Link format:**
```
https://github.com/camilojourney/Focusing-App/releases/download/vX.Y.Z/filename
```

### Future Options

**macOS:**
- Mac App Store (requires Apple Developer Program - $99/year)
- Homebrew Cask (free, popular with developers)
- Notarization (requires Apple Developer Program)

**Windows:**
- Microsoft Store (one-time $19 fee)
- Chocolatey (free, popular with developers)
- Winget (free, official Windows package manager)

**Cross-platform:**
- Your own website
- Update server (using Tauri updater)

---

## 🔐 Code Signing (Optional but Recommended)

### Why Sign?

✅ Users won't see "unidentified developer" warnings
✅ More professional
✅ Required for auto-updates
✅ Better security

### How to Sign

**macOS:**
1. Join Apple Developer Program ($99/year)
2. Get Developer ID certificate
3. Update build command:
   ```bash
   pnpm tauri build -- --sign "Developer ID Application: Your Name"
   ```

**Windows:**
1. Get code signing certificate (~$200-400/year)
2. Configure in `tauri.conf.json`
3. Build normally

**For GitHub Actions:**
Store signing keys in GitHub Secrets:
- `TAURI_PRIVATE_KEY`
- `TAURI_KEY_PASSWORD`
- `APPLE_CERTIFICATE` (base64 encoded)
- `APPLE_CERTIFICATE_PASSWORD`

---

## 🔄 Auto-Updates (Future Enhancement)

Tauri supports automatic updates. To enable:

1. Set up update server or use GitHub Releases
2. Configure in `tauri.conf.json`:
   ```json
   {
     "updater": {
       "active": true,
       "endpoints": [
         "https://releases.myapp.com/{{target}}/{{current_version}}"
       ],
       "dialog": true,
       "pubkey": "YOUR_PUBLIC_KEY"
     }
   }
   ```
3. Generate update manifests on release

---

## 📊 Release Metrics

Track downloads and usage:

1. **GitHub Insights**: See download counts per release
2. **Analytics** (optional): Add telemetry to track active users
3. **Crash Reports** (optional): Use Sentry or similar

---

## 🐛 Troubleshooting Releases

### Build fails on GitHub Actions

**Check:**
- Workflow file syntax (`.github/workflows/release.yml`)
- Rust/Node versions match your local setup
- Dependencies are correct

**View logs:**
Go to Actions tab → Click failed run → Check logs

### DMG/EXE not created

**macOS:**
- Check `targets` in `tauri.conf.json`
- Ensure icons exist

**Windows:**
- Install WiX Toolset (for MSI)
- Install NSIS (for EXE)

### Users can't install

**macOS:**
- Tell users: Right-click → Open → Open Anyway
- Or: Get app notarized (requires Apple Developer account)

**Windows:**
- Tell users: Click "More info" → "Run anyway"
- Or: Sign with code signing certificate

---

## 📞 Support

If users have issues:

1. Direct them to GitHub Issues
2. Provide installation instructions
3. Explain security warnings
4. Offer alternative download (direct app bundle)

**Installation FAQ**: Add to README.md or create INSTALL.md

---

## 🎯 Next Steps

After your first release:

1. ✅ Tag and push → Automatic build
2. 📝 Write release notes
3. 🔗 Share download link
4. 📊 Monitor downloads/feedback
5. 🐛 Fix reported issues → Next release

**Continuous improvement cycle:**
```
Code → Test → Tag → Release → Feedback → Code ...
```
