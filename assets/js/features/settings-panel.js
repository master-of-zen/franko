/**
 * Franko Reader - Settings Panel Feature
 * Handles the reader settings panel UI and state
 */

import { elements, createElement } from '../core/dom.js';
import { saveReadingSettings, loadReadingSettings, loadPanelWidth, savePanelWidth } from '../core/storage.js';
import { applyTheme, applyCustomColors } from './theme.js';
import { applyTypography } from './typography.js';

// Default reading settings
const DEFAULT_SETTINGS = {
    fontSize: 18,
    lineHeight: 1.8,
    textWidth: 800,
    fontFamily: 'serif',
    theme: 'dark',
    paraSpacing: 1,
    panelMinWidth: 250,
    panelMaxWidth: 600,
    customColors: {
        background: '#1a1a2e',
        text: '#eaeaea',
        accent: '#6366f1',
        link: '#818cf8'
    }
};

// Current settings state
let readingSettings = { ...DEFAULT_SETTINGS };

// Callbacks for position restoration
let positionCallbacks = {
    getPosition: null,
    restorePosition: null
};

/**
 * Get current reading settings
 * @returns {Object} Current settings
 */
export function getReadingSettings() {
    return { ...readingSettings };
}

/**
 * Set position restoration callbacks
 * @param {Function} getPosition - Function to get current position
 * @param {Function} restorePosition - Function to restore position
 */
export function setPositionCallbacks(getPosition, restorePosition) {
    positionCallbacks.getPosition = getPosition;
    positionCallbacks.restorePosition = restorePosition;
}

/**
 * Toggle settings panel visibility
 */
export function toggleSettingsPanel() {
    const { settingsPanel } = elements;
    if (!settingsPanel) return;

    settingsPanel.classList.toggle('open');

    // Handle overlay
    let overlay = document.querySelector('.settings-overlay');
    if (settingsPanel.classList.contains('open')) {
        if (!overlay) {
            overlay = createElement('div', {
                className: 'settings-overlay',
                onClick: toggleSettingsPanel
            });
            document.body.appendChild(overlay);
        }
        setTimeout(() => overlay.classList.add('active'), 10);
    } else if (overlay) {
        overlay.classList.remove('active');
        setTimeout(() => overlay.remove(), 300);
    }
}

/**
 * Initialize settings panel
 */
export function initSettingsPanel() {
    const { settingsPanel, toggleSettingsBtn, closeSettingsBtn, resizeHandle } = elements;

    if (!settingsPanel) return;

    // Toggle and close buttons
    if (toggleSettingsBtn) {
        toggleSettingsBtn.addEventListener('click', toggleSettingsPanel);
    }
    if (closeSettingsBtn) {
        closeSettingsBtn.addEventListener('click', toggleSettingsPanel);
    }

    // Panel resize functionality
    if (resizeHandle) {
        initPanelResize(settingsPanel, resizeHandle);
    }

    // Initialize all controls
    initFontControls();
    initLineHeightControls();
    initParagraphSpacingControls();
    initTextWidthControls();
    initFontFamilyControl();
    initThemeControls();
    initCustomColorControls();
    initPanelWidthControls();

    // Load saved settings
    loadAndApplySettings();
}

/**
 * Initialize panel resize functionality
 */
function initPanelResize(panel, handle) {
    let isResizing = false;
    let startX = 0;
    let startWidth = 0;

    handle.addEventListener('mousedown', (e) => {
        isResizing = true;
        startX = e.clientX;
        startWidth = panel.offsetWidth;
        panel.classList.add('resizing');
        handle.classList.add('active');
        e.preventDefault();
    });

    document.addEventListener('mousemove', (e) => {
        if (!isResizing) return;
        const deltaX = startX - e.clientX;
        const minW = readingSettings.panelMinWidth || 250;
        const maxW = readingSettings.panelMaxWidth || 600;
        const newWidth = Math.min(maxW, Math.max(minW, startWidth + deltaX));
        panel.style.width = newWidth + 'px';
    });

    document.addEventListener('mouseup', () => {
        if (isResizing) {
            isResizing = false;
            panel.classList.remove('resizing');
            handle.classList.remove('active');
            savePanelWidth(panel.style.width);
        }
    });

    // Restore saved width
    const savedWidth = loadPanelWidth();
    if (savedWidth) {
        panel.style.width = savedWidth;
    }
}

/**
 * Initialize font size controls
 */
function initFontControls() {
    const input = document.getElementById('font-size-input');

    document.getElementById('font-decrease')?.addEventListener('click', () => {
        adjustSetting('fontSize', -1);
    });
    document.getElementById('font-increase')?.addEventListener('click', () => {
        adjustSetting('fontSize', 1);
    });
    input?.addEventListener('change', (e) => {
        setSettingValue('fontSize', parseInt(e.target.value));
    });
}

/**
 * Initialize line height controls
 */
function initLineHeightControls() {
    const input = document.getElementById('line-height-input');

    document.getElementById('line-height-decrease')?.addEventListener('click', () => {
        adjustSetting('lineHeight', -0.1);
    });
    document.getElementById('line-height-increase')?.addEventListener('click', () => {
        adjustSetting('lineHeight', 0.1);
    });
    input?.addEventListener('change', (e) => {
        setSettingValue('lineHeight', parseFloat(e.target.value));
    });
}

/**
 * Initialize paragraph spacing controls
 */
function initParagraphSpacingControls() {
    const input = document.getElementById('para-spacing-input');

    document.getElementById('para-spacing-decrease')?.addEventListener('click', () => {
        adjustSetting('paraSpacing', -0.25);
    });
    document.getElementById('para-spacing-increase')?.addEventListener('click', () => {
        adjustSetting('paraSpacing', 0.25);
    });
    input?.addEventListener('change', (e) => {
        setSettingValue('paraSpacing', parseFloat(e.target.value));
    });
}

/**
 * Initialize text width controls
 */
function initTextWidthControls() {
    const input = document.getElementById('text-width-input');

    document.getElementById('text-width-decrease')?.addEventListener('click', () => {
        const current = readingSettings.textWidth || 800;
        setCustomTextWidth(current - 50);
    });
    document.getElementById('text-width-increase')?.addEventListener('click', () => {
        const current = readingSettings.textWidth || 800;
        setCustomTextWidth(current + 50);
    });
    input?.addEventListener('change', (e) => {
        setCustomTextWidth(parseInt(e.target.value));
    });

    // Preset buttons
    document.querySelectorAll('[data-width]').forEach(btn => {
        btn.addEventListener('click', () => {
            const widthValue = parseInt(btn.dataset.widthValue);
            setCustomTextWidth(widthValue);
            document.querySelectorAll('[data-width]').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
        });
    });
}

/**
 * Initialize font family control
 */
function initFontFamilyControl() {
    document.getElementById('font-family-select')?.addEventListener('change', (e) => {
        const position = getPosition();
        readingSettings.fontFamily = e.target.value;
        applyAllSettings();
        saveSettings();
        restorePosition(position);
    });
}

/**
 * Initialize theme controls
 */
function initThemeControls() {
    // Theme select dropdown
    document.getElementById('theme-select')?.addEventListener('change', (e) => {
        readingSettings.theme = e.target.value;
        applyAllSettings();
        updateCustomColorVisibility();
        saveSettings();
    });

    // Legacy theme buttons
    document.querySelectorAll('[data-theme]').forEach(btn => {
        btn.addEventListener('click', () => {
            readingSettings.theme = btn.dataset.theme;
            document.querySelectorAll('[data-theme]').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');

            const themeSelect = document.getElementById('theme-select');
            if (themeSelect) themeSelect.value = btn.dataset.theme;

            applyAllSettings();
            updateCustomColorVisibility();
            saveSettings();
        });
    });
}

/**
 * Initialize custom color controls
 */
function initCustomColorControls() {
    const colorInputs = ['background', 'text', 'accent', 'link'];

    colorInputs.forEach(colorType => {
        const input = document.getElementById(`custom-color-${colorType}`);
        const textInput = document.getElementById(`custom-color-${colorType}-text`);

        if (input) {
            input.addEventListener('input', (e) => {
                setCustomColor(colorType, e.target.value);
                if (textInput) textInput.value = e.target.value;
            });
        }
        if (textInput) {
            textInput.addEventListener('change', (e) => {
                const value = e.target.value;
                if (/^#[0-9A-Fa-f]{6}$/.test(value)) {
                    setCustomColor(colorType, value);
                    if (input) input.value = value;
                }
            });
        }
    });
}

/**
 * Initialize panel width limit controls
 */
function initPanelWidthControls() {
    const minInput = document.getElementById('panel-min-width-input');
    const maxInput = document.getElementById('panel-max-width-input');

    // Min width controls
    document.getElementById('panel-min-width-decrease')?.addEventListener('click', () => {
        let value = (readingSettings.panelMinWidth || 250) - 50;
        value = Math.max(200, Math.min(value, readingSettings.panelMaxWidth - 50));
        readingSettings.panelMinWidth = value;
        applyPanelWidthLimits();
        updateSettingsDisplay();
        saveSettings();
    });

    document.getElementById('panel-min-width-increase')?.addEventListener('click', () => {
        let value = (readingSettings.panelMinWidth || 250) + 50;
        value = Math.max(200, Math.min(value, readingSettings.panelMaxWidth - 50));
        readingSettings.panelMinWidth = value;
        applyPanelWidthLimits();
        updateSettingsDisplay();
        saveSettings();
    });

    minInput?.addEventListener('change', (e) => {
        let value = parseInt(e.target.value) || 200;
        value = Math.max(200, Math.min(value, readingSettings.panelMaxWidth - 50));
        readingSettings.panelMinWidth = value;
        applyPanelWidthLimits();
        updateSettingsDisplay();
        saveSettings();
    });

    // Max width controls
    document.getElementById('panel-max-width-decrease')?.addEventListener('click', () => {
        let value = (readingSettings.panelMaxWidth || 600) - 50;
        value = Math.max(readingSettings.panelMinWidth + 50, Math.min(value, 1200));
        readingSettings.panelMaxWidth = value;
        applyPanelWidthLimits();
        updateSettingsDisplay();
        saveSettings();
    });

    document.getElementById('panel-max-width-increase')?.addEventListener('click', () => {
        let value = (readingSettings.panelMaxWidth || 600) + 50;
        value = Math.max(readingSettings.panelMinWidth + 50, Math.min(value, 1200));
        readingSettings.panelMaxWidth = value;
        applyPanelWidthLimits();
        updateSettingsDisplay();
        saveSettings();
    });

    maxInput?.addEventListener('change', (e) => {
        let value = parseInt(e.target.value) || 800;
        value = Math.max(readingSettings.panelMinWidth + 50, Math.min(value, 1200));
        readingSettings.panelMaxWidth = value;
        applyPanelWidthLimits();
        updateSettingsDisplay();
        saveSettings();
    });
}

/**
 * Adjust a setting by delta
 */
function adjustSetting(setting, delta) {
    const position = getPosition();

    let newValue = readingSettings[setting] + delta;
    newValue = Math.round(newValue * 100) / 100;

    // Ensure minimum values
    if (setting === 'fontSize' && newValue < 1) newValue = 1;
    if (setting === 'lineHeight' && newValue < 0.1) newValue = 0.1;
    if (setting === 'paraSpacing' && newValue < 0) newValue = 0;

    readingSettings[setting] = newValue;
    applyAllSettings();
    updateSettingsDisplay();
    saveSettings();
    restorePosition(position);
}

/**
 * Set a specific setting value
 */
function setSettingValue(setting, value) {
    const position = getPosition();

    if (setting === 'fontSize' && value < 1) value = 1;
    if (setting === 'lineHeight' && value < 0.1) value = 0.1;
    if (setting === 'paraSpacing' && value < 0) value = 0;

    readingSettings[setting] = value;
    applyAllSettings();
    updateSettingsDisplay();
    saveSettings();
    restorePosition(position);
}

/**
 * Set custom text width
 */
function setCustomTextWidth(widthPx) {
    const position = getPosition();

    widthPx = Math.max(400, Math.min(1400, widthPx));
    readingSettings.textWidth = widthPx;
    applyAllSettings();
    updateSettingsDisplay();
    saveSettings();
    restorePosition(position);
}

/**
 * Set custom color
 */
function setCustomColor(colorType, value) {
    if (!readingSettings.customColors) {
        readingSettings.customColors = { ...DEFAULT_SETTINGS.customColors };
    }
    readingSettings.customColors[colorType] = value;

    if (readingSettings.theme === 'custom') {
        applyAllSettings();
    }
    saveSettings();
}

/**
 * Apply all current settings
 */
function applyAllSettings() {
    applyTypography(readingSettings);
    applyTheme(readingSettings.theme, readingSettings.customColors);
}

/**
 * Apply panel width limits
 */
function applyPanelWidthLimits() {
    const { settingsPanel } = elements;
    if (!settingsPanel) return;

    const minW = readingSettings.panelMinWidth || 250;
    const maxW = readingSettings.panelMaxWidth || 600;

    settingsPanel.style.minWidth = minW + 'px';
    settingsPanel.style.maxWidth = maxW + 'px';

    const currentWidth = settingsPanel.offsetWidth;
    if (currentWidth < minW) {
        settingsPanel.style.width = minW + 'px';
    } else if (currentWidth > maxW) {
        settingsPanel.style.width = maxW + 'px';
    }
}

/**
 * Update settings display
 */
function updateSettingsDisplay() {
    // Font size
    const fontSizeInput = document.getElementById('font-size-input');
    if (fontSizeInput) fontSizeInput.value = readingSettings.fontSize;

    // Line height
    const lineHeightInput = document.getElementById('line-height-input');
    if (lineHeightInput) lineHeightInput.value = readingSettings.lineHeight.toFixed(1);

    // Paragraph spacing
    const paraSpacingInput = document.getElementById('para-spacing-input');
    if (paraSpacingInput) paraSpacingInput.value = readingSettings.paraSpacing;

    // Text width
    const textWidthInput = document.getElementById('text-width-input');
    const textWidthValue = typeof readingSettings.textWidth === 'number'
        ? readingSettings.textWidth
        : parseInt(readingSettings.textWidth) || 800;
    if (textWidthInput) textWidthInput.value = textWidthValue;

    // Update preset buttons
    const presetWidths = { 600: 'narrow', 800: 'medium', 1000: 'wide', 1400: 'full' };
    const matchingPreset = presetWidths[textWidthValue];
    document.querySelectorAll('[data-width]').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.width === matchingPreset);
    });

    // Font family
    const fontSelect = document.getElementById('font-family-select');
    if (fontSelect) fontSelect.value = readingSettings.fontFamily;

    // Theme
    const themeSelect = document.getElementById('theme-select');
    if (themeSelect) themeSelect.value = readingSettings.theme;

    document.querySelectorAll('[data-theme]').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.theme === readingSettings.theme);
    });

    // Custom colors
    updateCustomColorInputs();
    updateCustomColorVisibility();

    // Panel width limits
    const panelMinWidthInput = document.getElementById('panel-min-width-input');
    const panelMaxWidthInput = document.getElementById('panel-max-width-input');
    if (panelMinWidthInput) panelMinWidthInput.value = readingSettings.panelMinWidth || 250;
    if (panelMaxWidthInput) panelMaxWidthInput.value = readingSettings.panelMaxWidth || 600;
}

/**
 * Update custom color inputs
 */
function updateCustomColorInputs() {
    if (!readingSettings.customColors) return;

    const colorInputs = ['background', 'text', 'accent', 'link'];
    colorInputs.forEach(colorType => {
        const input = document.getElementById(`custom-color-${colorType}`);
        const textInput = document.getElementById(`custom-color-${colorType}-text`);
        const value = readingSettings.customColors[colorType];

        if (input && value) input.value = value;
        if (textInput && value) textInput.value = value;
    });
}

/**
 * Update custom color controls visibility
 */
function updateCustomColorVisibility() {
    const customColorGroup = document.getElementById('custom-colors-group');
    if (customColorGroup) {
        customColorGroup.style.display = readingSettings.theme === 'custom' ? 'block' : 'none';
    }
}

/**
 * Save settings to storage
 */
function saveSettings() {
    saveReadingSettings(readingSettings);
}

/**
 * Load and apply saved settings
 */
function loadAndApplySettings() {
    readingSettings = loadReadingSettings(DEFAULT_SETTINGS);
    applyAllSettings();
    applyPanelWidthLimits();
    updateSettingsDisplay();
}

// Helper functions for position preservation
function getPosition() {
    return positionCallbacks.getPosition ? positionCallbacks.getPosition() : null;
}

function restorePosition(position) {
    if (positionCallbacks.restorePosition && position) {
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                positionCallbacks.restorePosition(position);
            });
        });
    }
}
