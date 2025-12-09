/**
 * Franko Reader - Keyboard Feature
 * Handles keyboard shortcuts throughout the reader
 */

import { toggleSidebar } from './sidebar.js';
import { toggleTheme } from './theme.js';
import { changeFontSize } from './typography.js';
import { setLayout, prevPage, nextPage, goToPage, getLayoutState } from './layout.js';
import { showToast } from '../core/toast.js';

// External callbacks that can be set
let toggleSettingsPanelCallback = null;
let stopAutoScrollCallback = null;
let currentFontSize = 16;

/**
 * Set callback for toggling settings panel
 * @param {Function} callback - Toggle function
 */
export function setToggleSettingsCallback(callback) {
    toggleSettingsPanelCallback = callback;
}

/**
 * Set callback for stopping auto scroll
 * @param {Function} callback - Stop function
 */
export function setStopAutoScrollCallback(callback) {
    stopAutoScrollCallback = callback;
}

/**
 * Set current font size for adjustments
 * @param {number} size - Current font size
 */
export function setCurrentFontSize(size) {
    currentFontSize = size;
}

/**
 * Handle keyboard shortcuts
 * @param {KeyboardEvent} e - Keyboard event
 */
export function handleKeyboard(e) {
    // Don't handle if in input
    if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') {
        return;
    }

    const layoutState = getLayoutState();

    switch (e.key) {
        case 'ArrowLeft':
        case 'h':
            if (layoutState.layout !== 'scroll') {
                e.preventDefault();
                prevPage();
            } else {
                navigatePrev();
            }
            break;

        case 'ArrowRight':
        case 'l':
            if (layoutState.layout !== 'scroll') {
                e.preventDefault();
                nextPage();
            } else {
                navigateNext();
            }
            break;

        case 'j':
            if (layoutState.layout === 'scroll') {
                window.scrollBy({ top: 100, behavior: 'smooth' });
            }
            break;

        case 'k':
            if (layoutState.layout === 'scroll') {
                window.scrollBy({ top: -100, behavior: 'smooth' });
            }
            break;

        case ' ':
            e.preventDefault();
            if (layoutState.layout !== 'scroll') {
                if (e.shiftKey) {
                    prevPage();
                } else {
                    nextPage();
                }
            } else {
                if (!e.shiftKey) {
                    window.scrollBy({ top: window.innerHeight - 100, behavior: 'smooth' });
                } else {
                    window.scrollBy({ top: -(window.innerHeight - 100), behavior: 'smooth' });
                }
            }
            break;

        case 'g':
            if (layoutState.layout !== 'scroll') {
                if (e.shiftKey) {
                    goToPage(layoutState.totalPages - 1);
                } else {
                    goToPage(0);
                }
            } else if (e.shiftKey) {
                window.scrollTo({ top: document.documentElement.scrollHeight, behavior: 'smooth' });
            } else {
                window.scrollTo({ top: 0, behavior: 'smooth' });
            }
            break;

        case 't':
            toggleSidebar();
            break;

        case '=':
        case '+':
            currentFontSize = changeFontSize(2, currentFontSize);
            break;

        case '-':
            currentFontSize = changeFontSize(-2, currentFontSize);
            break;

        case 'd':
            toggleTheme();
            break;

        case 'f':
            toggleFullscreen();
            break;

        case '1':
            setLayout('scroll');
            break;

        case '2':
            setLayout('paged');
            break;

        case '3':
            setLayout('dual');
            break;

        case '/':
            e.preventDefault();
            openSearch();
            break;

        case 's':
            if (toggleSettingsPanelCallback) {
                toggleSettingsPanelCallback();
            }
            break;

        case 'Escape':
            handleEscape();
            break;
    }
}

/**
 * Handle escape key
 */
function handleEscape() {
    const sidebar = document.getElementById('sidebar');
    if (sidebar && sidebar.classList.contains('open')) {
        toggleSidebar();
    }

    const settingsPanel = document.getElementById('settings-panel');
    if (settingsPanel && settingsPanel.classList.contains('open') && toggleSettingsPanelCallback) {
        toggleSettingsPanelCallback();
    }

    if (stopAutoScrollCallback) {
        stopAutoScrollCallback();
    }
}

/**
 * Navigate to previous chapter/page
 */
function navigatePrev() {
    const prevLink = document.querySelector('.nav-prev');
    if (prevLink) {
        prevLink.click();
    }
}

/**
 * Navigate to next chapter/page
 */
function navigateNext() {
    const nextLink = document.querySelector('.nav-next');
    if (nextLink) {
        nextLink.click();
    }
}

/**
 * Toggle fullscreen mode
 */
export function toggleFullscreen() {
    const readerLayout = document.querySelector('.reader-layout');
    if (!readerLayout) return;

    if (document.fullscreenElement) {
        document.exitFullscreen();
        readerLayout.classList.remove('fullscreen');
    } else {
        readerLayout.requestFullscreen().catch(() => { });
        readerLayout.classList.add('fullscreen');
    }
}

/**
 * Open search input
 */
function openSearch() {
    const searchInput = document.getElementById('search');
    if (searchInput) {
        searchInput.focus();
        searchInput.select();
    } else {
        showToast('Press / to search');
    }
}

/**
 * Initialize keyboard event listener
 */
export function initKeyboard() {
    document.addEventListener('keydown', handleKeyboard);
}
