/**
 * Franko Reader - DOM Utilities
 * Common DOM manipulation and element access
 */

/**
 * Cached DOM element references
 */
export const elements = {
    get sidebar() { return document.getElementById('sidebar'); },
    get toggleSidebarBtn() { return document.getElementById('toggle-sidebar'); },
    get closeSidebarBtn() { return document.getElementById('close-sidebar'); },
    get content() { return document.getElementById('content'); },
    get progressFill() { return document.getElementById('progress-fill'); },
    get increaseFontBtn() { return document.getElementById('increase-font'); },
    get decreaseFontBtn() { return document.getElementById('decrease-font'); },
    get toggleThemeBtn() { return document.getElementById('toggle-theme'); },
    get toggleFullscreenBtn() { return document.getElementById('toggle-fullscreen'); },
    get searchInput() { return document.getElementById('search'); },
    get readerContainer() { return document.getElementById('reader-container'); },
    get pageControls() { return document.getElementById('page-controls'); },
    get pageIndicator() { return document.getElementById('page-indicator'); },
    get pagePrevBtn() { return document.getElementById('page-prev'); },
    get pageNextBtn() { return document.getElementById('page-next'); },
    get settingsPanel() { return document.getElementById('settings-panel'); },
    get toggleSettingsBtn() { return document.getElementById('toggle-settings'); },
    get closeSettingsBtn() { return document.getElementById('close-settings'); },
    get resizeHandle() { return document.getElementById('settings-resize-handle'); }
};

/**
 * Query selector shorthand
 * @param {string} selector - CSS selector
 * @param {Element} [context=document] - Context element
 * @returns {Element|null} First matching element
 */
export function $(selector, context = document) {
    return context.querySelector(selector);
}

/**
 * Query selector all shorthand
 * @param {string} selector - CSS selector
 * @param {Element} [context=document] - Context element
 * @returns {NodeList} All matching elements
 */
export function $$(selector, context = document) {
    return context.querySelectorAll(selector);
}

/**
 * Create element with attributes and children
 * @param {string} tag - Tag name
 * @param {Object} [attrs={}] - Attributes
 * @param {string|Element|Element[]} [children] - Child content
 * @returns {Element} Created element
 */
export function createElement(tag, attrs = {}, children) {
    const el = document.createElement(tag);

    Object.entries(attrs).forEach(([key, value]) => {
        if (key === 'className') {
            el.className = value;
        } else if (key === 'style' && typeof value === 'object') {
            Object.assign(el.style, value);
        } else if (key.startsWith('on') && typeof value === 'function') {
            el.addEventListener(key.slice(2).toLowerCase(), value);
        } else {
            el.setAttribute(key, value);
        }
    });

    if (children) {
        if (typeof children === 'string') {
            el.textContent = children;
        } else if (Array.isArray(children)) {
            children.forEach(child => el.appendChild(child));
        } else {
            el.appendChild(children);
        }
    }

    return el;
}

/**
 * Add event listener to element
 * @param {Element|string} target - Element or selector
 * @param {string} event - Event name
 * @param {Function} handler - Event handler
 * @param {Object} [options] - Event listener options
 */
export function on(target, event, handler, options) {
    const el = typeof target === 'string' ? $(target) : target;
    if (el) {
        el.addEventListener(event, handler, options);
    }
}

/**
 * Add event listener to multiple elements
 * @param {NodeList|Element[]|string} targets - Elements or selector
 * @param {string} event - Event name
 * @param {Function} handler - Event handler
 */
export function onAll(targets, event, handler) {
    const elements = typeof targets === 'string' ? $$(targets) : targets;
    elements.forEach(el => el.addEventListener(event, handler));
}
