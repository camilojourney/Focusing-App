/**
 * Session Review Module
 *
 * Manages the session review drawer UI that displays check-ins from the current session.
 * Implements Spec 001: Session Review Panel
 */

const { invoke } = window.__TAURI__.tauri;

class SessionReview {
    constructor() {
        this.isOpen = false;
        this.sessionStartTime = null;
        this.entries = [];

        // DOM element references (will be set after DOM loads)
        this.drawerEl = null;
        this.triggerBtn = null;
        this.closeBtn = null;
        this.entriesContainer = null;
        this.summaryEl = null;
        this.emptyStateEl = null;

        // Bind methods
        this.open = this.open.bind(this);
        this.close = this.close.bind(this);
        this.toggle = this.toggle.bind(this);
        this.refresh = this.refresh.bind(this);
        this.handleCheckInCreated = this.handleCheckInCreated.bind(this);
    }

    /**
     * Initialize the session review module
     * Call this after DOM is loaded
     */
    init() {
        // Get DOM references
        this.drawerEl = document.getElementById('reviewDrawer');
        this.triggerBtn = document.getElementById('reviewTrigger');
        this.closeBtn = document.getElementById('reviewClose');
        this.entriesContainer = document.getElementById('reviewEntries');
        this.summaryEl = document.getElementById('reviewSummary');
        this.emptyStateEl = document.getElementById('reviewEmptyState');

        if (!this.drawerEl || !this.triggerBtn) {
            console.error('Session review elements not found in DOM');
            return;
        }

        // Add event listeners
        this.triggerBtn.addEventListener('click', this.toggle);
        if (this.closeBtn) {
            this.closeBtn.addEventListener('click', this.close);
        }

        // Listen for check-in created events
        window.addEventListener('ft:checkin-created', this.handleCheckInCreated);

        console.log('Session Review module initialized');
    }

    /**
     * Set the session start time (when timer starts)
     * @param {Date} startTime - The session start time
     */
    setSessionStartTime(startTime) {
        this.sessionStartTime = startTime;
        console.log('Session start time set:', this.sessionStartTime.toISOString());
    }

    /**
     * Open the review drawer
     */
    async open() {
        if (this.isOpen) return;

        this.isOpen = true;
        this.drawerEl.classList.add('open');
        this.drawerEl.setAttribute('aria-expanded', 'true');
        this.triggerBtn.setAttribute('aria-pressed', 'true');

        // Refresh data when opening
        await this.refresh();
    }

    /**
     * Close the review drawer
     */
    close() {
        if (!this.isOpen) return;

        this.isOpen = false;
        this.drawerEl.classList.remove('open');
        this.drawerEl.setAttribute('aria-expanded', 'false');
        this.triggerBtn.setAttribute('aria-pressed', 'false');
    }

    /**
     * Toggle the drawer open/closed
     */
    async toggle() {
        if (this.isOpen) {
            this.close();
        } else {
            await this.open();
        }
    }

    /**
     * Refresh the session entries from backend
     */
    async refresh() {
        if (!this.sessionStartTime) {
            console.warn('Cannot refresh: session start time not set');
            this.showEmptyState('No active session');
            return;
        }

        try {
            // Call backend to get session entries
            const startTimeIso = this.sessionStartTime.toISOString();
            this.entries = await invoke('list_session_entries', { startTimeIso });

            console.log(`Loaded ${this.entries.length} session entries`);

            // Render entries
            this.render();
        } catch (error) {
            console.error('Failed to load session entries:', error);
            this.showError(`Unable to load session log: ${error}`);
        }
    }

    /**
     * Render the entries list
     */
    render() {
        if (!this.entriesContainer) return;

        // Check if we have entries
        if (!this.entries || this.entries.length === 0) {
            this.showEmptyState('No check-ins yet. Stay mindful ✨');
            return;
        }

        // Hide empty state
        if (this.emptyStateEl) {
            this.emptyStateEl.style.display = 'none';
        }

        // Build entries HTML using DocumentFragment for performance
        const fragment = document.createDocumentFragment();

        this.entries.forEach(entry => {
            const entryEl = this.createEntryElement(entry);
            fragment.appendChild(entryEl);
        });

        // Clear and append
        this.entriesContainer.innerHTML = '';
        this.entriesContainer.appendChild(fragment);

        // Update summary
        this.updateSummary();
    }

    /**
     * Create a single entry element
     * @param {Object} entry - Session entry object
     * @returns {HTMLElement}
     */
    createEntryElement(entry) {
        const div = document.createElement('div');
        div.className = 'review-entry';

        // Parse timestamp for display
        const timestamp = new Date(entry.timestamp);
        const timeStr = timestamp.toLocaleTimeString('en-US', {
            hour: 'numeric',
            minute: '2-digit',
            hour12: true
        });

        // Build HTML
        div.innerHTML = `
            <div class="entry-header">
                <span class="entry-status">${entry.statusLabel}</span>
                <span class="entry-time">${timeStr}</span>
            </div>
            ${entry.note ? `<div class="entry-note">${this.escapeHtml(entry.note)}</div>` : ''}
        `;

        return div;
    }

    /**
     * Update the summary pill at top
     */
    updateSummary() {
        if (!this.summaryEl) return;

        // Count statuses
        const onTaskCount = this.entries.filter(e => e.status === 'On Task').length;
        const distractionCount = this.entries.filter(e =>
            e.status === 'Social Media' ||
            e.status === 'Email/Chat' ||
            e.status === 'Other Distraction'
        ).length;
        const breakCount = this.entries.filter(e => e.status === 'Taking a Break').length;

        this.summaryEl.innerHTML = `
            <span class="summary-stat">✅ On Task: ${onTaskCount}</span>
            <span class="summary-stat">🚫 Distracted: ${distractionCount}</span>
            <span class="summary-stat">☕️ Breaks: ${breakCount}</span>
        `;
    }

    /**
     * Show empty state message
     * @param {string} message - Message to display
     */
    showEmptyState(message) {
        if (this.entriesContainer) {
            this.entriesContainer.innerHTML = '';
        }

        if (this.emptyStateEl) {
            this.emptyStateEl.textContent = message;
            this.emptyStateEl.style.display = 'block';
        }

        if (this.summaryEl) {
            this.summaryEl.innerHTML = '<span class="summary-stat">No data yet</span>';
        }
    }

    /**
     * Show error message
     * @param {string} message - Error message
     */
    showError(message) {
        this.showEmptyState(`⚠️ ${message}`);
    }

    /**
     * Handle check-in created event
     */
    handleCheckInCreated() {
        console.log('Check-in created event received');

        // Refresh if drawer is open
        if (this.isOpen) {
            this.refresh();
        }
    }

    /**
     * Escape HTML to prevent XSS
     * @param {string} text - Text to escape
     * @returns {string}
     */
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    /**
     * Reset the review (when session resets)
     */
    reset() {
        this.sessionStartTime = null;
        this.entries = [];
        this.close();
        this.showEmptyState('Session reset');
    }
}

// Export singleton instance
const sessionReview = new SessionReview();

// Auto-initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => sessionReview.init());
} else {
    sessionReview.init();
}

// Export for use in other scripts
window.sessionReview = sessionReview;
