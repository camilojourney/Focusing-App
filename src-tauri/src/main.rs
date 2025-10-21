#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

use serde::{Deserialize, Serialize};
use tauri::{
    api::path::app_config_dir,
    AppHandle,
    CustomMenuItem,
    Icon,
    Manager,
    WindowBuilder,
    WindowUrl,
};

use tauri::{SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

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
            write_time: 20,
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

fn accountability_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut path = app_config_dir(&app.config())
        .ok_or("Unable to determine app config directory")?;
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("accountability_box.jsonl");
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
    .inner_size(400.0, 500.0)
    .resizable(false)
    .decorations(false)
    .visible(true)
    .always_on_top(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn switch_desktop() -> Result<(), String> {
    let script = r#"
        tell application "System Events"
            key code 18 using {command down}
        end tell
    "#;

    let status = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("osascript failed".to_string())
    }
}

#[tauri::command]
fn update_tray_timer(app: AppHandle, timer_text: String) -> Result<(), String> {
    let tray_handle = app.tray_handle();
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
fn save_accountability(app: AppHandle, answers: String) -> Result<(), String> {
    let path = accountability_file_path(&app)?;

    // Open file in append mode (create if doesn't exist)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    // Write the JSON line with a newline
    writeln!(file, "{}", answers).map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    // Create system tray menu
    let timer_item = CustomMenuItem::new("timer", "🧠 15:00").disabled();
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

    // Use a proper tray icon
    let system_tray = SystemTray::new()
        .with_icon(Icon::Raw(include_bytes!("../icons/32x32.png").to_vec()))
        .with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
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
                            .inner_size(400.0, 500.0)
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
            SystemTrayEvent::LeftClick { .. } => {
                // Show/hide window on left click
                if let Some(window) = app.get_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            switch_desktop,
            open_settings,
            update_tray_timer,
            log_check_in,
            save_accountability
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
