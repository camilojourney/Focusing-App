use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};

/// Internal log entry structure (matches JSONL format).
///
/// The backend parses and serializes every entry so JSONL has a single trusted
/// validation boundary before data reaches the append-only journal.
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
    auto_submitted: Option<bool>,
    focus_shield_active: Option<bool>,
}

/// Session entry returned to frontend (cleaned up).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub timestamp: String,
    pub status: String,
    #[serde(rename = "statusLabel")]
    pub status_label: String,
    pub note: String,
}

/// Metadata-only journal health information. No activity content is exposed.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogDiagnostics {
    pub valid_records: usize,
    pub malformed_records: usize,
    pub has_unterminated_tail: bool,
}

impl SessionEntry {
    /// Convert reported_status to display label with emoji.
    fn status_to_label(status: &str) -> String {
        match status {
            "On Task" => "✅ On Task".to_string(),
            "Social Media" => "📱 Social Media".to_string(),
            "Email/Chat" => "📧 Email/Chat".to_string(),
            "Other Distraction" => "🔀 Other Distraction".to_string(),
            "Taking a Break" => "☕️ Taking a Break".to_string(),
            _ => status.to_string(),
        }
    }

    fn from_log_entry(entry: LogEntry) -> Self {
        Self {
            timestamp: entry.timestamp,
            status: entry.reported_status.clone(),
            status_label: Self::status_to_label(&entry.reported_status),
            note: entry.notes.unwrap_or_default(),
        }
    }
}

/// Append a frontend log payload only after validating and normalizing it.
pub fn append_entry(app: &AppHandle, log_line: &str) -> Result<(), String> {
    append_entry_to_path(&log_file_path(app)?, log_line)
}

/// Append a serialized entry to a JSONL file without joining it to an interrupted tail.
pub fn append_entry_to_path(path: &Path, log_line: &str) -> Result<(), String> {
    let entry: LogEntry =
        serde_json::from_str(log_line).map_err(|e| format!("Check-in is not valid JSON: {e}"))?;
    DateTime::parse_from_rfc3339(&entry.timestamp)
        .map_err(|e| format!("Check-in timestamp is not RFC3339: {e}"))?;
    let serialized =
        serde_json::to_string(&entry).map_err(|e| format!("Failed to serialize check-in: {e}"))?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create focus log directory: {e}"))?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("Failed to open focus log: {e}"))?;

    if file
        .metadata()
        .map_err(|e| format!("Failed to inspect focus log: {e}"))?
        .len()
        > 0
    {
        file.seek(SeekFrom::End(-1))
            .map_err(|e| format!("Failed to inspect focus log tail: {e}"))?;
        let mut tail = [0_u8; 1];
        file.read_exact(&mut tail)
            .map_err(|e| format!("Failed to read focus log tail: {e}"))?;
        if tail[0] != b'\n' {
            eprintln!("Focus log has an unterminated trailing record; restoring its line boundary before append");
            file.write_all(b"\n")
                .map_err(|e| format!("Failed to restore focus log line boundary: {e}"))?;
        }
    }

    file.write_all(serialized.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.sync_data())
        .map_err(|e| format!("Failed to append check-in: {e}"))
}

/// Return metadata-only health information without exposing journal contents.
pub fn diagnostics(app: &AppHandle) -> Result<LogDiagnostics, String> {
    diagnostics_for_path(&log_file_path(app)?)
}

pub fn diagnostics_for_path(path: &Path) -> Result<LogDiagnostics, String> {
    if !path.exists() {
        return Ok(LogDiagnostics {
            valid_records: 0,
            malformed_records: 0,
            has_unterminated_tail: false,
        });
    }

    let data =
        std::fs::read(path).map_err(|e| format!("Failed to read focus log diagnostics: {e}"))?;
    let has_unterminated_tail = !data.is_empty() && !data.ends_with(b"\n");
    let mut valid_records = 0;
    let mut malformed_records = 0;

    for line in data
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
    {
        let valid_entry = serde_json::from_slice::<LogEntry>(line)
            .ok()
            .and_then(|entry| DateTime::parse_from_rfc3339(&entry.timestamp).ok());
        if valid_entry.is_some() {
            valid_records += 1;
        } else {
            malformed_records += 1;
        }
    }

    Ok(LogDiagnostics {
        valid_records,
        malformed_records,
        has_unterminated_tail,
    })
}

/// Read session entries since a given start time.
pub fn read_since(app: &AppHandle, start: DateTime<Utc>) -> Result<Vec<SessionEntry>, String> {
    let log_path = log_file_path(app)?;

    if !log_path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(&log_path).map_err(|e| format!("Failed to open log file: {e}"))?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Failed to read focus log line {}: {error}", line_num + 1);
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let log_entry: LogEntry = match serde_json::from_str(&line) {
            Ok(entry) => entry,
            Err(error) => {
                eprintln!("Failed to parse focus log line {}: {error}", line_num + 1);
                continue;
            }
        };

        let entry_time = match DateTime::parse_from_rfc3339(&log_entry.timestamp) {
            Ok(time) => time.with_timezone(&Utc),
            Err(error) => {
                eprintln!(
                    "Failed to parse focus log timestamp on line {}: {error}",
                    line_num + 1
                );
                continue;
            }
        };

        if entry_time >= start {
            entries.push(SessionEntry::from_log_entry(log_entry));
        }
    }

    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Ok(entries)
}

fn log_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path.join("focus_log.jsonl"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temporary_path(name: &str) -> PathBuf {
        let counter = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "hyper-awareness-{name}-{}-{counter}.jsonl",
            std::process::id()
        ))
    }

    fn entry(timestamp: &str, status: &str) -> String {
        serde_json::json!({
            "timestamp": timestamp,
            "session_goal": "Synthetic test goal",
            "reported_status": status,
            "notes": "Synthetic test note",
            "session_duration_setting": 720,
            "check_in_interval_setting": 20,
            "write_time_setting": 20,
            "check_in_number": 1
        })
        .to_string()
    }

    #[test]
    fn status_to_label() {
        assert_eq!(SessionEntry::status_to_label("On Task"), "✅ On Task");
        assert_eq!(
            SessionEntry::status_to_label("Social Media"),
            "📱 Social Media"
        );
        assert_eq!(SessionEntry::status_to_label("Unknown"), "Unknown");
    }

    #[test]
    fn log_entry_conversion() {
        let log_entry = LogEntry {
            timestamp: "2025-11-13T10:00:00Z".to_string(),
            session_goal: Some("Synthetic test goal".to_string()),
            reported_status: "On Task".to_string(),
            notes: Some("Synthetic test note".to_string()),
            session_duration_setting: Some(720),
            check_in_interval_setting: Some(20),
            write_time_setting: Some(20),
            check_in_number: Some(1),
            auto_submitted: None,
            focus_shield_active: None,
        };

        let session_entry = SessionEntry::from_log_entry(log_entry);

        assert_eq!(session_entry.timestamp, "2025-11-13T10:00:00Z");
        assert_eq!(session_entry.status, "On Task");
        assert_eq!(session_entry.status_label, "✅ On Task");
        assert_eq!(session_entry.note, "Synthetic test note");
    }

    #[test]
    fn diagnostics_count_invalid_timestamps_as_malformed_records() {
        let path = temporary_path("invalid-timestamp");
        std::fs::write(&path, entry("not-a-timestamp", "On Task")).unwrap();

        let diagnostics = diagnostics_for_path(&path).unwrap();

        assert_eq!(diagnostics.valid_records, 0);
        assert_eq!(diagnostics.malformed_records, 1);
        assert!(diagnostics.has_unterminated_tail);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn append_after_unterminated_tail_keeps_prior_valid_records_and_new_record() {
        let path = temporary_path("malformed-tail");
        let first = entry("2025-11-13T10:00:00Z", "On Task");
        let malformed_tail = b"{\"interrupted\":";
        let mut interrupted_file = format!("{first}\n").into_bytes();
        interrupted_file.extend_from_slice(malformed_tail);
        std::fs::write(&path, interrupted_file).unwrap();

        append_entry_to_path(&path, &entry("2025-11-13T10:20:00Z", "Taking a Break")).unwrap();

        let diagnostics = diagnostics_for_path(&path).unwrap();
        assert_eq!(diagnostics.valid_records, 2);
        assert_eq!(diagnostics.malformed_records, 1);
        assert!(!diagnostics.has_unterminated_tail);

        let contents = std::fs::read_to_string(&path).unwrap();
        let valid_statuses: Vec<String> = contents
            .lines()
            .filter_map(|line| serde_json::from_str::<LogEntry>(line).ok())
            .map(|entry| entry.reported_status)
            .collect();
        assert_eq!(valid_statuses, vec!["On Task", "Taking a Break"]);

        std::fs::remove_file(path).unwrap();
    }
}
