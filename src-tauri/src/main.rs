#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod calendar;
mod logs;

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::{
    api::path::app_config_dir,
    AppHandle,
    CustomMenuItem,
    Manager,
    PhysicalPosition,
    SystemTray,
    SystemTrayEvent,
    SystemTrayMenu,
    SystemTrayMenuItem,
    WindowBuilder,
    WindowUrl,
};

// Store the last known tray icon position
struct TrayPosition {
    x: f64,
    y: f64,
    _width: f64,
    height: f64,
}

struct AppState {
    tray_position: Mutex<Option<TrayPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    session_duration: u32,
    check_in_interval: u32,
    write_time: u32,
    window_position: String, // "auto" or "right-edge"
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            session_duration: 720,
            check_in_interval: 15,
            write_time: 20,
            window_position: "auto".to_string(),
        }
    }
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut path = app_config_dir(&app.config())
        .ok_or("Unable to determine app config directory")?;
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("settings.json");
    Ok(path)
}

fn load_settings(app: &AppHandle) -> Result<Settings, String> {
    let path = settings_path(app)?;
    if path.exists() {
        let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let settings: Settings = serde_json::from_str(&data).map_err(|e| e.to_string())?;
        Ok(settings)
    } else {
        Ok(Settings::default())
    }
}

fn save_settings_file(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let path = settings_path(app)?;
    let data = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

fn log_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut path = app_config_dir(&app.config())
        .ok_or("Unable to determine app config directory")?;
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("focus_log.jsonl");
    Ok(path)
}


#[tauri::command]
fn get_settings(app: AppHandle) -> Result<Settings, String> {
    load_settings(&app)
}

#[tauri::command]
fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    save_settings_file(&app, &settings)
}

#[tauri::command]
fn open_settings(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WindowBuilder::new(
        &app,
        "settings",
        WindowUrl::App("settings.html".into()),
    )
    .title("Settings")
    .inner_size(400.0, 430.0)
    .resizable(false)
    .decorations(true)
    .visible(true)
    .always_on_top(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn update_tray_timer(app: AppHandle, timer_text: String) -> Result<(), String> {
    let tray_handle = app.tray_handle();

    // Update the menu bar title (shows next to icon in menu bar)
    tray_handle.set_title(&timer_text)
        .map_err(|e| e.to_string())?;

    // Also update the menu item for consistency
    tray_handle.get_item("timer").set_title(&format!("🧠 {}", timer_text))
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn calculate_window_position(settings: &Settings, monitor_size: &tauri::PhysicalSize<u32>) -> (i32, i32) {
    let window_width = 380;
    let y = 50; // Just below menu bar with some padding

    let x = match settings.window_position.as_str() {
        "right-edge" => {
            // Position with right edge near screen edge (with 20px padding for safety)
            (monitor_size.width as i32) - window_width - 20
        },
        _ => {
            // Default "auto" - position closer to where menu bar icons typically are
            // Menu bar icons are usually in the rightmost ~300px area
            // Center the window around that area
            let from_right = 250; // Distance from right edge to center of window
            (monitor_size.width as i32) - from_right - (window_width / 2)
        }
    };

    (x, y)
}

#[tauri::command]
fn position_window_at_top(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_window("main") {
        // Try to get the stored tray position
        let state = app.state::<AppState>();
        let tray_pos = state.tray_position.lock().unwrap();

        if let Some(pos) = tray_pos.as_ref() {
            // Use stored tray icon position
            let window_width = 380.0;
            let icon_width = 22.0;
            let icon_center_x = pos.x + (icon_width / 2.0);
            let window_x = (icon_center_x - (window_width / 2.0)).round() as i32;
            let window_y = (pos.y + pos.height + 5.0).round() as i32;

            window.set_position(tauri::Position::Physical(PhysicalPosition {
                x: window_x,
                y: window_y
            }))
            .map_err(|e| e.to_string())?;
        } else {
            // Fallback to old positioning if we don't have tray position yet
            let settings = load_settings(&app).unwrap_or_default();
            if let Ok(Some(monitor)) = window.current_monitor() {
                let monitor_size = monitor.size();
                let (x, y) = calculate_window_position(&settings, &monitor_size);
                window.set_position(tauri::Position::Physical(PhysicalPosition { x, y }))
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn log_check_in(app: AppHandle, log_line: String) -> Result<(), String> {
    let path = log_file_path(&app)?;

    // Open file in append mode (create if doesn't exist)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    // Write the JSON line with a newline
    writeln!(file, "{}", log_line).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_current_event() -> Result<Option<String>, String> {
    calendar::get_current_calendar_event()
}

#[tauri::command]
fn request_calendar_permission() -> Result<String, String> {
    calendar::request_calendar_access()
}

#[tauri::command]
fn list_session_entries(app: AppHandle, start_time_iso: String) -> Result<Vec<logs::SessionEntry>, String> {
    use chrono::DateTime;

    // Parse the ISO timestamp
    let start_time = DateTime::parse_from_rfc3339(&start_time_iso)
        .map_err(|e| format!("Invalid timestamp format: {}", e))?
        .with_timezone(&chrono::Utc);

    // Read entries since start time
    logs::read_since(&app, start_time)
}


fn main() {
    // Create system tray menu
    let timer_item = CustomMenuItem::new("timer", "15:00").disabled();
    let show = CustomMenuItem::new("show", "Show Timer");
    let settings_item = CustomMenuItem::new("settings", "Settings");
    let quit = CustomMenuItem::new("quit", "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(timer_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show)
        .add_item(settings_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("Hyper Awareness")
        .with_menu_on_left_click(false);

    tauri::Builder::default()
        .manage(AppState {
            tray_position: Mutex::new(None),
        })
        .setup(|app| {
            // Make this a menu bar-only app (no dock icon)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let tray_handle = app.tray_handle();

            // Load settings and update tray with correct initial time
            let settings = load_settings(&app.handle()).unwrap_or_default();
            let initial_time = format!("{}:00", settings.check_in_interval);

            // Update tray menu item title
            let _ = tray_handle.get_item("timer").set_title(&format!("🧠 {}", initial_time));

            // Hide main window initially
            if let Some(window) = app.get_window("main") {
                // Disable window shadow on macOS to prevent white border
                #[cfg(target_os = "macos")]
                {
                    use cocoa::appkit::NSWindow;
                    use cocoa::base::{id, NO};

                    let ns_window = window.ns_window().unwrap() as id;
                    unsafe {
                        ns_window.setHasShadow_(NO);
                    }
                }
                let _ = window.hide();
            }

            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { position, size, .. } => {
                // Store the tray position for later use
                let state = app.state::<AppState>();
                let mut tray_pos = state.tray_position.lock().unwrap();
                *tray_pos = Some(TrayPosition {
                    x: position.x,
                    y: position.y,
                    _width: size.width as f64,
                    height: size.height as f64,
                });
                drop(tray_pos); // Release lock before showing window

                // Toggle window visibility when clicking the tray icon
                if let Some(window) = app.get_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        // Window is visible, hide it
                        let _ = window.hide();
                    } else {
                        // Window is hidden, show it
                        // Position window directly below the tray icon
                        let window_width = 380.0;
                        let icon_width = 22.0; // Standard macOS menu bar icon width

                        // position.x is the left edge of the ENTIRE status item (icon + text)
                        // We want to center under just the icon, which is at the left
                        let icon_center_x = position.x + (icon_width / 2.0);
                        let window_x = (icon_center_x - (window_width / 2.0)).round() as i32;
                        let window_y = (position.y + size.height as f64 + 5.0).round() as i32;

                        let _ = window.set_position(tauri::Position::Physical(PhysicalPosition {
                            x: window_x,
                            y: window_y
                        }));
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "show" => {
                        if let Some(window) = app.get_window("main") {
                            // Try to use stored tray position
                            let state = app.state::<AppState>();
                            let tray_pos = state.tray_position.lock().unwrap();

                            if let Some(pos) = tray_pos.as_ref() {
                                let window_width = 380.0;
                                let icon_width = 22.0;
                                let icon_center_x = pos.x + (icon_width / 2.0);
                                let window_x = (icon_center_x - (window_width / 2.0)).round() as i32;
                                let window_y = (pos.y + pos.height + 5.0).round() as i32;

                                let _ = window.set_position(tauri::Position::Physical(PhysicalPosition {
                                    x: window_x,
                                    y: window_y
                                }));
                            } else {
                                // Fallback to settings-based positioning
                                let settings = load_settings(app).unwrap_or_default();
                                if let Ok(Some(monitor)) = window.current_monitor() {
                                    let monitor_size = monitor.size();
                                    let (x, y) = calculate_window_position(&settings, &monitor_size);
                                    let _ = window.set_position(tauri::Position::Physical(PhysicalPosition { x, y }));
                                }
                            }
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    "settings" => {
                        // Open settings window
                        if let Some(window) = app.get_window("settings") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        } else {
                            let _ = WindowBuilder::new(
                                app,
                                "settings",
                                WindowUrl::App("settings.html".into()),
                            )
                            .title("Settings")
                            .inner_size(400.0, 430.0)
                            .resizable(false)
                            .build();
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            SystemTrayEvent::DoubleClick { position, size, .. } => {
                // Store the tray position
                let state = app.state::<AppState>();
                let mut tray_pos = state.tray_position.lock().unwrap();
                *tray_pos = Some(TrayPosition {
                    x: position.x,
                    y: position.y,
                    _width: size.width as f64,
                    height: size.height as f64,
                });
                drop(tray_pos);

                // Open main window on double click
                if let Some(window) = app.get_window("main") {
                    // Position window directly below the tray icon
                    let window_width = 380.0;
                    let icon_width = 22.0;
                    let icon_center_x = position.x + (icon_width / 2.0);
                    let window_x = (icon_center_x - (window_width / 2.0)).round() as i32;
                    let window_y = (position.y + size.height as f64 + 5.0).round() as i32;

                    let _ = window.set_position(tauri::Position::Physical(PhysicalPosition {
                        x: window_x,
                        y: window_y
                    }));
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            open_settings,
            update_tray_timer,
            position_window_at_top,
            log_check_in,
            get_current_event,
            request_calendar_permission,
            list_session_entries
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
