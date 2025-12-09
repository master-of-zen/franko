/**
 * Franko Reader - Main Entry Point
 *
 * This is the main JavaScript entry point that orchestrates all modules.
 * It replaces the monolithic reader.js with a modular ES6 architecture.
 */

// Core modules
import { throttle, debounce } from './core/utils.js';
import { elements } from './core/dom.js';
import { loadSettings, saveSetting } from './core/storage.js';
import { showToast } from './core/toast.js';

// Feature modules
import { applyTheme, toggleTheme, setAccentColor } from './features/theme.js';
import { applyTypography, applyFontSize, changeFontSize, setFontFamily } from './features/typography.js';
import { initLayout, setLayout, prevPage, nextPage, goToPage, recalculatePages, getLayoutState } from './features/layout.js';
import { initChapterTracking, updateScrollProgress, saveBookProgress, loadBookProgress, startProgressAutoSave } from './features/progress.js';
import { initSidebar, toggleSidebar, initTocNavigation } from './features/sidebar.js';
import { initKeyboard, toggleFullscreen, setToggleSettingsCallback, setStopAutoScrollCallback, setCurrentFontSize } from './features/keyboard.js';
import { initSearch } from './features/search.js';
import { startAutoScroll, stopAutoScroll, getAutoScrollSpeed } from './features/autoscroll.js';
import { initAnimations } from './features/animations.js';
import { initSettingsPanel, toggleSettingsPanel, setPositionCallbacks, getReadingSettings } from './features/settings-panel.js';
import { getReadingPosition, restoreReadingPosition } from './features/position.js';

/**
 * Initialize the reader application
 */
function init() {
    // Load saved settings first
    const settings = loadSettings();

    // Apply initial settings
    if (settings.fontSize) {
        applyFontSize(settings.fontSize);
        setCurrentFontSize(settings.fontSize);
    }

    if (settings.theme) {
        applyTheme(settings.theme);
    }

    if (settings.accentColor) {
        setAccentColor(settings.accentColor);
    }

    // Initialize features
    initSidebar();
    initSearch();
    initAnimations();
    initLayoutControls();
    initChapterTracking();
    initTocNavigation();

    // Initialize settings panel with position preservation
    setPositionCallbacks(getReadingPosition, restoreReadingPosition);
    initSettingsPanel();

    // Set up keyboard callbacks
    setToggleSettingsCallback(toggleSettingsPanel);
    setStopAutoScrollCallback(stopAutoScroll);
    initKeyboard();

    // Initialize layout
    const savedLayout = settings.layoutMode || 'scroll';
    initLayout(savedLayout);

    // Set up scroll progress tracking
    window.addEventListener('scroll', throttle(updateScrollProgress, 50));

    // Set up resize handler for paged layouts
    window.addEventListener('resize', debounce(() => {
        const { layout } = getLayoutState();
        if (layout !== 'scroll') {
            recalculatePages();
        }
    }, 250));

    // Set up progress auto-save
    startProgressAutoSave(30000);

    // Save progress on page leave
    window.addEventListener('beforeunload', saveBookProgress);

    // Initialize page navigation buttons
    const { pagePrevBtn, pageNextBtn } = elements;
    if (pagePrevBtn) {
        pagePrevBtn.addEventListener('click', prevPage);
    }
    if (pageNextBtn) {
        pageNextBtn.addEventListener('click', nextPage);
    }

    // Font size quick controls (if present in header)
    const { increaseFontBtn, decreaseFontBtn, toggleThemeBtn, toggleFullscreenBtn } = elements;

    let currentFontSize = settings.fontSize || 16;

    if (increaseFontBtn) {
        increaseFontBtn.addEventListener('click', () => {
            currentFontSize = changeFontSize(2, currentFontSize);
            setCurrentFontSize(currentFontSize);
        });
    }

    if (decreaseFontBtn) {
        decreaseFontBtn.addEventListener('click', () => {
            currentFontSize = changeFontSize(-2, currentFontSize);
            setCurrentFontSize(currentFontSize);
        });
    }

    if (toggleThemeBtn) {
        toggleThemeBtn.addEventListener('click', toggleTheme);
    }

    if (toggleFullscreenBtn) {
        toggleFullscreenBtn.addEventListener('click', toggleFullscreen);
    }
}

/**
 * Initialize layout control buttons
 */
function initLayoutControls() {
    const layoutBtns = document.querySelectorAll('.layout-btn');

    layoutBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const layout = btn.dataset.layout;
            setLayout(layout);

            // Update active state
            layoutBtns.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
        });
    });
}

/**
 * Initialize the settings page (if on settings page)
 */
function initSettingsPage() {
    if (!document.querySelector('.settings-page')) return;

    const settings = loadSettings();

    // Theme buttons
    document.querySelectorAll('.theme-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const theme = btn.dataset.theme;
            applyTheme(theme);
            saveSetting('theme', theme);

            document.querySelectorAll('.theme-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            showToast(`Theme: ${theme.charAt(0).toUpperCase() + theme.slice(1)}`);
        });

        // Set initial active state
        if (btn.dataset.theme === settings.theme) {
            btn.classList.add('active');
        }
    });

    // Color picker
    document.querySelectorAll('.color-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const color = btn.dataset.color;
            setAccentColor(color);

            document.querySelectorAll('.color-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
        });

        if (btn.dataset.color === settings.accentColor) {
            btn.classList.add('active');
        }
    });

    // Font family select
    const fontFamilySelect = document.getElementById('font-family');
    if (fontFamilySelect) {
        fontFamilySelect.value = settings.fontFamily || 'serif';
        fontFamilySelect.addEventListener('change', () => {
            setFontFamily(fontFamilySelect.value);
        });
    }

    // Font size range
    const fontSizeRange = document.getElementById('font-size-range');
    if (fontSizeRange) {
        fontSizeRange.value = settings.fontSize || 16;
        fontSizeRange.addEventListener('input', () => {
            const size = parseInt(fontSizeRange.value);
            applyFontSize(size);
            saveSetting('fontSize', size);

            const display = document.querySelector('.font-size-display');
            if (display) display.textContent = size + 'px';
        });
    }

    // Toggle switches
    document.querySelectorAll('.toggle input').forEach(toggle => {
        if (settings[toggle.id] !== undefined) {
            toggle.checked = settings[toggle.id];
        }

        toggle.addEventListener('change', () => {
            saveSetting(toggle.id, toggle.checked);
            showToast(`${formatSettingName(toggle.id)}: ${toggle.checked ? 'On' : 'Off'}`);
        });
    });

    // Save button
    const saveBtn = document.getElementById('save-settings');
    if (saveBtn) {
        saveBtn.addEventListener('click', () => {
            showToast('Settings saved!');
        });
    }

    // Reset button
    const resetBtn = document.getElementById('reset-settings');
    if (resetBtn) {
        resetBtn.addEventListener('click', () => {
            if (confirm('Are you sure you want to reset all settings to defaults?')) {
                localStorage.removeItem('franko-settings');
                localStorage.removeItem('franko-reading-settings');
                showToast('Settings reset to defaults');
                location.reload();
            }
        });
    }

    // Export button
    const exportBtn = document.getElementById('export-settings');
    if (exportBtn) {
        exportBtn.addEventListener('click', () => {
            const settingsData = localStorage.getItem('franko-settings') || '{}';
            const blob = new Blob([settingsData], { type: 'application/json' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'franko-settings.json';
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
            showToast('Settings exported');
        });
    }

    // Import button
    const importBtn = document.getElementById('import-settings');
    if (importBtn) {
        importBtn.addEventListener('click', () => {
            const input = document.createElement('input');
            input.type = 'file';
            input.accept = '.json';
            input.onchange = (e) => {
                const file = e.target.files[0];
                if (file) {
                    const reader = new FileReader();
                    reader.onload = (evt) => {
                        try {
                            const data = JSON.parse(evt.target.result);
                            localStorage.setItem('franko-settings', JSON.stringify(data));
                            showToast('Settings imported');
                            location.reload();
                        } catch (err) {
                            showToast('Invalid settings file');
                        }
                    };
                    reader.readAsText(file);
                }
            };
            input.click();
        });
    }
}

/**
 * Format setting name for display
 */
function formatSettingName(name) {
    return name
        .replace(/-/g, ' ')
        .replace(/([A-Z])/g, ' $1')
        .replace(/^./, str => str.toUpperCase())
        .trim();
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        init();
        initSettingsPage();

        // Load reading progress after a short delay
        setTimeout(loadBookProgress, 100);
    });
} else {
    init();
    initSettingsPage();
    setTimeout(loadBookProgress, 100);
}

// Export global functions for inline handlers
window.frankoStopAutoScroll = stopAutoScroll;
