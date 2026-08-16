use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};

const ACTIVE_SESSION_STATE_VERSION: u32 = 1;
const ACTIVE_SESSION_FILE_NAME: &str = "active_session.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SessionPhase {
    Active,
    Writing,
    Paused,
    Interrupted,
}

impl SessionPhase {
    fn needs_restart_recovery(&self) -> bool {
        matches!(self, Self::Active | Self::Writing)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveSessionState {
    pub version: u32,
    pub phase: SessionPhase,
    pub session_goal: String,
    pub session_started_at: Option<i64>,
    pub session_duration: u32,
    pub check_in_interval: u32,
    pub write_time: u32,
    pub session_time_remaining: u64,
    pub check_in_time_remaining: u64,
    pub write_time_remaining: u64,
    pub check_ins_completed: u32,
    pub skipped_check_ins: u32,
    pub last_check_in_was_skipped: bool,
    pub focus_shield_active: bool,
    pub focus_shield_until: Option<i64>,
    pub recovery_reason: Option<String>,
}

impl ActiveSessionState {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != ACTIVE_SESSION_STATE_VERSION {
            return Err(format!(
                "Unsupported active session state version: {}",
                self.version
            ));
        }
        if self.session_duration == 0 || self.check_in_interval == 0 || self.write_time == 0 {
            return Err("Active session state contains an invalid timer setting".to_string());
        }
        Ok(())
    }
}

pub fn state_path(app: &AppHandle) -> Result<PathBuf, String> {
    let path = app.path().app_config_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path.join(ACTIVE_SESSION_FILE_NAME))
}

pub fn save(app: &AppHandle, state: ActiveSessionState) -> Result<(), String> {
    save_to_path(&state_path(app)?, &state)
}

pub fn recover(app: &AppHandle) -> Result<Option<ActiveSessionState>, String> {
    recover_from_path(&state_path(app)?)
}

pub fn clear(app: &AppHandle) -> Result<(), String> {
    let path = state_path(app)?;
    if path.exists() {
        fs::remove_file(path).map_err(|e| format!("Failed to clear active session state: {e}"))?;
    }
    Ok(())
}

pub fn save_to_path(path: &Path, state: &ActiveSessionState) -> Result<(), String> {
    state.validate()?;
    let serialized = serde_json::to_vec_pretty(state)
        .map_err(|e| format!("Failed to serialize active session state: {e}"))?;
    let parent = path
        .parent()
        .ok_or_else(|| "Active session state path has no parent directory".to_string())?;
    fs::create_dir_all(parent)
        .map_err(|e| format!("Failed to create active session state directory: {e}"))?;

    let temporary_path = path.with_extension(format!("tmp-{}", unique_suffix()));
    let result = (|| -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
            .map_err(|e| format!("Failed to create active session state file: {e}"))?;
        file.write_all(&serialized)
            .map_err(|e| format!("Failed to write active session state: {e}"))?;
        file.sync_all()
            .map_err(|e| format!("Failed to sync active session state: {e}"))?;
        fs::rename(&temporary_path, path)
            .map_err(|e| format!("Failed to finalize active session state: {e}"))?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }
    result
}

pub fn recover_from_path(path: &Path) -> Result<Option<ActiveSessionState>, String> {
    if !path.exists() {
        return Ok(None);
    }

    let data = fs::read(path).map_err(|e| format!("Failed to read active session state: {e}"))?;
    let mut state: ActiveSessionState = serde_json::from_slice(&data)
        .map_err(|e| format!("Failed to parse active session state: {e}"))?;
    state.validate()?;

    if state.phase.needs_restart_recovery() {
        state.phase = SessionPhase::Interrupted;
        state.recovery_reason =
            Some("Application restarted while this session was active".to_string());
        save_to_path(path, &state)?;
    }

    Ok(Some(state))
}

fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temporary_path(name: &str) -> PathBuf {
        let counter = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "hyper-awareness-{name}-{}-{counter}.json",
            std::process::id()
        ))
    }

    fn active_state() -> ActiveSessionState {
        ActiveSessionState {
            version: ACTIVE_SESSION_STATE_VERSION,
            phase: SessionPhase::Active,
            session_goal: "Synthetic test goal".to_string(),
            session_started_at: Some(1_700_000_000_000),
            session_duration: 720,
            check_in_interval: 20,
            write_time: 20,
            session_time_remaining: 42_000,
            check_in_time_remaining: 1_200,
            write_time_remaining: 0,
            check_ins_completed: 3,
            skipped_check_ins: 1,
            last_check_in_was_skipped: true,
            focus_shield_active: false,
            focus_shield_until: None,
            recovery_reason: None,
        }
    }

    #[test]
    fn restart_recovery_marks_active_session_interrupted_without_resetting_it() {
        let path = temporary_path("restart-recovery");
        let original = active_state();
        save_to_path(&path, &original).unwrap();

        let recovered = recover_from_path(&path).unwrap().unwrap();

        assert_eq!(recovered.phase, SessionPhase::Interrupted);
        assert_eq!(recovered.session_goal, original.session_goal);
        assert_eq!(
            recovered.session_time_remaining,
            original.session_time_remaining
        );
        assert_eq!(
            recovered.check_in_time_remaining,
            original.check_in_time_remaining
        );
        assert_eq!(recovered.check_ins_completed, original.check_ins_completed);
        assert!(recovered.recovery_reason.is_some());
        assert_eq!(recover_from_path(&path).unwrap().unwrap(), recovered);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn paused_session_is_preserved_without_being_marked_interrupted() {
        let path = temporary_path("paused-recovery");
        let mut paused = active_state();
        paused.phase = SessionPhase::Paused;
        save_to_path(&path, &paused).unwrap();

        assert_eq!(recover_from_path(&path).unwrap(), Some(paused));

        fs::remove_file(path).unwrap();
    }
}
