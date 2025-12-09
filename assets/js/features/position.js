/**
 * Franko Reader - Position Preservation
 * Maintains reading position during settings changes
 */

/**
 * Get current reading position
 * @returns {Object|null} Position data
 */
export function getReadingPosition() {
    const contentEl = document.getElementById('content');
    if (!contentEl) return null;

    const scrollTop = window.scrollY;
    const viewportHeight = window.innerHeight;
    const viewportCenter = scrollTop + viewportHeight / 3; // Use top third of viewport

    // Find the element at the reading position
    const paragraphs = contentEl.querySelectorAll('p, h1, h2, h3, h4, h5, h6');
    let closestElement = null;
    let closestDistance = Infinity;

    paragraphs.forEach((el, index) => {
        const rect = el.getBoundingClientRect();
        const elementTop = rect.top + scrollTop;
        const distance = Math.abs(elementTop - viewportCenter);

        if (distance < closestDistance) {
            closestDistance = distance;
            const textContent = el.textContent || '';
            const firstWords = textContent.substring(0, 50);
            closestElement = {
                element: el,
                index: index,
                firstWords: firstWords,
                viewportCenterY: viewportCenter,
                elementTop: elementTop,
                pixelOffset: viewportCenter - elementTop
            };
        }
    });

    return closestElement;
}

/**
 * Restore reading position after settings change
 * @param {Object} position - Position data from getReadingPosition
 */
export function restoreReadingPosition(position) {
    if (!position) return;

    const contentEl = document.getElementById('content');
    if (!contentEl) return;

    const paragraphs = contentEl.querySelectorAll('p, h1, h2, h3, h4, h5, h6');

    // First try to find element by matching text content (more reliable)
    let targetEl = null;

    for (let i = 0; i < paragraphs.length; i++) {
        const el = paragraphs[i];
        const textContent = el.textContent || '';
        const firstWords = textContent.substring(0, 50);

        if (firstWords === position.firstWords) {
            targetEl = el;
            break;
        }
    }

    // Fall back to index if text match fails
    if (!targetEl && paragraphs[position.index]) {
        targetEl = paragraphs[position.index];
    }

    if (targetEl) {
        const rect = targetEl.getBoundingClientRect();
        const elementTop = rect.top + window.scrollY;
        const viewportHeight = window.innerHeight;

        // Position element at the top third of viewport
        const targetScroll = elementTop - viewportHeight / 3;

        window.scrollTo({ top: Math.max(0, targetScroll), behavior: 'instant' });
    }
}
