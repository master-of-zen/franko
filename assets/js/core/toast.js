/**
 * Franko Reader - Toast Notifications
 * Simple toast notification system
 */

let toastElement = null;
let toastTimeout = null;

/**
 * Show a toast notification
 * @param {string} message - Message to display
 * @param {number} [duration=2000] - Duration in milliseconds
 */
export function showToast(message, duration = 2000) {
    if (!toastElement) {
        toastElement = document.createElement('div');
        toastElement.className = 'toast';
        toastElement.style.cssText = `
            position: fixed;
            bottom: 2rem;
            left: 50%;
            transform: translateX(-50%) translateY(20px);
            padding: 0.75rem 1.5rem;
            background: var(--bg-elevated, #2a2a2a);
            color: var(--text-primary, #fff);
            border-radius: 9999px;
            font-size: 0.875rem;
            font-weight: 500;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            opacity: 0;
            transition: all 0.3s ease;
            z-index: 1000;
            pointer-events: none;
        `;
        document.body.appendChild(toastElement);
    }

    // Clear existing timeout
    if (toastTimeout) {
        clearTimeout(toastTimeout);
    }

    // Show toast
    toastElement.textContent = message;
    toastElement.style.opacity = '1';
    toastElement.style.transform = 'translateX(-50%) translateY(0)';

    // Hide after duration
    toastTimeout = setTimeout(() => {
        toastElement.style.opacity = '0';
        toastElement.style.transform = 'translateX(-50%) translateY(20px)';
    }, duration);
}

/**
 * Hide the toast immediately
 */
export function hideToast() {
    if (toastElement) {
        toastElement.style.opacity = '0';
        toastElement.style.transform = 'translateX(-50%) translateY(20px)';
    }
    if (toastTimeout) {
        clearTimeout(toastTimeout);
        toastTimeout = null;
    }
}
