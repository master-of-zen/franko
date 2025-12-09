/**
 * Franko Reader - Animations Feature
 * Handles card animations and intersection observers
 */

/**
 * Initialize card animations with intersection observer
 */
export function initAnimations() {
    const cards = document.querySelectorAll('.book-card, .library-card');

    if ('IntersectionObserver' in window) {
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.style.opacity = '1';
                    entry.target.style.transform = 'translateY(0)';
                }
            });
        }, { threshold: 0.1 });

        cards.forEach(card => {
            card.style.opacity = '0';
            card.style.transform = 'translateY(20px)';
            card.style.transition = 'opacity 0.4s ease, transform 0.4s ease';
            observer.observe(card);
        });
    } else {
        // Fallback: just show all cards
        cards.forEach(card => {
            card.style.opacity = '1';
        });
    }
}

/**
 * Create a fade-in animation for an element
 * @param {HTMLElement} element - Element to animate
 * @param {number} [delay=0] - Animation delay in ms
 */
export function fadeIn(element, delay = 0) {
    element.style.opacity = '0';
    element.style.transform = 'translateY(10px)';
    element.style.transition = `opacity 0.3s ease ${delay}ms, transform 0.3s ease ${delay}ms`;

    requestAnimationFrame(() => {
        requestAnimationFrame(() => {
            element.style.opacity = '1';
            element.style.transform = 'translateY(0)';
        });
    });
}

/**
 * Create a staggered fade-in for multiple elements
 * @param {NodeList|Array} elements - Elements to animate
 * @param {number} [staggerDelay=50] - Delay between each element
 */
export function staggerFadeIn(elements, staggerDelay = 50) {
    elements.forEach((el, index) => {
        fadeIn(el, index * staggerDelay);
    });
}

/**
 * Animate element removal
 * @param {HTMLElement} element - Element to remove
 * @returns {Promise} Resolves when animation completes
 */
export function fadeOutAndRemove(element) {
    return new Promise(resolve => {
        element.style.transition = 'opacity 0.3s ease, transform 0.3s ease';
        element.style.opacity = '0';
        element.style.transform = 'translateY(-10px)';

        setTimeout(() => {
            element.remove();
            resolve();
        }, 300);
    });
}
