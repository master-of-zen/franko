/**
 * Franko Reader - Auto Scroll Feature
 * Handles automatic scrolling for hands-free reading
 */

import { saveSetting } from '../core/storage.js';
import { createElement } from '../core/dom.js';

let autoScrollSpeed = 0;
let autoScrollInterval = null;
let indicatorElement = null;

/**
 * Start auto-scrolling
 * @param {number} speed - Scroll speed (1-10)
 */
export function startAutoScroll(speed) {
    stopAutoScroll();
    autoScrollSpeed = speed;

    const layout = document.getElementById('reader-container')?.dataset?.layout;

    if (speed > 0 && layout === 'scroll') {
        const pixelsPerSecond = speed * 10;
        autoScrollInterval = setInterval(() => {
            window.scrollBy(0, pixelsPerSecond / 60);

            // Check if reached bottom
            if ((window.innerHeight + window.scrollY) >= document.documentElement.scrollHeight) {
                stopAutoScroll();
            }
        }, 1000 / 60);

        showAutoScrollIndicator(true);
    }

    saveSetting('autoScrollSpeed', speed);
}

/**
 * Stop auto-scrolling
 */
export function stopAutoScroll() {
    if (autoScrollInterval) {
        clearInterval(autoScrollInterval);
        autoScrollInterval = null;
    }
    autoScrollSpeed = 0;
    showAutoScrollIndicator(false);
}

/**
 * Get current auto-scroll speed
 * @returns {number} Current speed
 */
export function getAutoScrollSpeed() {
    return autoScrollSpeed;
}

/**
 * Adjust auto-scroll speed
 * @param {number} delta - Speed adjustment (-1 or +1)
 */
export function adjustAutoScrollSpeed(delta) {
    const newSpeed = Math.max(0, Math.min(10, autoScrollSpeed + delta));
    if (newSpeed === 0) {
        stopAutoScroll();
    } else {
        startAutoScroll(newSpeed);
    }
}

/**
 * Show or hide auto-scroll indicator
 * @param {boolean} show - Whether to show the indicator
 */
function showAutoScrollIndicator(show) {
    if (show) {
        if (!indicatorElement) {
            indicatorElement = createElement('div', {
                className: 'auto-scroll-indicator active'
            });

            const label = createElement('span', {}, 'Auto-scrolling');
            const stopBtn = createElement('button', {
                onClick: stopAutoScroll
            }, 'Stop');

            indicatorElement.appendChild(label);
            indicatorElement.appendChild(stopBtn);
            document.body.appendChild(indicatorElement);
        }
        indicatorElement.classList.add('active');
    } else if (indicatorElement) {
        indicatorElement.classList.remove('active');
    }
}

// Expose stop function globally for the indicator button
window.frankoStopAutoScroll = stopAutoScroll;
