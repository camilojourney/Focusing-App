export const DEFAULT_TIMER_SETTINGS = Object.freeze({
    sessionDuration: 720,
    checkInInterval: 20,
    writeTime: 20
});

function remainingAt(deadline, currentRemaining, now) {
    return deadline
        ? Math.max(0, Math.ceil((deadline - now) / 1000))
        : Math.max(0, Math.round(currentRemaining));
}

export function captureTimerRemainders(timers, now = Date.now()) {
    return {
        sessionTimeRemaining: remainingAt(
            timers.sessionEndTimestamp,
            timers.sessionTimeRemaining,
            now
        ),
        checkInTimeRemaining: remainingAt(
            timers.checkInEndTimestamp,
            timers.checkInTimeRemaining,
            now
        ),
        writeTimeRemaining: remainingAt(
            timers.writeEndTimestamp,
            timers.writeTimeRemaining,
            now
        )
    };
}

export function resumeTimerDeadlines(remainders, now = Date.now()) {
    return {
        sessionEndTimestamp: now + Math.max(0, remainders.sessionTimeRemaining) * 1000,
        checkInEndTimestamp: now + Math.max(0, remainders.checkInTimeRemaining) * 1000
    };
}

export function applySettingsUpdate(currentSettings, payload, timers, isSessionRunning, now = Date.now()) {
    const settings = {
        sessionDuration: payload.session_duration || currentSettings.sessionDuration,
        checkInInterval: payload.check_in_interval || currentSettings.checkInInterval,
        writeTime: payload.write_time || currentSettings.writeTime
    };
    const remainders = captureTimerRemainders(timers, now);
    const deadlines = isSessionRunning ? resumeTimerDeadlines(remainders, now) : {};

    return { settings, remainders, deadlines };
}

export function recoveredSessionSnapshot(state, now = Date.now()) {
    const settings = {
        sessionDuration: state.sessionDuration,
        checkInInterval: state.checkInInterval,
        writeTime: state.writeTime
    };
    const focusShieldActive = Boolean(state.focusShieldActive && state.focusShieldUntil > now);

    return {
        settings,
        sessionTimeRemaining: state.sessionTimeRemaining,
        checkInTimeRemaining: state.checkInTimeRemaining,
        writeTimeRemaining: state.writeTimeRemaining,
        checkInsCompleted: state.checkInsCompleted,
        skippedCheckIns: state.skippedCheckIns,
        lastCheckInWasSkipped: state.lastCheckInWasSkipped,
        focusShieldActive,
        focusShieldUntil: focusShieldActive ? state.focusShieldUntil : null,
        sessionStartedAt: state.sessionStartedAt,
        sessionGoal: state.sessionGoal || '',
        statusMessage: state.phase === 'interrupted'
            ? 'Session interrupted by restart - review and resume'
            : 'Session paused - resume when ready'
    };
}
