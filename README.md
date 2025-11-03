# Focus Time

A macOS menu bar focus timer application designed to help you maintain deep work sessions with periodic check-ins and reflection breaks.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)
![Tauri](https://img.shields.io/badge/Tauri-v1.8.3-brightgreen.svg)

## Overview

Focus Time is a productivity app that combines long-form focus sessions with periodic check-ins to help you maintain awareness and intentionality throughout your work. Unlike traditional Pomodoro timers, Focus Time is designed for extended deep work sessions (default 12 hours) gzxadr

**Phase 1 "Self-Aware MVP" is now complete!** The app tracks your intentions vs. actual behavior, creating a data foundation for future AI-powered insights. See [PHASE1_IMPLEMENTATION.md](PHASE1_IMPLEMENTATION.md) for details.

### Key Features

- **Session Goal Tracking**: Set your intention at the start of each session
- **Interactive Check-ins**: Report what you're actually doing at each check-in point
- **Cognitive Awareness**: See your goal displayed when checking in - creates powerful metacognition
- **Calendar Integration**: Automatically detects current calendar events to help contextualize your focus
- **Data Logging**: All check-ins are logged locally to JSONL format for future analysis
- **Status Tracking**: On Task, Social Media, Email/Chat, Other Distractions, or Taking a Break
- **Accountability Box**: Answer deep reflection questions and save them to your personal accountability log
- **Automatic Desktop Switching**: Switches to Desktop 1 on macOS during check-ins for a clean reflection space
- **Menu Bar Integration**: Beautiful adaptive icon that changes with system theme (white in dark mode, black in light mode)
- **Menu Bar Timer**: Live countdown display right in your menu bar
- **Persistent Settings**: Your preferences are saved between sessions
- **Privacy First**: All data stored locally only - no cloud sync, complete privacy

## How It Works

### The Focus Session Flow

1. **Set Your Goal**: Enter what you want to accomplish (e.g., "Finish chapter 4 of my thesis")
2. **Start Session**: Begin your focus timer. Both the session timer and check-in countdown start.
3. **Check-in Triggered**: When the check-in interval is reached (default: 15 minutes):
   - Session automatically pauses
   - Desktop switches to Desktop 1 (macOS)
   - Check-in screen appears showing your goal
4. **Report Status**: Click what you're actually doing:
   - ✅ On Task
   - 📱 Social Media (distraction)
   - 📧 Email/Chat (distraction)
   - 🔀 Other Distraction
   - ☕️ Taking a Break
5. **Optional Notes**: Add a quick reflection or note
6. **Data Logged**: Your response is saved to a local JSONL file
7. **Auto-Resume**: Session automatically resumes after you respond (or after 20s timeout)
8. **Repeat**: Continue until the full session duration is complete

### Default Settings

- **Session Duration**: 720 minutes (12 hours)
- **Check-in Interval**: 15 minutes
- **Write Time**: 20 seconds

All settings can be customized through the settings window.

## Technologies

This app is built with modern, performant technologies:

**Frontend**
- HTML5, CSS3, JavaScript (vanilla)
- Tauri frontend API for native integration

**Backend**
- Rust with Tauri framework
- System tray integration with adaptive icon (macOS template image)
- Calendar access via EventKit (macOS)
- AppleScript execution for desktop switching

**Build Tools**
- Tauri CLI
- npm for dependency management
- Cargo for Rust dependencies

## Project Structure

```
Focusing-App/
├── src/                      # Frontend application
│   ├── index.html           # Main timer interface
│   └── settings.html        # Settings configuration UI
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Main application logic
│   │   └── calendar.rs      # macOS Calendar integration
│   ├── icons/               # Application icons
│   │   └── 18x18.png        # Menu bar icon (template image)
│   ├── Cargo.toml           # Rust dependencies
│   ├── tauri.conf.json      # Tauri configuration
│   └── entitlements.plist   # macOS permissions
├── package.json             # npm configuration
└── README.md               # This file
```

## Getting Started

### Prerequisites

- **Node.js** (v14 or higher)
- **Rust** (latest stable version) - Required for Tauri backend
- **macOS** (for full functionality including desktop switching)

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

3. Install npm dependencies:
   ```bash
   npm install
   ```

4. Run in development mode:
   ```bash
   npm run dev
   ```

   **Note:** If you get a "cargo not found" error, make sure to run:
   ```bash
   source $HOME/.cargo/env
   ```

5. Build for production:
   ```bash
   npm run build
   ```

The compiled application will be available in `src-tauri/target/release/bundle/`.

## Usage

### Starting the App

After launching Focus Time, you'll see a timer icon in your macOS menu bar.

**Menu Bar Options:**
- Click the tray icon to show/hide the main window
- Right-click for menu: Show, Settings, Quit

### Using the Timer

1. **Start a Session**: Click the "Start Session" button
2. **Monitor Progress**: Watch your session time, check-in countdown, and completed check-ins
3. **Handle Check-ins**: When a check-in occurs:
   - Your desktop will switch to Desktop 1
   - Take a moment to reflect during the writing countdown
   - Session resumes automatically
4. **Pause/Resume**: Use the pause button if you need to take a break
5. **Reset**: Start fresh with the reset button

### Configuring Settings

1. Click the "Settings" button or access from the menu bar
2. Adjust your preferences:
   - **Session Duration**: Total focus time in minutes
   - **Check-in Interval**: How often to pause for reflection
   - **Write Time**: Duration of reflection period in seconds
3. Click "Save Settings" to apply changes

## Architecture

### Backend (Rust)

The Rust backend ([src-tauri/src/main.rs](src-tauri/src/main.rs)) handles:

- **Settings Management**: Load and save user preferences to disk
- **System Tray**: Menu bar integration and timer display
- **Window Management**: Show/hide application windows
- **Desktop Switching**: Execute AppleScript commands for macOS desktop control
- **IPC Commands**: Expose functions to the frontend via Tauri's command system

**Key Tauri Commands:**
- `get_settings()` - Retrieve saved settings
- `save_settings()` - Persist user preferences
- `open_settings()` - Launch settings window
- `log_check_in()` - Save check-in data to JSONL file
- `get_current_event()` - Get current calendar event from macOS Calendar
- `request_calendar_permission()` - Request calendar access permission
- `update_tray_timer()` - Update menu bar timer display

### Frontend (JavaScript)

The frontend ([src/index.html](src/index.html)) manages:

- **Session State**: Track timers and session status
- **UI Updates**: Real-time display of elapsed time, remaining time, and check-ins
- **Timer Logic**: Coordinate session timer, check-in intervals, and write time
- **User Interactions**: Handle button clicks and state transitions

**State Management:**
```javascript
sessionTimeRemaining      // Total session countdown
checkInTimeRemaining      // Next check-in countdown
writeTimeRemaining        // Writing mode countdown
isSessionRunning          // Active session flag
isWriting                 // Writing mode flag
```

### Communication Flow

```
Frontend (HTML/JS) <---> Tauri IPC <---> Backend (Rust) <---> System (macOS)
     |                                        |
     |                                        |
  UI Updates                            File I/O, Tray,
  Timers                                Desktop Switching
```

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
3. **Reinstall dependencies**: `npm install`

## Development

### Hot Reload Development

```bash
npm run dev
```

This starts the Tauri development server with hot reload for both frontend and backend changes.

### Building for Production

```bash
npm run build
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
