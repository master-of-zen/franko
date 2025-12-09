/**
 * Franko Reader - Typography Feature
 * Handles font size, family, line height, and text width
 */

import { saveSetting } from '../core/storage.js';
import { showToast } from '../core/toast.js';
import { clamp } from '../core/utils.js';

// Font families mapping
export const FONT_FAMILIES = {
    serif: 'Georgia, Cambria, "Times New Roman", Times, serif',
    sans: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    mono: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
    system: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    inter: '"Inter", -apple-system, sans-serif',
    merriweather: '"Merriweather", Georgia, serif',
    literata: '"Literata", Georgia, serif',
    jetbrains: '"JetBrains Mono", ui-monospace, monospace',
    fira: '"Fira Code", ui-monospace, monospace',
    opendyslexic: '"OpenDyslexic", sans-serif'
};

// Font family display names
export const FONT_FAMILY_NAMES = {
    system: 'System',
    serif: 'Serif',
    sans: 'Sans Serif',
    mono: 'Monospace',
    inter: 'Inter',
    merriweather: 'Merriweather',
    literata: 'Literata',
    jetbrains: 'JetBrains Mono',
    fira: 'Fira Code',
    opendyslexic: 'OpenDyslexic'
};

// Constraints
const MIN_FONT_SIZE = 12;
const MAX_FONT_SIZE = 32;
const MIN_LINE_HEIGHT = 1.0;
const MAX_LINE_HEIGHT = 3.0;
const MIN_TEXT_WIDTH = 400;
const MAX_TEXT_WIDTH = 1400;

/**
 * Apply font size
 * @param {number} size - Font size in pixels
 */
export function applyFontSize(size) {
    size = clamp(size, MIN_FONT_SIZE, MAX_FONT_SIZE);
    document.documentElement.style.setProperty('--font-size', size + 'px');
    return size;
}

/**
 * Change font size by delta
 * @param {number} delta - Amount to change
 * @param {number} currentSize - Current font size
 * @returns {number} New font size
 */
export function changeFontSize(delta, currentSize) {
    const newSize = clamp(currentSize + delta, MIN_FONT_SIZE, MAX_FONT_SIZE);
    applyFontSize(newSize);
    showToast(`Font size: ${newSize}px`);
    saveSetting('fontSize', newSize);
    return newSize;
}

/**
 * Apply line height
 * @param {number} height - Line height value
 */
export function applyLineHeight(height) {
    height = clamp(height, MIN_LINE_HEIGHT, MAX_LINE_HEIGHT);
    document.documentElement.style.setProperty('--line-height', height);
    return height;
}

/**
 * Apply paragraph spacing
 * @param {number} spacing - Spacing in em units
 */
export function applyParagraphSpacing(spacing) {
    spacing = Math.max(0, spacing);
    document.documentElement.style.setProperty('--para-spacing', spacing + 'em');
    return spacing;
}

/**
 * Apply text width
 * @param {number|string} width - Width in pixels or preset name
 */
export function applyTextWidth(width) {
    let widthValue;

    if (typeof width === 'number') {
        widthValue = clamp(width, MIN_TEXT_WIDTH, MAX_TEXT_WIDTH) + 'px';
    } else if (typeof width === 'string') {
        const widths = { narrow: '600px', medium: '800px', wide: '1000px', full: '100%' };
        widthValue = widths[width] || width;
    }

    document.documentElement.style.setProperty('--text-width', widthValue);
    return widthValue;
}

/**
 * Apply font family
 * @param {string} family - Font family key
 */
export function applyFontFamily(family) {
    const fontStack = FONT_FAMILIES[family] || FONT_FAMILIES.serif;
    document.documentElement.style.setProperty('--font-family-reading', fontStack);
    return family;
}

/**
 * Set font family with feedback
 * @param {string} family - Font family key
 */
export function setFontFamily(family) {
    applyFontFamily(family);
    saveSetting('fontFamily', family);
    const displayName = FONT_FAMILY_NAMES[family] || family;
    showToast(`Font: ${displayName}`);
}

/**
 * Apply all typography settings at once
 * @param {Object} settings - Typography settings
 */
export function applyTypography(settings) {
    const root = document.documentElement;

    if (settings.fontSize !== undefined) {
        root.style.setProperty('--font-size', settings.fontSize + 'px');
    }

    if (settings.lineHeight !== undefined) {
        root.style.setProperty('--line-height', settings.lineHeight);
    }

    if (settings.paraSpacing !== undefined) {
        root.style.setProperty('--para-spacing', settings.paraSpacing + 'em');
    }

    if (settings.textWidth !== undefined) {
        const textWidth = typeof settings.textWidth === 'number'
            ? settings.textWidth + 'px'
            : settings.textWidth;
        root.style.setProperty('--text-width', textWidth);
    }

    if (settings.fontFamily !== undefined) {
        const fontStack = FONT_FAMILIES[settings.fontFamily] || FONT_FAMILIES.serif;
        root.style.setProperty('--font-family-reading', fontStack);
    }
}
