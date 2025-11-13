use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use tauri::AppHandle;

/// Internal log entry structure (matches JSONL format)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogEntry {
    timestamp: String,
    session_goal: Option<String>,
    reported_status: String,
    notes: Option<String>,
    session_duration_setting: Option<u32>,
    check_in_interval_setting: Option<u32>,
    write_time_setting: Option<u32>,
    check_in_number: Option<u32>,
}

/// Session entry returned to frontend (cleaned up)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub timestamp: String,
    pub status: String,
    #[serde(rename = "statusLabel")]
    pub status_label: String,
    pub note: String,
}

impl SessionEntry {
    /// Convert reported_status to display label with emoji
    fn status_to_label(status: &str) -> String {
        match status {
            "On Task" => "✅ On Task".to_string(),
            "Social Media" => "📱 Social Media".to_string(),
            "Email/Chat" => "📧 Email/Chat".to_string(),
            "Other Distraction" => "🔀 Other Distraction".to_string(),
            "Taking a Break" => "☕️ Taking a Break".to_string(),
            _ => status.to_string(), // Fallback for unknown statuses
        }
    }

    fn from_log_entry(entry: LogEntry) -> Self {
        SessionEntry {
            timestamp: entry.timestamp,
            status: entry.reported_status.clone(),
            status_label: Self::status_to_label(&entry.reported_status),
            note: entry.notes.unwrap_or_default(),
        }
    }
}

/// Read session entries since a given start time
pub fn read_since(app: &AppHandle, start: DateTime<Utc>) -> Result<Vec<SessionEntry>, String> {
    let log_path = log_file_path(app)?;

    // If log file doesn't exist, return empty vec (not an error)
    if !log_path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(&log_path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to read line {}: {}", line_num + 1, e);
                continue; // Skip malformed lines
            }
        };

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse JSON
        let log_entry: LogEntry = match serde_json::from_str(&line) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to parse line {}: {}", line_num + 1, e);
                continue; // Skip malformed JSON
            }
        };

        // Parse timestamp
        let entry_time = match DateTime::parse_from_rfc3339(&log_entry.timestamp) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(e) => {
                eprintln!(
                    "Failed to parse timestamp on line {}: {}",
                    line_num + 1,
                    e
                );
                continue; // Skip entries with invalid timestamps
            }
        };

        // Filter by start time
        if entry_time >= start {
            entries.push(SessionEntry::from_log_entry(log_entry));
        }
    }

    // Sort by timestamp (ascending - oldest first)
    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    Ok(entries)
}

/// Helper to get log file path (reuse from main.rs)
fn log_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    use tauri::api::path::app_config_dir;
    let mut path = app_config_dir(&app.config())
        .ok_or("Unable to determine app config directory")?;
    std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("focus_log.jsonl");
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_to_label() {
        assert_eq!(
            SessionEntry::status_to_label("On Task"),
            "✅ On Task"
        );
        assert_eq!(
            SessionEntry::status_to_label("Social Media"),
            "📱 Social Media"
        );
        assert_eq!(
            SessionEntry::status_to_label("Unknown"),
            "Unknown"
        );
    }

    #[test]
    fn test_log_entry_conversion() {
        let log_entry = LogEntry {
            timestamp: "2025-11-13T10:00:00Z".to_string(),
            session_goal: Some("Test goal".to_string()),
            reported_status: "On Task".to_string(),
            notes: Some("Test note".to_string()),
            session_duration_setting: Some(720),
            check_in_interval_setting: Some(15),
            write_time_setting: Some(20),
            check_in_number: Some(1),
        };

        let session_entry = SessionEntry::from_log_entry(log_entry);

        assert_eq!(session_entry.timestamp, "2025-11-13T10:00:00Z");
        assert_eq!(session_entry.status, "On Task");
        assert_eq!(session_entry.status_label, "✅ On Task");
        assert_eq!(session_entry.note, "Test note");
    }
}
