#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod calendar;

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tauri::{
    api::path::app_config_dir,
    AppHandle,
    CustomMenuItem,
    Manager,
    SystemTray,
    SystemTrayEvent,
    SystemTrayMenu,
    SystemTrayMenuItem,
    WindowBuilder,
    WindowUrl,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    session_duration: u32,
    check_in_interval: u32,
    write_time: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            session_duration: 720,
            check_in_interval: 15,
            write_time: 25,
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

            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                // Show main window when clicking the tray icon
                if let Some(window) = app.get_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "show" => {
                        if let Some(window) = app.get_window("main") {
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
            SystemTrayEvent::DoubleClick { .. } => {
                // Open main window on double click
                if let Some(window) = app.get_window("main") {
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
            log_check_in,
            get_current_event,
            request_calendar_permission
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
