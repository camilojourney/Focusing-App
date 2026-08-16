import test from 'node:test';
import assert from 'node:assert/strict';
import {
    DEFAULT_TIMER_SETTINGS,
    applySettingsUpdate,
    captureTimerRemainders,
    recoveredSessionSnapshot,
    resumeTimerDeadlines
} from '../src/js/timer-state.mjs';

test('timer transition captures elapsed time and resumes from the preserved remainders', () => {
    const pausedAt = 1_000_000;
    const remainders = captureTimerRemainders({
        sessionTimeRemaining: 900,
        checkInTimeRemaining: 300,
        writeTimeRemaining: 0,
        sessionEndTimestamp: pausedAt + 899_100,
        checkInEndTimestamp: pausedAt + 299_100,
        writeEndTimestamp: null
    }, pausedAt);

    assert.deepEqual(remainders, {
        sessionTimeRemaining: 900,
        checkInTimeRemaining: 300,
        writeTimeRemaining: 0
    });
    assert.deepEqual(resumeTimerDeadlines(remainders, pausedAt), {
        sessionEndTimestamp: pausedAt + 900_000,
        checkInEndTimestamp: pausedAt + 300_000
    });
});

test('user settings remain configurable and preserve active timer remainders', () => {
    const now = 2_000_000;
    const result = applySettingsUpdate(
        DEFAULT_TIMER_SETTINGS,
        { session_duration: 90, check_in_interval: 25, write_time: 35 },
        {
            sessionTimeRemaining: 1_200,
            checkInTimeRemaining: 240,
            writeTimeRemaining: 0,
            sessionEndTimestamp: now + 1_199_500,
            checkInEndTimestamp: now + 239_500,
            writeEndTimestamp: null
        },
        true,
        now
    );

    assert.deepEqual(result.settings, {
        sessionDuration: 90,
        checkInInterval: 25,
        writeTime: 35
    });
    assert.deepEqual(result.remainders, {
        sessionTimeRemaining: 1_200,
        checkInTimeRemaining: 240,
        writeTimeRemaining: 0
    });
    assert.deepEqual(result.deadlines, {
        sessionEndTimestamp: now + 1_200_000,
        checkInEndTimestamp: now + 240_000
    });
});

test('restart recovery presents an explicit interruption without resetting saved timer state', () => {
    const recovered = recoveredSessionSnapshot({
        phase: 'interrupted',
        sessionGoal: 'Synthetic test goal',
        sessionStartedAt: 3_000_000,
        sessionDuration: 720,
        checkInInterval: 20,
        writeTime: 20,
        sessionTimeRemaining: 42_000,
        checkInTimeRemaining: 1_200,
        writeTimeRemaining: 0,
        checkInsCompleted: 3,
        skippedCheckIns: 1,
        lastCheckInWasSkipped: true,
        focusShieldActive: false,
        focusShieldUntil: null
    }, 4_000_000);

    assert.equal(recovered.statusMessage, 'Session interrupted by restart - review and resume');
    assert.equal(recovered.sessionTimeRemaining, 42_000);
    assert.equal(recovered.checkInTimeRemaining, 1_200);
    assert.equal(recovered.checkInsCompleted, 3);
});
