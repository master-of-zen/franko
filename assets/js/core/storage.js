/**
 * Franko Reader - Storage Module
 * Handles localStorage operations for settings and progress
 */

const STORAGE_KEYS = {
    SETTINGS: 'franko-settings',
    READING_SETTINGS: 'franko-reading-settings',
    SETTINGS_PANEL_WIDTH: 'franko-settings-panel-width',
    PROGRESS_PREFIX: 'franko-progress-'
};

/**
 * Save a setting to localStorage
 * @param {string} key - Setting key
 * @param {*} value - Setting value
 */
export function saveSetting(key, value) {
    const settings = JSON.parse(localStorage.getItem(STORAGE_KEYS.SETTINGS) || '{}');
    settings[key] = value;
    localStorage.setItem(STORAGE_KEYS.SETTINGS, JSON.stringify(settings));
}

/**
 * Load settings from localStorage
 * @returns {Object} Settings object
 */
export function loadSettings() {
    try {
        return JSON.parse(localStorage.getItem(STORAGE_KEYS.SETTINGS) || '{}');
    } catch (e) {
        console.error('Failed to load settings:', e);
        return {};
    }
}

/**
 * Save reading settings to localStorage
 * @param {Object} settings - Reading settings object
 */
export function saveReadingSettings(settings) {
    localStorage.setItem(STORAGE_KEYS.READING_SETTINGS, JSON.stringify(settings));
}

/**
 * Load reading settings from localStorage
 * @param {Object} defaults - Default settings
 * @returns {Object} Merged settings
 */
export function loadReadingSettings(defaults = {}) {
    try {
        const saved = localStorage.getItem(STORAGE_KEYS.READING_SETTINGS);
        if (saved) {
            const parsed = JSON.parse(saved);
            // Migrate old string textWidth to numeric
            if (typeof parsed.textWidth === 'string') {
                const widthMap = { narrow: 600, medium: 800, wide: 1000, full: 1400 };
                parsed.textWidth = widthMap[parsed.textWidth] || 800;
            }
            return { ...defaults, ...parsed };
        }
    } catch (e) {
        console.error('Failed to load reading settings:', e);
    }
    return defaults;
}

/**
 * Save settings panel width
 * @param {string} width - Width value (e.g., "320px")
 */
export function savePanelWidth(width) {
    localStorage.setItem(STORAGE_KEYS.SETTINGS_PANEL_WIDTH, width);
}

/**
 * Load settings panel width
 * @returns {string|null} Saved width or null
 */
export function loadPanelWidth() {
    return localStorage.getItem(STORAGE_KEYS.SETTINGS_PANEL_WIDTH);
}

/**
 * Save book reading progress
 * @param {string} bookId - Book identifier
 * @param {Object} progress - Progress data
 */
export function saveProgress(bookId, progress) {
    if (!bookId) return;

    const data = {
        ...progress,
        timestamp: Date.now()
    };
    localStorage.setItem(STORAGE_KEYS.PROGRESS_PREFIX + bookId, JSON.stringify(data));
}

/**
 * Load book reading progress
 * @param {string} bookId - Book identifier
 * @returns {Object|null} Progress data or null
 */
export function loadProgress(bookId) {
    if (!bookId) return null;

    try {
        const saved = localStorage.getItem(STORAGE_KEYS.PROGRESS_PREFIX + bookId);
        return saved ? JSON.parse(saved) : null;
    } catch (e) {
        console.error('Failed to load progress:', e);
        return null;
    }
}

/**
 * Clear all stored settings
 */
export function clearAllSettings() {
    localStorage.removeItem(STORAGE_KEYS.SETTINGS);
    localStorage.removeItem(STORAGE_KEYS.READING_SETTINGS);
    localStorage.removeItem(STORAGE_KEYS.SETTINGS_PANEL_WIDTH);
}

/**
 * Export all settings as JSON
 * @returns {string} JSON string of settings
 */
export function exportSettings() {
    const settings = loadSettings();
    const readingSettings = loadReadingSettings();
    return JSON.stringify({ settings, readingSettings }, null, 2);
}

/**
 * Import settings from JSON
 * @param {string} json - JSON string of settings
 * @returns {boolean} Success status
 */
export function importSettings(json) {
    try {
        const data = JSON.parse(json);
        if (data.settings) {
            localStorage.setItem(STORAGE_KEYS.SETTINGS, JSON.stringify(data.settings));
        }
        if (data.readingSettings) {
            localStorage.setItem(STORAGE_KEYS.READING_SETTINGS, JSON.stringify(data.readingSettings));
        }
        return true;
    } catch (e) {
        console.error('Failed to import settings:', e);
        return false;
    }
}
