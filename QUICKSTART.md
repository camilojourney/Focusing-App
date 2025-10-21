# Quick Start Guide - Focus Time

Get up and running with Focus Time in 5 minutes!

## Prerequisites

Before you start, make sure you have:

1. **Node.js** (v14 or higher)
   ```bash
   node --version  # Should be 14+
   ```

2. **Rust and Cargo**
   ```bash
   # Install Rust if not already installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Verify installation
   cargo --version
   ```

3. **macOS** (for full functionality including desktop switching)

## Installation

```bash
# Clone or navigate to the project
cd Focusing-App

# Install npm dependencies
npm install

# Run in development mode
npm run dev
```

The app should launch and appear in your menu bar!

## First Session

### 1. Set Your Goal
When the app window opens, you'll see a text input field:
```
Your Goal for this Session: [________________]
```

Enter what you want to accomplish. Examples:
- "Write 1000 words for Chapter 3"
- "Debug authentication issues"
- "Study for chemistry exam"
- "Complete client proposal"

### 2. Start the Session
Click the **"Start Session"** button. You'll see:
- Timer counting down to next check-in (15:00 → 14:59...)
- Session progress (0 / 720 min)
- Check-ins completed counter

### 3. Work!
Focus on your task. The app runs in the background.

### 4. Check-in Time
After 15 minutes, the check-in screen appears:

```
Your Goal: "Write 1000 words for Chapter 3"

What are you doing right now?

[✅ On Task]  [📱 Social Media]
[📧 Email/Chat]  [🔀 Other]
[☕️ Taking a Break]

Optional Notes: [________________]

(Resuming in 20 seconds...)
```

**Click** what you're actually doing. Be honest! This is for you, not anyone else.

### 5. Session Continues
After you click (or after 20 seconds), the session automatically resumes for another 15 minutes.

### 6. Repeat
Continue working and checking in until your session is complete!

## Understanding the UI

### Main Screen Elements

```
🧠 Focus Time
─────────────────────────────
Your Goal for this Session:
[Finish chapter 4 of thesis]

Ready to start

Next Check-in
15:00

[Start Session]  [Reset]
[Settings]  [Test Switch]

Session: 0 / 720 min
Check-ins completed: 0
```

**Status Messages:**
- "Ready to start" - Before session begins
- "Session active" - Timer is running
- "Session paused" - Manually paused

**Timer Display:**
- Shows time until next check-in (MM:SS format)
- Updates every second while running

**Session Progress:**
- "0 / 720 min" = 0 minutes elapsed of 720 total
- Updates as session runs

### Check-in Screen Elements

```
Your Goal: "Write thesis chapter"

What are you doing right now?

[Status Buttons]

Optional Notes: [Quick thoughts...]

Resuming in 18 seconds...
```

**Status Buttons:**
- ✅ **On Task**: You're working on your stated goal
- 📱 **Social Media**: Twitter, Reddit, TikTok, etc.
- 📧 **Email/Chat**: Email, Slack, iMessage, etc.
- 🔀 **Other**: Any other distraction
- ☕️ **Taking a Break**: Intentional break time

**Notes Field** (Optional):
- Quick reflection: "Making good progress"
- Context: "Stuck on this bug"
- Reminder: "Need to research this topic"

## Menu Bar Features

Look for the 🧠 icon in your macOS menu bar.

**Left Click**: Show/hide the timer window
**Right Click**: Menu with options
- Timer (displays current countdown)
- Show Timer
- Settings
- Quit

## Settings

Click **"Settings"** to customize:

```
Session Duration: 720 minutes (12 hours)
Check-in Interval: 15 minutes
Write Time: 20 seconds
```

**Session Duration**: Total time for your focus session
- Default: 720 min (12 hours) for full work day
- Try: 240 min (4 hours) for half session
- Or: 120 min (2 hours) for shorter blocks

**Check-in Interval**: How often to pause and reflect
- Default: 15 minutes
- Try: 10 minutes for higher awareness
- Or: 30 minutes for longer deep work

**Write Time**: Countdown during check-in screen
- Default: 20 seconds
- Adjust if you want more/less time to respond

## Tips for Success

### Week 1: Getting Started
- **Don't judge yourself** - Just observe and report honestly
- **Set specific goals** - "Write Chapter 4" not just "Write"
- **Use the notes** - Quick reflections help later
- **Trust the process** - The cognitive awareness builds over time

### Week 2-3: Building the Habit
- **Notice patterns** - What distracts you most?
- **Experiment with intervals** - Maybe 10 min works better than 15?
- **Refine your goals** - Learn to set achievable session goals
- **Use breaks intentionally** - Click "Taking a Break" when you need one

### Week 4: Analysis
- **Review your data** - See [Data Analysis](README.md#data-analysis) section
- **Calculate your focus rate** - What % on-task?
- **Identify triggers** - When do you get distracted?
- **Adjust your strategy** - Use insights to improve

## Common Workflows

### Deep Work Session (4-6 hours)
```
Settings: 360 min session, 15 min check-ins
Goal: "Complete client proposal draft"
Strategy:
- First 2 hours: Research and outline
- Middle 2 hours: Writing
- Last 1-2 hours: Review and polish
```

### Study Session (2-3 hours)
```
Settings: 180 min session, 10 min check-ins
Goal: "Study chapters 5-7 for chemistry exam"
Strategy:
- Shorter check-ins for higher accountability
- Take intentional 5-min breaks every hour
- Use notes to track concepts learned
```

### Writing Sprint (2 hours)
```
Settings: 120 min session, 20 min check-ins
Goal: "Write 2000 words for blog post"
Strategy:
- Longer intervals for flow state
- Track word count in notes at each check-in
- Minimize interruptions
```

## Desktop Switching Feature

**What it does**: When a check-in occurs, your Mac switches to Desktop 1

**Why**: Creates a clean space for reflection, away from open tabs/apps

**Setup**:
1. Ensure you have multiple desktops (Mission Control)
2. Keep Desktop 1 clean/minimal
3. Do your work on Desktop 2+

**Testing**:
Click the **"Test Switch"** button to verify it works

**Disable it** (if you prefer):
Edit [src/index.html](src/index.html) and comment out:
```javascript
// await invoke('switch_desktop');
```

## Data Location

Your data is stored locally at:
```
~/Library/Application Support/com.focustime.app/
├── settings.json       # Your preferences
└── focus_log.jsonl    # All check-in data
```

**Backup**: Just copy these files to save your data

**Privacy**: Nothing is sent to any server. All local.

## Troubleshooting

### App won't start
```bash
# Check Rust is installed
cargo --version

# Reinstall dependencies
npm install

# Try again
npm run dev
```

### Check-in screen doesn't appear
- Check console for errors (Cmd+Option+I in dev mode)
- Verify timer is counting down
- Try a shorter interval (1 minute) for testing

### Desktop switching doesn't work
- macOS only feature
- Requires System Events permissions
- Test with "Test Switch" button
- Check you have multiple desktops set up

### Can't find log file
```bash
# List contents of config directory
ls -la ~/Library/Application\ Support/com.focustime.app/

# If empty, trigger a check-in first
# Log file is created on first check-in
```

### Want to reset all data
```bash
# Delete the entire config directory
rm -rf ~/Library/Application\ Support/com.focustime.app/

# Restart the app - fresh start!
```

## Keyboard Shortcuts

Currently none implemented, but you could add:
- `Cmd+S` - Start/Stop session
- `Cmd+R` - Reset session
- `Cmd+,` - Open settings
- Numbers 1-5 - Quick check-in responses

## Next Steps

1. ✅ Complete your first session
2. ✅ Accumulate 1 week of data
3. ✅ Review your patterns
4. ✅ Adjust settings based on insights
5. ✅ Use for 1 month to build dataset
6. 📊 Analyze your data (see [README](README.md#data-analysis))
7. 🚀 Move to Phase 2 (coming soon!)

## Need Help?

- **Full Documentation**: [README.md](README.md)
- **Implementation Details**: [PHASE1_IMPLEMENTATION.md](PHASE1_IMPLEMENTATION.md)
- **Issues/Bugs**: Create a GitHub issue
- **Questions**: Check the README's Use Cases and Troubleshooting sections

---

**Ready to start?** Run `npm run dev` and begin your first focus session! 🧠
