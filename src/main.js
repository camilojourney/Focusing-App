import { waitForTauriBridge } from './tauri-bridge.js';
import './js/sessionReview.js';

// Create shortcuts for convenience (assigned after Tauri bridge is ready)
let invoke;
let listen;
let emit;
let appWindow;
const dom = {};

let settings = {
    sessionDuration: 720,
    checkInInterval: 15,
    writeTime: 20
};

let sessionTimeRemaining = settings.sessionDuration * 60;
let checkInTimeRemaining = settings.checkInInterval * 60;
let writeTimeRemaining = 0;

let isSessionRunning = false;
let isWriting = false;
let checkInsCompleted = 0;
let isUsingCalendarEvent = false;

let sessionEndTimestamp = null;
let checkInEndTimestamp = null;
let writeEndTimestamp = null;
let tickInterval = null;
let lastTickTimestamp = null;
let calendarRefreshInterval = null;

let focusShieldActive = false;
let focusShieldUntil = null;
let statusOverride = null;
let skippedCheckIns = 0;
let lastCheckInWasSkipped = false;

const TICK_RATE_MS = 1000;
const SLEEP_THRESHOLD_MS = 60_000;
const FOCUS_SHIELD_EXTENSION_MINUTES = 15;

async function ensureTauriReady() {
    if (invoke && listen && emit && appWindow) {
        return;
    }

    await waitForTauriBridge();

    invoke = window.Tauri?.invoke;
    listen = window.Tauri?.listen;
    emit = window.Tauri?.emit;
    appWindow = window.Tauri?.appWindow;

    if (!invoke || !listen || !appWindow) {
        throw new Error('Tauri APIs are not available. Run inside the Tauri shell.');
    }
}

async function hideMainWindow(context = '') {
    const contextLabel = context ? ` (${context})` : '';
    try {
        await ensureTauriReady();
        const win = appWindow || window.Tauri?.appWindow;
        if (win?.hide) {
            await win.hide();
            return;
        }
    } catch (error) {
        console.error(`❌ Failed to hide window via JS${contextLabel}:`, error);
    }

    try {
        await invoke('hide_window');
    } catch (fallbackError) {
        console.error(`❌ hide_window fallback failed${contextLabel}:`, fallbackError);
    }
}

function cacheDom() {
    dom.timerLabel = document.getElementById('timerLabel');
    dom.timer = document.getElementById('timer');
    dom.status = document.getElementById('status');
    dom.sessionProgress = document.getElementById('sessionProgress');
    dom.checkIns = document.getElementById('checkIns');
    dom.startBtn = document.getElementById('startBtn');
    dom.resetBtn = document.getElementById('resetBtn');
    dom.settingsBtn = document.getElementById('settingsBtn');
    dom.testBtn = document.getElementById('testBtn');
    dom.calendarBtn = document.getElementById('calendarBtn');
    dom.sessionGoal = document.getElementById('sessionGoal');
    dom.mainScreen = document.getElementById('mainScreen');
    dom.checkInScreen = document.getElementById('checkInScreen');
    dom.checkInGoalText = document.getElementById('checkInGoalText');
    dom.checkInNotes = document.getElementById('checkInNotes');
    dom.checkInCountdown = document.getElementById('checkInCountdown');
    dom.focusShieldBtn = document.getElementById('focusShieldBtn');
    dom.focusShieldBanner = document.getElementById('focusShieldBanner');
    dom.focusShieldText = document.getElementById('focusShieldText');
    dom.focusShieldCancelBtn = document.getElementById('focusShieldCancelBtn');
    dom.winClose = document.getElementById('winClose');
    dom.winMinimize = document.getElementById('winMinimize');
}

function formatTime(seconds) {
    const safeSeconds = Math.max(0, Math.round(seconds));
    const mins = Math.floor(safeSeconds / 60);
    const secs = safeSeconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
}

function formatClock(timestamp) {
    return new Date(timestamp).toLocaleTimeString([], { hour: 'numeric', minute: '2-digit' });
}

async function loadSettings() {
    try {
        const loaded = await invoke('get_settings');
        settings = {
            sessionDuration: loaded.session_duration || settings.sessionDuration,
            checkInInterval: loaded.check_in_interval || settings.checkInInterval,
            writeTime: loaded.write_time || settings.writeTime
        };
        console.log('Settings loaded:', settings);

        if (!isSessionRunning && !isWriting) {
            sessionTimeRemaining = settings.sessionDuration * 60;
            checkInTimeRemaining = settings.checkInInterval * 60;
        }
    } catch (error) {
        console.error('Failed to load settings:', error);
    }
}

function captureRemainingTimes(now = Date.now()) {
    if (sessionEndTimestamp) {
        sessionTimeRemaining = Math.max(0, Math.ceil((sessionEndTimestamp - now) / 1000));
    }
    if (checkInEndTimestamp) {
        checkInTimeRemaining = Math.max(0, Math.ceil((checkInEndTimestamp - now) / 1000));
    }
    if (writeEndTimestamp) {
        writeTimeRemaining = Math.max(0, Math.ceil((writeEndTimestamp - now) / 1000));
    }
}

function startTicking() {
    if (!tickInterval) {
        console.log('startTicking: Starting tick interval');
        lastTickTimestamp = Date.now();
        tickInterval = setInterval(() => {
            tick().catch((error) => console.error('Tick loop failed:', error));
            // Keep the backend alive by sending a ping every second
            invoke('keep_app_alive').catch(() => { }); // Silent fail, non-critical
        }, TICK_RATE_MS);
    } else {
        console.log('startTicking: Tick interval already running');
    }
}

function stopTickingIfIdle(force = false) {
    const shouldStop = force || (!isSessionRunning && !isWriting && !focusShieldActive);
    console.log('stopTickingIfIdle:', { force, isSessionRunning, isWriting, focusShieldActive, shouldStop, hasInterval: !!tickInterval });
    if (tickInterval && shouldStop) {
        console.log('stopTickingIfIdle: Clearing tick interval');
        clearInterval(tickInterval);
        tickInterval = null;
        lastTickTimestamp = null;
    }
}

function shouldDeferCheckIn(now) {
    return focusShieldActive && focusShieldUntil && now < focusShieldUntil;
}

function deferCheckIn(now) {
    const base = focusShieldUntil || now;
    checkInEndTimestamp = base + settings.checkInInterval * 60 * 1000;
    captureRemainingTimes(now);
    statusOverride = 'Check-in postponed';
    updateDisplay();
}

function handleSystemSleep(deltaMs) {
    console.log(`Large time gap detected: ${Math.round(deltaMs / 1000)}s delay`);
    // Only pause for very long gaps (5+ minutes) which indicate actual sleep
    // Short gaps are just window being hidden/throttled - don't pause for those
    if (deltaMs > 5 * 60 * 1000 && isSessionRunning) {
        pauseSession({ reason: 'sleep' });
        statusOverride = `Paused: Mac was asleep for ${Math.round(deltaMs / 60000)} min`;
        updateFocusShieldUi();
        captureRemainingTimes();
    }
    // For shorter gaps, just continue - the timestamp-based timers will catch up
}

async function tick() {
    const now = Date.now();
    if (lastTickTimestamp) {
        const delta = now - lastTickTimestamp;
        if (delta > SLEEP_THRESHOLD_MS) {
            handleSystemSleep(delta);
        }
    }
    lastTickTimestamp = now;

    if (focusShieldActive && focusShieldUntil && now >= focusShieldUntil) {
        focusShieldActive = false;
        focusShieldUntil = null;
        statusOverride = 'Focus Shield ended';
        updateFocusShieldUi();
    }

    if (isWriting && writeEndTimestamp) {
        writeTimeRemaining = Math.max(0, Math.ceil((writeEndTimestamp - now) / 1000));
        updateCheckInCountdown();
        if (writeTimeRemaining <= 0) {
            handleCheckInTimeout();
            return;
        }
    }

    if (isSessionRunning) {
        if (sessionEndTimestamp) {
            sessionTimeRemaining = Math.max(0, Math.ceil((sessionEndTimestamp - now) / 1000));
            if (sessionTimeRemaining <= 0) {
                endSession();
                return;
            }
        }

        if (checkInEndTimestamp) {
            checkInTimeRemaining = Math.max(0, Math.ceil((checkInEndTimestamp - now) / 1000));
            if (checkInTimeRemaining <= 0) {
                if (shouldDeferCheckIn(now)) {
                    deferCheckIn(now);
                } else {
                    await triggerCheckIn();
                }
                return;
            }
        }
    } else if (!isWriting) {
        captureRemainingTimes(now);
    }

    updateDisplay();
}

function updateFocusShieldUi() {
    if (!dom.focusShieldBanner) {
        return;
    }

    if (focusShieldActive && focusShieldUntil) {
        dom.focusShieldBanner.hidden = false;
        // Add glowing border to app shell when active
        document.querySelector('.app-shell').style.borderColor = 'rgba(255, 193, 7, 0.5)';
        if (dom.focusShieldText) {
            dom.focusShieldText.textContent = `Shield active until ${formatClock(focusShieldUntil)}`;
        }
        if (dom.focusShieldBtn) {
            dom.focusShieldBtn.textContent = '🛡️ Extend';
            dom.focusShieldBtn.style.color = '#ffc107';
        }
    } else {
        dom.focusShieldBanner.hidden = true;
        document.querySelector('.app-shell').style.borderColor = '';
        if (dom.focusShieldBtn) {
            dom.focusShieldBtn.textContent = '🛡️ Focus Shield';
            dom.focusShieldBtn.style.color = '';
        }
    }
}

async function updateDisplay() {
    updateFocusShieldUi();

    const timerLabelEl = dom.timerLabel;
    const timerEl = dom.timer;
    const statusEl = dom.status;

    let trayText = '';

    if (isWriting) {
        if (timerLabelEl) timerLabelEl.textContent = '✍️ LOG ACTIVITY';
        if (timerEl) timerEl.textContent = `${writeTimeRemaining}`;
        if (statusEl) statusEl.textContent = "Write what you're doing";
        trayText = `✍️ ${writeTimeRemaining}s`;
    } else {
        if (timerLabelEl) timerLabelEl.textContent = 'NEXT CHECK-IN';
        if (timerEl) timerEl.textContent = formatTime(checkInTimeRemaining);

        let statusText = isSessionRunning ? 'Session active' : 'Ready';
        if (focusShieldActive && focusShieldUntil) {
            const minutesLeft = Math.max(1, Math.ceil((focusShieldUntil - Date.now()) / 60000));
            statusText = `Focus Shield • ${minutesLeft} min`;
        }
        if (statusOverride) {
            statusText = statusOverride;
            statusOverride = null;
        }
        if (statusEl) statusEl.textContent = statusText;

        trayText = formatTime(checkInTimeRemaining);
    }

    const elapsedSeconds = Math.max(0, (settings.sessionDuration * 60) - sessionTimeRemaining);
    const sessionMinsElapsed = Math.floor(elapsedSeconds / 60);

    if (dom.sessionProgress) {
        dom.sessionProgress.textContent = `${sessionMinsElapsed}m / ${settings.sessionDuration}m`;
    }
    if (dom.checkIns) {
        let checkInText = `${checkInsCompleted}`;
        if (skippedCheckIns > 0) {
            checkInText += ` (${skippedCheckIns} skip)`;
        }
        dom.checkIns.textContent = checkInText;
    }

    if (lastCheckInWasSkipped) {
        trayText = '🔴 ' + trayText;
    }

    try {
        await invoke('update_tray_timer', { timerText: trayText });
    } catch (error) {
        console.error('Failed to update tray timer:', error);
    }
}

function toggleSession() {
    if (isSessionRunning) {
        pauseSession({ reason: 'user' });
    } else {
        startSession();
    }
}

async function startSession({ autoHide = true } = {}) {
    const now = Date.now();

    if (!isSessionRunning && !isWriting) {
        if (sessionTimeRemaining <= 0 || sessionTimeRemaining > settings.sessionDuration * 60) {
            sessionTimeRemaining = settings.sessionDuration * 60;
        }
        if (checkInTimeRemaining <= 0 || checkInTimeRemaining > settings.checkInInterval * 60) {
            checkInTimeRemaining = settings.checkInInterval * 60;
        }
        if (window.sessionReview) {
            window.sessionReview.setSessionStartTime(new Date());
        }
    }

    sessionEndTimestamp = now + sessionTimeRemaining * 1000;
    checkInEndTimestamp = now + checkInTimeRemaining * 1000;
    isSessionRunning = true;

    if (dom.startBtn) {
        dom.startBtn.textContent = 'Pause Focus';
        dom.startBtn.style.background = 'rgba(255,255,255,0.1)';
        dom.startBtn.style.color = '#fff';
    }

    startTicking();
    updateDisplay();

    // Hide window after starting - timer runs in background
    if (autoHide) {
        await hideMainWindow('start focus');
    }
} function pauseSession({ reason } = {}) {
    if (!isSessionRunning) return;

    captureRemainingTimes();
    isSessionRunning = false;
    sessionEndTimestamp = null;
    checkInEndTimestamp = null;

    if (dom.startBtn) {
        dom.startBtn.textContent = 'Resume Focus';
        dom.startBtn.style.background = 'white';
        dom.startBtn.style.color = 'black';
    }

    if (reason === 'user') statusOverride = 'Session paused';
    updateDisplay();
    stopTickingIfIdle();
}

function resetSession() {
    isSessionRunning = false;
    isWriting = false;
    checkInsCompleted = 0;
    skippedCheckIns = 0;
    lastCheckInWasSkipped = false;
    sessionTimeRemaining = settings.sessionDuration * 60;
    checkInTimeRemaining = settings.checkInInterval * 60;
    writeTimeRemaining = 0;
    sessionEndTimestamp = null;
    checkInEndTimestamp = null;
    writeEndTimestamp = null;
    focusShieldActive = false;
    focusShieldUntil = null;
    isUsingCalendarEvent = false;

    hideCheckInScreen();
    stopTickingIfIdle(true);
    stopCalendarAutoRefresh();
    updateCalendarButtonState();

    if (window.sessionReview) window.sessionReview.reset();

    if (dom.sessionGoal) dom.sessionGoal.placeholder = 'What are you trying to achieve?';
    if (dom.startBtn) {
        dom.startBtn.textContent = 'Start Focus';
        dom.startBtn.style.background = 'white';
        dom.startBtn.style.color = 'black';
    }

    statusOverride = 'Ready';
    updateDisplay();
}

async function triggerCheckIn({ forced = false } = {}) {
    console.log('triggerCheckIn: Starting check-in #' + (checkInsCompleted + 1));
    const now = Date.now();
    if (!forced && shouldDeferCheckIn(now)) {
        deferCheckIn(now);
        return;
    }

    captureRemainingTimes(now);

    // Set isWriting BEFORE pausing to prevent stopTickingIfIdle from killing the interval
    isWriting = true;
    console.log('triggerCheckIn: Set isWriting=true BEFORE pauseSession');
    pauseSession({ reason: 'check-in' });

    try {
        // Center the window for check-in prompts
        await invoke('position_window_centered');
        await appWindow.show();
        await appWindow.setFocus();
    } catch (error) {
        console.error('Failed to show window:', error);
    }

    showCheckInScreen();

    // isWriting already set to true above (before pauseSession)
    writeTimeRemaining = settings.writeTime;
    writeEndTimestamp = Date.now() + settings.writeTime * 1000;
    checkInsCompleted += 1;
    updateCheckInCountdown();

    startTicking();
    updateDisplay();
}

function showCheckInScreen() {
    const goalText = (dom.sessionGoal?.value || '').trim() || '(No specific goal)';
    if (dom.checkInGoalText) dom.checkInGoalText.textContent = `"${goalText}"`;
    if (dom.checkInNotes) dom.checkInNotes.value = '';

    dom.mainScreen?.classList.add('hidden');
    dom.checkInScreen?.classList.add('active');
    document.body.classList.add('check-in-active');
    updateCheckInCountdown();
}

function hideCheckInScreen() {
    dom.mainScreen?.classList.remove('hidden');
    dom.checkInScreen?.classList.remove('active');
    document.body.classList.remove('check-in-active');
}

function updateCheckInCountdown() {
    if (dom.checkInCountdown) dom.checkInCountdown.textContent = `Resuming session in ${writeTimeRemaining}s...`;
}

function handleCheckInTimeout() {
    skippedCheckIns++;
    lastCheckInWasSkipped = true;
    handleCheckInResponse('Skip', { auto: true });
}

async function handleCheckInResponse(status, options = {}) {
    if (!isWriting) return;
    if (!options.auto) lastCheckInWasSkipped = false;

    const logEntry = {
        timestamp: new Date().toISOString(),
        session_goal: dom.sessionGoal?.value || '',
        reported_status: status,
        notes: dom.checkInNotes?.value || '',
        session_duration_setting: settings.sessionDuration,
        check_in_interval_setting: settings.checkInInterval,
        write_time_setting: settings.writeTime,
        check_in_number: checkInsCompleted,
        auto_submitted: !!options.auto,
        focus_shield_active: focusShieldActive
    };

    try {
        await invoke('log_check_in', { logLine: JSON.stringify(logEntry) });
        window.dispatchEvent(new CustomEvent('ft:checkin-created'));
    } catch (error) {
        console.error('Failed to log check-in:', error);
    }

    // Hide window immediately after response
    await hideMainWindow('check-in submit');

    endWriteTime({ auto: options.auto, status });
}

async function endWriteTime({ auto = false, status } = {}) {
    console.log('endWriteTime called, isSessionRunning before:', isSessionRunning);
    isWriting = false;
    writeEndTimestamp = null;
    writeTimeRemaining = 0;
    hideCheckInScreen();

    // CRITICAL: Reset check-in timer for the NEXT check-in
    // This needs to happen BEFORE resuming the session
    checkInTimeRemaining = settings.checkInInterval * 60;
    const now = Date.now();
    checkInEndTimestamp = now + checkInTimeRemaining * 1000;

    // CRITICAL: Re-establish session timer
    // The session is still running in the background, we just need to resume ticking
    if (sessionTimeRemaining > 0) {
        sessionEndTimestamp = now + sessionTimeRemaining * 1000;
    }

    if (auto) statusOverride = 'Skipped';
    else if (status) statusOverride = `Logged: ${status}`;
    else statusOverride = 'Session active';

    // Ensure the main screen is visible
    if (dom.mainScreen) {
        dom.mainScreen.classList.remove('hidden');
    }

    updateDisplay();

    // Resume the session (restart ticking)
    isSessionRunning = true;
    startTicking();

    console.log('endWriteTime done, isSessionRunning after:', isSessionRunning, 'checkInEndTimestamp:', new Date(checkInEndTimestamp).toLocaleTimeString(), 'sessionEndTimestamp:', new Date(sessionEndTimestamp).toLocaleTimeString());
} function endSession() {
    // Reset session timers but keep running (continuous mode)
    checkInsCompleted = 0;
    skippedCheckIns = 0;
    lastCheckInWasSkipped = false;
    sessionTimeRemaining = settings.sessionDuration * 60;
    checkInTimeRemaining = settings.checkInInterval * 60;

    const now = Date.now();
    sessionEndTimestamp = now + sessionTimeRemaining * 1000;
    checkInEndTimestamp = now + checkInTimeRemaining * 1000;

    statusOverride = 'New cycle started';
    console.log('Session cycle complete, starting new cycle automatically');
    updateDisplay();
}

function extendFocusShield() {
    const now = Date.now();
    const extensionMs = FOCUS_SHIELD_EXTENSION_MINUTES * 60 * 1000;

    if (focusShieldActive && focusShieldUntil && focusShieldUntil > now) {
        focusShieldUntil += extensionMs;
        statusOverride = 'Shield extended';
    } else {
        focusShieldUntil = now + extensionMs;
        statusOverride = 'Shield enabled';
    }

    focusShieldActive = true;
    if (isSessionRunning && checkInEndTimestamp) {
        const earliest = focusShieldUntil + settings.checkInInterval * 60 * 1000;
        if (checkInEndTimestamp < earliest) {
            checkInEndTimestamp = earliest;
        }
    }

    updateFocusShieldUi();
    startTicking();
    updateDisplay();
}

function cancelFocusShield() {
    focusShieldActive = false;
    focusShieldUntil = null;
    statusOverride = 'Shield off';
    updateFocusShieldUi();
    updateDisplay();
    stopTickingIfIdle();
}

async function openSettings() {
    try { await invoke('open_settings'); } catch (error) { console.error('Failed to open settings:', error); }
}

async function testCheckIn() { await triggerCheckIn({ forced: true }); }

async function useCalendarEvent(silent = false) {
    try {
        const result = await invoke('get_current_event');
        if (result) {
            if (dom.sessionGoal) dom.sessionGoal.value = result;
            isUsingCalendarEvent = true;
            updateCalendarButtonState();
            startCalendarAutoRefresh();
        } else if (!silent) {
            alert('No calendar event found for the current time.');
        }
    } catch (error) {
        if (!silent) alert(error || 'Failed to access calendar.');
        console.error('Calendar access failed:', error);
    }
}

function updateCalendarButtonState() {
    const btn = dom.calendarBtn;
    if (!btn) return;
    if (isUsingCalendarEvent) {
        btn.textContent = '✅ Synced';
        btn.style.color = '#48bb78';
    } else {
        btn.textContent = '📅 Event';
        btn.style.color = '';
    }
}

function startCalendarAutoRefresh() {
    if (calendarRefreshInterval) clearInterval(calendarRefreshInterval);
    calendarRefreshInterval = setInterval(async () => {
        if (!isUsingCalendarEvent) return;
        try {
            const result = await invoke('get_current_event');
            if (result && dom.sessionGoal && dom.sessionGoal.value !== result) {
                dom.sessionGoal.value = result;
            } else if (!result) {
                if (dom.sessionGoal) dom.sessionGoal.placeholder = 'No current event';
                isUsingCalendarEvent = false;
                updateCalendarButtonState();
                stopCalendarAutoRefresh();
            }
        } catch (error) { }
    }, 60000);
}

function stopCalendarAutoRefresh() {
    if (calendarRefreshInterval) {
        clearInterval(calendarRefreshInterval);
        calendarRefreshInterval = null;
    }
}

// Shortcuts and Init
document.addEventListener('keydown', async (e) => {
    // ESC to close/hide window
    if (e.key === 'Escape') {
        try {
            await ensureTauriReady();
            const win = window.Tauri?.appWindow;
            console.log('🔽 ESC pressed - hiding window...');
            if (win && win.hide) {
                await win.hide();
                console.log('✅ Window hidden via ESC');
            }
        } catch (error) {
            console.error('❌ Failed to hide window on ESC:', error);
        }
        return;
    }

    // Cmd+Shift+I to open devtools
    if ((e.metaKey || e.ctrlKey) && e.shiftKey && e.key === 'I') {
        try {
            await ensureTauriReady();
            await appWindow.openDevtools();
        } catch (error) { }
    }
});

window.addEventListener('DOMContentLoaded', async () => {
    await ensureTauriReady();
    cacheDom();
    updateFocusShieldUi();
    updateDisplay();

    // Center window on startup
    try {
        await invoke('position_window_centered');
    } catch (error) {
        console.error('Failed to center window on startup:', error);
    }

    await appWindow.onCloseRequested(async (event) => {
        event.preventDefault();
        await appWindow.hide();
    });

    if (dom.startBtn) dom.startBtn.addEventListener('click', toggleSession);
    if (dom.resetBtn) dom.resetBtn.addEventListener('click', resetSession);
    if (dom.settingsBtn) dom.settingsBtn.addEventListener('click', openSettings);
    if (dom.testBtn) dom.testBtn.addEventListener('click', testCheckIn);

    if (dom.calendarBtn) {
        dom.calendarBtn.addEventListener('click', () => {
            if (isUsingCalendarEvent) {
                isUsingCalendarEvent = false;
                updateCalendarButtonState();
                stopCalendarAutoRefresh();
                if (dom.sessionGoal) dom.sessionGoal.value = '';
            } else {
                useCalendarEvent(false);
            }
        });
    }

    if (dom.focusShieldBtn) dom.focusShieldBtn.addEventListener('click', extendFocusShield);
    if (dom.focusShieldCancelBtn) dom.focusShieldCancelBtn.addEventListener('click', cancelFocusShield);

    // Window Controls
    const winClose = document.getElementById('winClose');
    const winMinimize = document.getElementById('winMinimize');

    if (winClose) {
        winClose.addEventListener('click', async () => {
            try {
                await ensureTauriReady();
                await appWindow.hide();
            } catch (e) { console.error('Failed to hide window:', e); }
        });
    }

    if (winMinimize) {
        winMinimize.addEventListener('click', async () => {
            try {
                await ensureTauriReady();
                await appWindow.minimize();
            } catch (e) { console.error('Failed to minimize window:', e); }
        });
    }

    document.querySelectorAll('.check-in-buttons button').forEach((button) => {
        button.addEventListener('click', () => {
            handleCheckInResponse(button.getAttribute('data-status'));
        });
    });

    if (dom.checkInNotes) {
        dom.checkInNotes.addEventListener('keypress', (event) => {
            if (event.key === 'Enter') handleCheckInResponse('On Task');
        });
    }

    if (dom.sessionGoal) {
        dom.sessionGoal.addEventListener('keypress', (event) => {
            if (event.key === 'Enter' && !isSessionRunning) toggleSession();
        });
        dom.sessionGoal.addEventListener('input', (event) => {
            if (isUsingCalendarEvent && event.inputType) {
                isUsingCalendarEvent = false;
                updateCalendarButtonState();
                stopCalendarAutoRefresh();
            }
        });
    }

    await loadSettings();
    resetSession();

    // Auto-sync calendar on startup
    useCalendarEvent(true);

    // Handle visibility changes - ensure timer keeps running when window becomes visible
    document.addEventListener('visibilitychange', () => {
        if (document.visibilityState === 'visible' && isSessionRunning) {
            console.log('Window became visible, ensuring timer is running');
            // Force an immediate tick to catch up
            tick().catch((error) => console.error('Visibility tick failed:', error));
        }
    });

    // Listen for settings updates from settings window
    async function setupSettingsListener() {
        try {
            await listen('settings-updated', async (event) => {
                console.log('Settings updated event received:', event.payload);
                currentSettings = event.payload;

                // Update timer display if not currently running
                const startBtn = document.getElementById('startBtn');
                const isIdle = startBtn.textContent === 'Start Focus';

                if (isIdle) {
                    const minutes = event.payload.check_in_interval;
                    document.getElementById('timer').textContent = `${String(minutes).padStart(2, '0')}:00`;
                    console.log('Timer display updated to:', minutes, 'minutes');
                } else {
                    console.log('Timer is running, settings will apply on next session');
                }
            });
        } catch (error) {
            console.error('Failed to setup settings listener:', error);
        }
    }

    setupSettingsListener();

    const unlisten = await listen('settings-updated', (event) => {
        if (event?.payload) {
            const payload = event.payload;
            settings = {
                sessionDuration: payload.session_duration || settings.sessionDuration,
                checkInInterval: payload.check_in_interval || settings.checkInInterval,
                writeTime: payload.write_time || settings.writeTime
            };
            const now = Date.now();
            captureRemainingTimes(now);
            if (isSessionRunning) {
                sessionEndTimestamp = now + sessionTimeRemaining * 1000;
                checkInEndTimestamp = now + checkInTimeRemaining * 1000;
            }
            updateDisplay();
        }
    });

    window.addEventListener('beforeunload', () => unlisten());
});
