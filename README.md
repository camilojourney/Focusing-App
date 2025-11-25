# Hyper Awareness (Focus Time)

A menu bar focus timer application designed to help you maintain deep work sessions with periodic check-ins and reflection breaks.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey.svg)
![Tauri](https://img.shields.io/badge/Tauri-v2.9.3-brightgreen.svg)
![Status](https://img.shields.io/badge/status-v1.0%20Production-success.svg)

## 📥 Download

**Latest Release:** [v1.0.0](https://github.com/camilojourney/Focusing-App/releases/latest)

- **macOS** (10.13+): [Download DMG](https://github.com/camilojourney/Focusing-App/releases/latest/download/Hyper-Awareness_1.0.0_universal.dmg)
  - Universal Binary (Intel + Apple Silicon)
- **Windows** (10+): [Download Installer](https://github.com/camilojourney/Focusing-App/releases/latest/download/Hyper-Awareness_1.0.0_x64-setup.exe)

> **Note:** These are unsigned installers. On macOS, right-click → Open. On Windows, click "More info" → "Run anyway".

## Overview

**Hyper Awareness** is a productivity app that combines long-form focus sessions with periodic check-ins to help you maintain awareness and intentionality throughout your work. Unlike traditional Pomodoro timers, Hyper Awareness is designed for extended deep work sessions (default 12 hours) with intelligent interruptions that promote metacognition.

**v1.0 is now live!** The app provides a complete self-awareness system with menu bar integration, calendar sync, and comprehensive session logging.

### Key Features

#### Core Functionality
- 🧠 **Session Goal Tracking**: Set your intention at the start of each session
- ✅ **Interactive Check-ins**: Report what you're actually doing at each check-in point (every 15 min)
- 🎯 **Cognitive Awareness**: See your goal displayed when checking in - creates powerful metacognition
- 📅 **Calendar Integration**: Automatically detects current calendar events to help contextualize your focus
- 📊 **Session Review**: Timeline visualization of your focus patterns with statistics
- 💾 **Data Logging**: All check-ins logged locally to JSONL format for future analysis
- 🔔 **Status Tracking**: On Task, Social Media, Email/Chat, Other Distractions, or Taking a Break

#### Menu Bar Integration (macOS)
- 🍎 **Native Menu Bar App**: Lives in the system tray like Spotlight or Dropbox
- ⏱️ **Live Timer Display**: Countdown shown directly in menu bar (e.g., "14:32")
- 🎨 **macOS Sequoia Compatible**: Fixed rendering for latest macOS (colored icon support)
- 📍 **Smart Positioning**: Window centers on check-ins, appears near tray when clicked
- 👻 **Auto-Hide Behavior**: Window hides after Start and check-in responses
- 🖱️ **Fully Draggable**: Move window anywhere during interaction

#### Privacy & Settings
- 🔒 **Privacy First**: All data stored locally only - no cloud sync, complete privacy
- ⚙️ **Customizable Intervals**: Adjust session duration, check-in frequency, and write time
- 💾 **Persistent Settings**: Your preferences are saved between sessions

## How It Works

### The Focus Session Flow

1. **Launch App**: Click brain icon in menu bar
2. **Set Your Goal**: Enter what you want to accomplish (or use 📅 Event button for current meeting)
3. **Start Session**: Click "Start Focus" - window hides, timer runs in background
4. **Check-in Triggered**: When the check-in interval is reached (default: 15 minutes):
   - Session automatically pauses
   - Window appears **centered on screen**
   - Check-in screen shows your goal
5. **Report Status**: Click what you're actually doing:
   - ✅ On Task
   - ☕️ Taking a Break
   - 📱 Social Media
   - 📧 Email/Chat
   - 🔀 Other Distraction
   - ⏭️ Skip
6. **Optional Notes**: Add a quick reflection or note
7. **Data Logged**: Your response is saved to a local JSONL file
8. **Auto-Hide**: Window hides immediately after response
9. **Repeat**: Continue until the full session duration is complete

### Menu Bar Access

- **Click tray icon**: Shows/hides main window (positioned near tray)
- **Right-click tray icon**: Access menu (Show Timer, Settings, Quit)
- **Close window**: Red button hides window (app keeps running)
- **Quit app**: Use menu bar → Quit

### Default Settings

- **Session Duration**: 720 minutes (12 hours)
- **Check-in Interval**: 15 minutes
- **Write Time**: 20 seconds
- **Window Position**: Auto (centered on check-ins)

All settings can be customized through the settings window (⚙️ button or tray menu).

## Technologies

This app is built with modern, performant technologies:

**Frontend**
- HTML5, CSS3, JavaScript (vanilla - no frameworks)
- Glassmorphic UI with backdrop filters
- Tauri frontend API for native integration

**Backend**
- Rust with Tauri v2 framework
- Menu bar tray icon with timer text updates
- Calendar access via EventKit (macOS)
- Window positioning and auto-hide management
- JSONL-based append-only logging

**Build Tools**
- Tauri CLI v2
- pnpm for dependency management
- Cargo for Rust dependencies

## Project Structure

```
Focusing-App/
├── src/                      # Frontend application
│   ├── index.html           # Main timer interface
│   ├── settings.html        # Settings configuration UI
│   ├── main.js              # Core timer logic
│   ├── tauri-bridge.js      # Tauri IPC bridge
│   └── js/
│       └── sessionReview.js # Session history panel
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Main application logic & commands
│   │   ├── calendar.rs      # macOS Calendar integration (EventKit)
│   │   └── logs.rs          # JSONL logging and reading
│   ├── icons/               # Application icons
│   │   ├── tray-44x44.png   # Menu bar icon (RGBA colored)
│   │   └── icon.icns        # App bundle icon
│   ├── Cargo.toml           # Rust dependencies
│   ├── tauri.conf.json      # Tauri v2 configuration
│   └── entitlements.plist   # macOS permissions
├── docs/                    # Documentation
│   ├── DOCUMENT_OF_TRUTH.md # System design & architecture
│   ├── ROADMAP.md           # Feature roadmap
│   └── LESSONS.md           # Engineering lessons learned
├── specs/                   # Feature specifications
│   ├── 001-session-review-panel.md
│   ├── 002-calendar-integration.md
│   ├── 003-accountability-box.md
│   ├── 004-settings-management.md
│   ├── 005-data-logging.md
│   └── 006-menu-bar-integration.md
├── AGENTS.md                # Project constitution
├── package.json             # npm configuration
└── README.md               # This file
```

## Getting Started

### Prerequisites

- **Node.js** v18 or higher
- **pnpm** (package manager) - `npm install -g pnpm`
- **Rust** (latest stable version) - Required for Tauri backend
- **macOS** 11 or higher (tested on macOS Sequoia 15)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/Focusing-App.git
   cd Focusing-App
   ```

2. **Install Rust** (if not already installed):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   After installation, configure your current shell:

   ```bash
   source $HOME/.cargo/env
   ```

   Or add this to your `~/.zshrc` or `~/.bashrc` for permanent access:

   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

3. Install pnpm dependencies:

   ```bash
   pnpm install
   ```

4. Run in development mode:

   ```bash
   pnpm run dev
   ```

   **Note:** If you get a "cargo not found" error, make sure to run:

   ```bash
   source $HOME/.cargo/env
   ```

5. Build for production:

   ```bash
   pnpm run build
   ```

The compiled application will be available in `src-tauri/target/release/bundle/macos/Hyper Awareness.app`.

## macOS Sequoia Compatibility

If you're running **macOS Sequoia (15.x)**, the app has been tested and verified to work correctly with the following fix:

- **Tray Icon Rendering**: The app uses a colored RGBA PNG icon with `icon_as_template(false)` to ensure visibility in the menu bar. This is a deliberate trade-off for Sequoia compatibility - the icon will not adapt to light/dark mode changes but will always be visible.

For more technical details, see [specs/006-menu-bar-integration.md](specs/006-menu-bar-integration.md#macos-sequoia-compatibility).

## Usage

### Starting the App

After launching **Hyper Awareness**, you'll see a brain icon (🧠) in your macOS menu bar with a timer countdown.

**Menu Bar Options:**
- **Click the tray icon**: Show/hide the main window (positioned near tray)
- **Right-click the tray icon**: Access menu with Show Timer, Settings, Quit

### Using the Timer

1. **Set Your Goal**: Enter what you want to accomplish, or click 📅 **Event** to pull from your current calendar event
2. **Start a Session**: Click "Start Focus" button
   - Window automatically hides and moves to the background
   - Timer runs silently with live countdown in menu bar (e.g., "14:32")
3. **Check-in Occurs**: After the check-in interval (default: 15 minutes):
   - Session pauses automatically
   - Window appears **centered on screen**
   - Your original goal is displayed
   - You have 20 seconds (default) to respond
4. **Report Your Status**: Click one of 6 options:
   - ✅ **On Task** - You're doing what you intended
   - ☕️ **Taking a Break** - Intentional rest
   - 📱 **Social Media** - Got distracted by social apps
   - 📧 **Email/Chat** - Got distracted by messages
   - 🔀 **Other Distraction** - Something else pulled you away
   - ⏭️ **Skip** - Don't want to answer right now
5. **Optional Note**: Add a quick reflection (optional)
6. **Auto-Resume**: After you respond, the window hides and the session continues automatically
7. **Review Anytime**: Click 📊 **Review** to see your session timeline and focus statistics

### Configuring Settings

1. Click ⚙️ **Settings** button or access from the tray menu
2. Adjust your preferences:
   - **Session Duration**: Total focus time in minutes (default: 720 = 12 hours)
   - **Check-in Interval**: How often to check in (default: 15 minutes)
   - **Write Time**: How long you have to respond (default: 20 seconds)
   - **Window Position**: Auto (recommended) or Manual positioning
3. Click "💾 Save Settings" to apply changes

## Architecture

### Backend (Rust)

The Rust backend (`src-tauri/src/`) handles system integration and native functionality:

**Main Module** (`main.rs`):
- **Settings Management**: Load/save user preferences to disk
- **Menu Bar Tray**: Integration with macOS menu bar, live timer display
- **Window Management**: Show/hide, positioning (centered or tray-relative)
- **IPC Commands**: Expose Rust functions to frontend via Tauri commands

**Calendar Module** (`calendar.rs`):
- **EventKit Integration**: Access macOS Calendar to fetch current events
- **Permission Handling**: Request calendar access via native macOS dialogs

**Logs Module** (`logs.rs`):
- **JSONL Logging**: Append-only check-in data logging
- **Session Reading**: Parse and return session entries for review panel

**Key Tauri Commands:**
- `get_settings()` - Retrieve saved settings
- `save_settings(settings)` - Persist user preferences
- `open_settings()` - Launch settings window
- `update_tray_timer(text)` - Update menu bar timer display
- `position_window_at_top()` - Position window near tray icon
- `position_window_centered()` - Center window on screen (used for check-ins)
- `log_check_in(...)` - Save check-in data to JSONL file
- `list_session_entries(...)` - Read session data for review panel
- `get_current_event()` - Get current calendar event from macOS Calendar
- `request_calendar_permission()` - Request calendar access permission

### Frontend (JavaScript)

The frontend (`src/`) manages UI and user interaction:

**Main Interface** (`index.html` + `main.js`):
- **Session State**: Track timers and session status
- **UI Updates**: Real-time display of elapsed time, remaining time, check-ins completed
- **Timer Logic**: Coordinate session timer, check-in intervals, and write time
- **User Interactions**: Handle button clicks and state transitions
- **Auto-Hide Behavior**: Hide window after Start and after check-in responses

**Session Review** (`js/sessionReview.js`):
- **Timeline Visualization**: Display all check-ins in chronological order
- **Statistics**: Calculate focus score, distraction breakdown, time analysis
- **Export**: Copy session data to clipboard for further analysis

**State Management:**

```javascript
sessionTimeRemaining      // Total session countdown (e.g., 43200 seconds = 12 hours)
checkInTimeRemaining      // Next check-in countdown (e.g., 900 seconds = 15 min)
writeTimeRemaining        // Check-in response timeout (e.g., 20 seconds)
isSessionRunning          // Active session flag (true/false)
isCheckInPending          // Check-in screen visible flag (true/false)
completedCheckIns         // Count of completed check-ins this session
```

### Communication Flow

```
Frontend (HTML/JS) <--IPC--> Tauri Commands <---> Backend (Rust) <---> macOS APIs
                                                         |
                                                         +---> EventKit (Calendar)
                                                         +---> NSStatusBar (Tray)
                                                         +---> File System (Logs/Settings)
```

## Recent Updates (v1.0 - November 2025)

### ✅ Completed Features

- **macOS Sequoia Compatibility**: Fixed tray icon rendering issue for macOS 15.x
- **Window Positioning System**: Three modes (centered, tray-relative, fully draggable)
- **Auto-Hide Behavior**: Window hides after Start button and after check-in responses
- **Standard Window Decorations**: Native macOS close/minimize/maximize buttons
- **Session Review Panel**: Timeline visualization with focus statistics
- **Calendar Integration**: One-click goal setting from current calendar events
- **Live Menu Bar Timer**: Real-time countdown displayed directly in menu bar
- **Complete Settings Management**: Persistent configuration for all app behavior

### 🔧 Technical Improvements

- Upgraded to Tauri v2.1.0 (from v1.8.3)
- Implemented `position_window_centered()` command for check-ins
- Added debug logging with emoji markers for window hide operations
- Created modular Rust architecture (main.rs, calendar.rs, logs.rs)
- Improved JSONL logging with structured session data

## Platform Notes

### macOS

Full functionality including:
- Automatic desktop switching via AppleScript
- Calendar integration with EventKit for current event detection
- Adaptive menu bar icon that automatically switches colors with system theme
- Menu bar timer display

**Calendar Permission**: On first launch, the app will request permission to access your calendar. This allows it to show your current event during focus sessions.

### Windows/Linux

The app can be compiled for Windows and Linux, but the following features are macOS-specific:
- Desktop switching (uses AppleScript)
- Calendar integration (uses EventKit)
- Adaptive menu bar icon (uses macOS template images)

Alternative implementations would be needed for full cross-platform support.

## Configuration Files

Settings and data are stored in a platform-specific configuration directory:

**macOS:**
- Settings: `~/Library/Application Support/com.focustime.app/settings.json`
- Log data: `~/Library/Application Support/com.focustime.app/focus_log.jsonl`
- Accountability Box: `~/Library/Application Support/com.focustime.app/accountability_box.jsonl`

**Linux:**
- Settings: `~/.config/focus-time/settings.json`
- Log data: `~/.config/focus-time/focus_log.jsonl`
- Accountability Box: `~/.config/focus-time/accountability_box.jsonl`

**Windows:**
- Settings: `%APPDATA%\com.focustime.app\settings.json`
- Log data: `%APPDATA%\com.focustime.app\focus_log.jsonl`
- Accountability Box: `%APPDATA%\com.focustime.app\accountability_box.jsonl`

### Settings Format

The settings file is JSON formatted:
```json
{
  "session_duration": 720,
  "check_in_interval": 15,
  "write_time": 20
}
```

### Log Data Format

Each check-in creates one line in the JSONL (JSON Lines) file:
```json
{
  "timestamp": "2025-10-20T21:30:00.123Z",
  "session_goal": "Finish chapter 4 of my thesis",
  "reported_status": "On Task",
  "notes": "Making good progress",
  "session_duration_setting": 720,
  "check_in_interval_setting": 15,
  "write_time_setting": 20,
  "check_in_number": 5
}
```

This format is perfect for data analysis with Python, R, or any data tool.

### Accountability Box Format

The Accountability Box stores your reflection answers in JSONL format:
```json
{
  "timestamp": "2025-10-20T22:30:00.123Z",
  "q1_accomplished": "Completed the tray icon fix and implemented accountability feature",
  "q2_challenges": "Had to debug icon loading issues with Tauri",
  "q3_tomorrow": "Add data visualization for the accountability responses",
  "q4_focus_rating": "8/10 - stayed mostly on task with good flow",
  "q5_learned": "Learned how to create PNG files programmatically in Python"
}
```

**The 5 Accountability Questions:**
1. What did you accomplish today?
2. What challenges did you face?
3. What will you do tomorrow?
4. How focused were you? (1-10)
5. What did you learn?

These questions help you reflect on your work and build self-awareness over time.

## Troubleshooting

### Menu Bar Icon Not Showing

If the brain icon doesn't appear in your menu bar:

1. **Check System Theme Compatibility**: The icon uses macOS template images which adapt to your theme
2. **Restart the Dock**: Sometimes macOS needs a refresh:
   ```bash
   killall Dock
   ```
3. **Verify Icon File**: Ensure `src-tauri/icons/18x18.png` exists and is a valid template image:
   - Pure black (`#000000`) pixels for visible parts
   - Transparent (alpha = 0) for invisible parts
   - No anti-aliasing or gray pixels

4. **Check Configuration**: In [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json), verify:
   ```json
   "systemTray": {
     "iconPath": "icons/18x18.png",
     "iconAsTemplate": true,
     "menuOnLeftClick": false
   }
   ```

### Calendar Permission Issues

If calendar integration isn't working:

1. Open **System Settings > Privacy & Security > Calendars**
2. Ensure "Hyper Awareness" has permission enabled
3. If not listed, click the app's "Request Permission" button

### Build Errors

If you encounter build errors:

1. **Update Rust**: `rustup update`
2. **Clean build**: `cargo clean` in `src-tauri/`
3. **Reinstall dependencies**: `pnpm install`

## Development

### Hot Reload Development

```bash
pnpm run dev
```

This starts the Tauri development server with hot reload for both frontend and backend changes.

### Building for Production

```bash
pnpm run build
```

Creates an optimized production bundle in `src-tauri/target/release/bundle/`.

### Modifying the UI

Edit files in the `src/` directory:
- [src/index.html](src/index.html) - Main timer interface
- [src/settings.html](src/settings.html) - Settings dialog

### Modifying Backend Logic

Edit [src-tauri/src/main.rs](src-tauri/src/main.rs) for:
- Tauri commands
- System tray behavior
- Settings persistence
- Desktop switching logic

## Use Cases

Focus Time is ideal for:

- **Deep Work Practitioners**: Long focus sessions with mindful check-ins
- **Writers**: Extended writing sessions with periodic reflection
- **Developers**: Full-day coding sessions with awareness breaks
- **Researchers**: Long study sessions with progress tracking
- **Anyone**: Looking to build intentionality into their focused work

## Data Analysis

After using Focus Time for a while, you can analyze your focus patterns using the logged data.

### Viewing Your Data

```bash
# macOS - view your log file
cat ~/Library/Application\ Support/com.focustime.app/focus_log.jsonl

# Validate JSON format (requires jq: brew install jq)
cat ~/Library/Application\ Support/com.focustime.app/focus_log.jsonl | jq .
```

### Python Analysis Example

```python
import json
from collections import Counter

# Read all log entries
with open('focus_log.jsonl', 'r') as f:
    entries = [json.loads(line) for line in f]

# Calculate on-task percentage
on_task = sum(1 for e in entries if e['reported_status'] == 'On Task')
total = len(entries)
print(f"On-task rate: {on_task/total*100:.1f}%")

# Find most common distractions
statuses = [e['reported_status'] for e in entries]
print(Counter(statuses).most_common())

# Analyze by time of day
import datetime
for entry in entries:
    dt = datetime.datetime.fromisoformat(entry['timestamp'].replace('Z', '+00:00'))
    print(f"{dt.hour:02d}:00 - {entry['reported_status']}")
```

### Excel/Google Sheets Import

1. Open your JSONL file in a text editor
2. Each line is a separate JSON object
3. Use Excel's "Power Query" or Google Sheets' "ImportJSON" to parse
4. Create pivot tables for:
   - Status distribution
   - Time-of-day patterns
   - Goal achievement tracking

### Insights You Can Discover

- **Focus Patterns**: What times of day are you most focused?
- **Distraction Triggers**: What types of distractions are most common?
- **Goal Correlation**: Do certain types of goals lead to better focus?
- **Session Optimization**: Is 15-minute check-in interval optimal for you?
- **Behavioral Trends**: How does your focus change over days/weeks?

## Customization Ideas

Here are some ways you could extend Focus Time:

**Current Roadmap:**
- **Phase 2**: Data visualization dashboard with charts and metrics
- **Phase 3**: AI-powered predictions and personalized insights
  - **Accountability Box**: Click the "Accountability Box" button to answer reflection questions and save them to your personal accountability log
- **Phase 4**: Smart interventions based on your patterns

**Other Ideas:**
- Add sound notifications for check-ins
- Integrate with note-taking apps during write time
- Track and visualize focus session history
- Add different timer presets (short, medium, long sessions)
- Include focus metrics and analytics
- Support for different reflection prompts at check-in
- Cross-platform desktop switching implementations
- Export reports (daily/weekly summaries)
- Gamification with streaks and achievements

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

Built with [Tauri](https://tauri.app/) - A framework for building tiny, fast binaries for all major desktop platforms.

---

**Happy Focusing!** 🧠
