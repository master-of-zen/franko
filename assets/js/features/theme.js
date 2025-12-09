/**
 * Franko Reader - Theme Feature
 * Handles theme switching and custom colors
 */

import { adjustColor } from '../core/utils.js';
import { saveSetting } from '../core/storage.js';
import { showToast } from '../core/toast.js';

// All available themes
export const ALL_THEMES = [
    // Light themes
    'light', 'paper', 'sepia', 'solarized-light', 'gruvbox-light',
    'catppuccin-latte', 'github-light', 'rose-pine-dawn', 'everforest-light',
    'atom-one-light', 'ayu-light', 'night-owl-light', 'flexoki-light',
    // Dark themes
    'dark', 'tokyo-night', 'dracula', 'nord', 'one-dark', 'atom-one-dark', 'monokai',
    'solarized-dark', 'gruvbox-dark', 'catppuccin-mocha', 'catppuccin-macchiato',
    'catppuccin-frappe', 'github-dark', 'rose-pine', 'rose-pine-moon',
    'everforest-dark', 'kanagawa', 'material-dark', 'night-owl', 'palenight',
    'shades-of-purple', 'ayu-dark', 'ayu-mirage', 'horizon', 'cobalt2',
    'synthwave84', 'iceberg', 'zenburn', 'poimandres', 'vesper',
    'flexoki-dark', 'oxocarbon-dark', 'amoled', 'high-contrast',
    // E-Reader themes
    'kindle', 'kobo',
    // Night reading themes
    'midnight-blue', 'warm-night',
    // Custom
    'custom'
];

/**
 * Apply a theme to the document
 * @param {string} theme - Theme name
 * @param {Object} [customColors] - Custom colors for 'custom' theme
 */
export function applyTheme(theme, customColors = null) {
    const root = document.documentElement;

    // Remove all theme classes
    ALL_THEMES.forEach(t => root.classList.remove(t));

    // Add new theme class
    root.classList.add(theme);

    // Apply custom colors when custom theme is selected
    if (theme === 'custom' && customColors) {
        applyCustomColors(customColors);
    } else {
        // Remove custom color overrides when using preset themes
        clearCustomColors();
    }
}

/**
 * Apply custom theme colors
 * @param {Object} colors - Custom color values
 */
export function applyCustomColors(colors) {
    const root = document.documentElement;

    if (colors.background) {
        root.style.setProperty('--bg-primary', colors.background);
        root.style.setProperty('--bg-secondary', adjustColor(colors.background, 10));
        root.style.setProperty('--bg-tertiary', adjustColor(colors.background, 20));
    }

    if (colors.text) {
        root.style.setProperty('--text-primary', colors.text);
        root.style.setProperty('--text-secondary', adjustColor(colors.text, -20));
    }

    if (colors.accent) {
        root.style.setProperty('--accent-primary', colors.accent);
        root.style.setProperty('--accent-secondary', adjustColor(colors.accent, 15));
    }

    if (colors.link) {
        root.style.setProperty('--link-color', colors.link);
    }
}

/**
 * Clear custom color overrides
 */
export function clearCustomColors() {
    const root = document.documentElement;
    const props = [
        '--bg-primary', '--bg-secondary', '--bg-tertiary',
        '--text-primary', '--text-secondary',
        '--accent-primary', '--accent-secondary', '--link-color'
    ];
    props.forEach(prop => root.style.removeProperty(prop));
}

/**
 * Toggle between light and dark theme
 */
export function toggleTheme() {
    const html = document.documentElement;
    const isDark = html.classList.contains('dark') || !html.classList.contains('light');

    if (isDark) {
        html.classList.remove('dark');
        html.classList.add('light');
        showToast('Light mode');
    } else {
        html.classList.remove('light');
        html.classList.add('dark');
        showToast('Dark mode');
    }

    saveSetting('theme', isDark ? 'light' : 'dark');
}

/**
 * Set a specific theme
 * @param {string} theme - Theme name
 */
export function setTheme(theme) {
    const root = document.documentElement;
    root.classList.remove('dark', 'light', 'auto');

    if (theme !== 'auto') {
        root.classList.add(theme);
    }

    saveSetting('theme', theme);
    showToast(`Theme: ${theme.charAt(0).toUpperCase() + theme.slice(1)}`);
}

/**
 * Set accent color
 * @param {string} color - Color name (indigo, purple, pink, etc.)
 */
export function setAccentColor(color) {
    const colors = {
        indigo: { primary: '#6366f1', secondary: '#818cf8' },
        purple: { primary: '#a855f7', secondary: '#c084fc' },
        pink: { primary: '#ec4899', secondary: '#f472b6' },
        blue: { primary: '#3b82f6', secondary: '#60a5fa' },
        teal: { primary: '#14b8a6', secondary: '#2dd4bf' },
        green: { primary: '#22c55e', secondary: '#4ade80' },
        orange: { primary: '#f97316', secondary: '#fb923c' },
        red: { primary: '#ef4444', secondary: '#f87171' }
    };

    if (colors[color]) {
        const root = document.documentElement;
        root.style.setProperty('--accent-primary', colors[color].primary);
        root.style.setProperty('--accent-secondary', colors[color].secondary);
        saveSetting('accentColor', color);
        showToast(`Accent: ${color.charAt(0).toUpperCase() + color.slice(1)}`);
    }
}
