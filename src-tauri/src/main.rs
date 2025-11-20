// Temporarily allow console in release mode for debugging
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod calendar;
mod logs;

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{ActivationPolicy, AppHandle, Manager, PhysicalPosition};

// Store the last known tray icon position
struct TrayPosition {
    x: f64,
    y: f64,
    _width: f64,
    _height: f64,
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
    let mut path = app.path().app_config_dir().map_err(|e| e.to_string())?;
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
    let mut path = app.path().app_config_dir().map_err(|e| e.to_string())?;
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
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let _window = tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("settings.html".into()),
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
    if let Some(tray) = app.tray_by_id("main") {
        // Update the menu bar title (shows next to icon in menu bar)
        tray.set_title(Some(&timer_text))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn calculate_window_position(
    settings: &Settings,
    monitor_size: &tauri::PhysicalSize<u32>,
) -> (i32, i32) {
    let window_width = 380;
    let y = 50; // Just below menu bar with some padding

    let x = match settings.window_position.as_str() {
        "right-edge" => {
            // Position with right edge near screen edge (with 20px padding for safety)
            (monitor_size.width as i32) - window_width - 20
        }
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
    if let Some(window) = app.get_webview_window("main") {
        // Try to get the stored tray position
        let state = app.state::<AppState>();
        let tray_pos = state.tray_position.lock().unwrap();

        if let Some(pos) = tray_pos.as_ref() {
            // Get the monitor where the tray icon is located
            if let Ok(Some(monitor)) = window.current_monitor() {
                let scale_factor = monitor.scale_factor();
                let monitor_position = monitor.position();
                let monitor_size = monitor.size();

                eprintln!("🔍 Tray Position (physical): x={}, y={}", pos.x, pos.y);
                eprintln!("🔍 Monitor scale: {}", scale_factor);
                eprintln!(
                    "🔍 Monitor position: ({}, {})",
                    monitor_position.x, monitor_position.y
                );
                eprintln!(
                    "🔍 Monitor size: ({}, {})",
                    monitor_size.width, monitor_size.height
                );

                // Convert from physical pixels to logical pixels
                let tray_x_logical = pos.x / scale_factor;
                let tray_y_logical = pos.y / scale_factor;

                eprintln!(
                    "🔍 Tray Position (logical): x={}, y={}",
                    tray_x_logical, tray_y_logical
                );

                // Use stored tray icon position (now in logical pixels)
                let window_width = 380.0;
                let icon_width = 22.0;
                let icon_center_x = tray_x_logical + (icon_width / 2.0);
                let mut window_x = (icon_center_x - (window_width / 2.0)).round() as i32;
                let window_y = (tray_y_logical + icon_width + 5.0).round() as i32;

                eprintln!("🔍 Icon center (logical): {}", icon_center_x);
                eprintln!("🔍 Window width: {}", window_width);
                eprintln!("🔍 Initial window_x (logical): {}", window_x);
                eprintln!("🔍 Initial window_y (logical): {}", window_y);

                // Convert monitor bounds to logical pixels
                let monitor_right =
                    monitor_position.x + (monitor_size.width as i32 / scale_factor as i32);
                let monitor_left = monitor_position.x;

                // Constrain window to monitor bounds (with 10px padding)
                if window_x < monitor_left {
                    eprintln!("⚠️ Window X too far left, constraining to monitor");
                    window_x = monitor_left + 10;
                } else if window_x + (window_width as i32) > monitor_right {
                    eprintln!("⚠️ Window X too far right, constraining to monitor");
                    window_x = monitor_right - (window_width as i32) - 10;
                }

                eprintln!("🔍 Final window_x (logical): {}", window_x);

                window
                    .set_position(tauri::Position::Logical(tauri::LogicalPosition {
                        x: window_x as f64,
                        y: window_y as f64,
                    }))
                    .map_err(|e| e.to_string())?;
            } else {
                eprintln!("⚠️ Could not get current monitor");
            }
        } else {
            // Fallback to old positioning if we don't have tray position yet
            eprintln!("⚠️ No tray position captured yet, using fallback");
            let settings = load_settings(&app).unwrap_or_default();
            if let Ok(Some(monitor)) = window.current_monitor() {
                let monitor_size = monitor.size();
                let (x, y) = calculate_window_position(&settings, &monitor_size);
                window
                    .set_position(tauri::Position::Physical(PhysicalPosition { x, y }))
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn position_window_centered(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(Some(monitor)) = window.current_monitor() {
            let monitor_frame = monitor.position();
            let monitor_size = monitor.size();
            let scale_factor = monitor.scale_factor();

            let window_width = 380.0;
            let window_height = 500.0;

            // Calculate center position in logical pixels
            let center_x = monitor_frame.x as f64 + (monitor_size.width as f64 / scale_factor) / 2.0 - (window_width / 2.0);
            let center_y = monitor_frame.y as f64 + (monitor_size.height as f64 / scale_factor) / 2.0 - (window_height / 2.0);

            eprintln!("📍 Centering window at ({}, {})", center_x, center_y);

            window
                .set_position(tauri::Position::Logical(tauri::LogicalPosition {
                    x: center_x,
                    y: center_y,
                }))
                .map_err(|e| e.to_string())?;
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
fn list_session_entries(
    app: AppHandle,
    start_time_iso: String,
) -> Result<Vec<logs::SessionEntry>, String> {
    use chrono::DateTime;

    // Parse the ISO timestamp
    let start_time = DateTime::parse_from_rfc3339(&start_time_iso)
        .map_err(|e| format!("Invalid timestamp format: {}", e))?
        .with_timezone(&chrono::Utc);

    // Read entries since start time
    logs::read_since(&app, start_time)
}

#[tauri::command]
fn hide_window(window: tauri::WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            tray_position: Mutex::new(None),
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                    #[cfg(target_os = "macos")]
                    let _ = app.show();
                }
            }
            "settings" => {
                let _ = open_settings(app.clone());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        // Add this block to prevent the app from quitting on window close
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                eprintln!("🏃 Window close requested, hiding window to keep app alive.");
                api.prevent_close();
                window.hide().unwrap();
            }
            _ => {}
        })
        .setup(|app| {
            // Use Regular activation policy to show in Dock
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(ActivationPolicy::Regular);
            }

            let settings = load_settings(app.handle()).unwrap_or_default();
            let initial_time = format!("{}:00", settings.check_in_interval);

            // Create context menu items
            let show_i = MenuItem::with_id(app, "show", "Show Timer", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let _menu = Menu::with_items(
                app,
                &[
                    &show_i,
                    &settings_i,
                    &PredefinedMenuItem::separator(app)?,
                    &quit_i,
                ],
            )?;

            eprintln!("🛠️ Building tray icon...");
            // Load smaller tray icon (22x22) - closer to v1's 18x18
            let icon_bytes = include_bytes!("../icons/tray-22x22.png");
            let image = image::load_from_memory(icon_bytes).map_err(|e| e.to_string())?.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();

            let icon = Image::new(&rgba, width, height);
            eprintln!("✅ Icon loaded successfully ({}x{})", width, height);

            // Build tray with icon and menu attached
            eprintln!("🔨 Building TrayIconBuilder...");
            let tray_result = TrayIconBuilder::with_id("main")
                .tooltip("Hyper Awareness")
                .title(&initial_time)
                .icon(icon)
                .icon_as_template(true) // Use template mode like working v1 version
                .menu(&_menu) // Attach the menu to the tray
                .show_menu_on_left_click(false) // Prevent menu from opening on left click
                .on_tray_icon_event(move |tray, event| {
                    eprintln!("🖱️ Tray event received: {:?}", event);
                    let app = tray.app_handle();
                    let state = app.state::<AppState>();

                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            position,
                            ..
                        } => {
                            eprintln!("✅ Left click detected at position: {:?}", position);
                            *state.tray_position.lock().unwrap() = Some(TrayPosition {
                                x: position.x,
                                y: position.y,
                                _width: 22.0,
                                _height: 22.0,
                            });

                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    // CRITICAL: Unminimize and activate app for proper window focus in builds
                                    let _ = window.unminimize();
                                    let _ = position_window_at_top(app.clone());
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    // Force app to foreground on macOS
                                    #[cfg(target_os = "macos")]
                                    let _ = app.show();
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .build(app);

            match tray_result {
                Ok(_) => {
                    eprintln!("✅ Tray icon BUILT successfully - should now be visible in menu bar");
                    eprintln!("📍 If you don't see it, check: 1) App is still running  2) Menu bar is not full  3) System tray settings");
                }
                Err(e) => {
                    eprintln!("❌ TRAY BUILD FAILED: {:?}", e);
                    return Err(Box::new(e));
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            open_settings,
            update_tray_timer,
            position_window_at_top,
            position_window_centered,
            hide_window,
            log_check_in,
            get_current_event,
            request_calendar_permission,
            list_session_entries
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Reopen { .. } => {
                eprintln!("🚀 Dock icon clicked (Reopen event)");
                if let Some(window) = app_handle.get_webview_window("main") {
                    if let Ok(visible) = window.is_visible() {
                        eprintln!("📊 Window visible status: {}", visible);
                    }
                    let _ = window.unminimize();
                    let _ = position_window_centered(app_handle.clone());
                    let _ = window.show();
                    let _ = window.set_focus();
                    #[cfg(target_os = "macos")]
                    let _ = app_handle.show();
                    eprintln!("✅ Window should now be visible and focused");
                }
            }
            _ => {}
        });
}
