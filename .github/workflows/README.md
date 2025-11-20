# GitHub Actions CI/CD Workflows

This folder contains automated workflows that build your app for multiple platforms.

## 📋 Workflows

### 1. `build.yml` - Continuous Integration
**Triggers:** Every push to `main` branch or pull request

**What it does:**
- Builds the app for both macOS and Windows
- Tests that everything compiles correctly
- Saves build artifacts for 7 days (for testing)

**Use case:** Quality checks on every code change

### 2. `release.yml` - Release Automation
**Triggers:** When you create a version tag (like `v1.0.0`)

**What it does:**
- Builds production-ready installers for macOS and Windows
- Creates a universal macOS binary (works on both Intel and Apple Silicon)
- Automatically creates a GitHub Release
- Uploads all installers to the release

**Use case:** Automated releases for users to download

## 🚀 How to Use

### For Regular Development (build.yml)

Just push your code - the workflow runs automatically:

```bash
git add .
git commit -m "Your changes"
git push
```

✅ GitHub will automatically build for macOS and Windows
✅ Check the "Actions" tab to see build status
✅ Download test builds from the workflow artifacts

### For Releases (release.yml)

Create and push a version tag:

```bash
# Create a new version tag
git tag -a v1.0.1 -m "Release v1.0.1: Bug fixes and improvements"

# Push the tag
git push --tags
```

✅ GitHub Actions automatically:
1. Builds for macOS (Universal - Intel + Apple Silicon)
2. Builds for Windows (MSI + NSIS installers)
3. Creates a GitHub Release
4. Uploads all installers

### Manual Trigger

You can also manually trigger the release workflow:

1. Go to your repo → Actions tab
2. Select "Build and Release"
3. Click "Run workflow"
4. Choose the branch

## 📦 What Gets Built

### macOS
- **DMG**: Drag-and-drop installer (recommended)
- **App Bundle**: `.app` file
- **Architecture**: Universal (Intel x86_64 + Apple Silicon arm64)

### Windows
- **MSI**: Windows Installer package (recommended for enterprises)
- **NSIS**: Modern installer with wizard (recommended for users)

## 📥 Where to Find Downloads

After a release is created, users download from:

```
https://github.com/YOUR_USERNAME/Focusing-App/releases/latest
```

Each release will have:
- `Hyper-Awareness_VERSION_universal.dmg` (macOS)
- `Hyper-Awareness_VERSION_x64-setup.exe` (Windows NSIS)
- `Hyper-Awareness_VERSION_x64_en-US.msi` (Windows MSI)

## 🔧 How It Works

### The Process:

1. **Trigger**: Push a tag or code
2. **Virtual Machines**: GitHub spins up:
   - macOS runner (for Mac builds)
   - Windows runner (for Windows builds)
3. **Setup**: Each runner installs:
   - Node.js and pnpm (for frontend)
   - Rust (for Tauri backend)
   - Platform-specific build tools
4. **Build**: Compiles your app
5. **Upload**: Packages and uploads installers
6. **Release**: (For tags) Creates GitHub release with downloads

### Why This Is Awesome:

✅ **No manual builds** - Just push a tag
✅ **Cross-platform** - Build for Windows even from a Mac
✅ **Consistent** - Same build environment every time
✅ **Fast** - GitHub's servers build in parallel
✅ **Free** - For public repos (2000 minutes/month for private)

## 🐛 Troubleshooting

### Build fails on Windows
- Check that all paths use forward slashes or `path.join()`
- Windows is case-sensitive in CI

### Build fails on macOS
- Universal builds require both architectures
- Check Rust toolchain includes both targets

### Release not created
- Make sure tag starts with `v` (e.g., `v1.0.0`)
- Check you pushed the tag: `git push --tags`
- Verify GITHUB_TOKEN has permissions (automatic in public repos)

## 📊 Monitoring Builds

1. Go to your repo on GitHub
2. Click "Actions" tab
3. See all workflow runs
4. Click any run to see detailed logs
5. Download artifacts from completed runs

## 🔒 Security (Optional)

For code signing (makes installers trusted):

1. Generate signing keys (see Tauri docs)
2. Add to GitHub Secrets:
   - `TAURI_PRIVATE_KEY`
   - `TAURI_KEY_PASSWORD`
3. Builds will be automatically signed

Without signing:
- macOS: Users see "unidentified developer" warning
- Windows: Users see "unknown publisher" warning

(Users can still install, they just need to click "Open Anyway")
