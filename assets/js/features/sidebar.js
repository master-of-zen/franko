/**
 * Franko Reader - Sidebar Feature
 * Handles sidebar toggle and TOC navigation
 */

import { elements, createElement } from '../core/dom.js';

let sidebarOverlay = null;

/**
 * Toggle sidebar visibility
 */
export function toggleSidebar() {
    const { sidebar } = elements;
    if (!sidebar) return;

    sidebar.classList.toggle('open');

    if (sidebar.classList.contains('open')) {
        showSidebarOverlay();
    } else {
        hideSidebarOverlay();
    }
}

/**
 * Open sidebar
 */
export function openSidebar() {
    const { sidebar } = elements;
    if (sidebar && !sidebar.classList.contains('open')) {
        sidebar.classList.add('open');
        showSidebarOverlay();
    }
}

/**
 * Close sidebar
 */
export function closeSidebar() {
    const { sidebar } = elements;
    if (sidebar && sidebar.classList.contains('open')) {
        sidebar.classList.remove('open');
        hideSidebarOverlay();
    }
}

/**
 * Show sidebar overlay
 */
function showSidebarOverlay() {
    if (!sidebarOverlay) {
        sidebarOverlay = createElement('div', {
            className: 'sidebar-overlay',
            style: {
                position: 'fixed',
                inset: '0',
                background: 'rgba(0, 0, 0, 0.5)',
                zIndex: '150',
                opacity: '0',
                transition: 'opacity 0.3s ease'
            },
            onClick: toggleSidebar
        });
        document.body.appendChild(sidebarOverlay);
    }

    // Trigger reflow for animation
    setTimeout(() => {
        sidebarOverlay.style.opacity = '1';
    }, 10);
}

/**
 * Hide sidebar overlay
 */
function hideSidebarOverlay() {
    if (sidebarOverlay) {
        sidebarOverlay.style.opacity = '0';
        setTimeout(() => {
            if (sidebarOverlay) {
                sidebarOverlay.remove();
                sidebarOverlay = null;
            }
        }, 300);
    }
}

/**
 * Initialize TOC navigation
 */
export function initTocNavigation() {
    const tocLinks = document.querySelectorAll('.toc a');

    tocLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const chapterIndex = parseInt(link.dataset.chapter);
            const chapter = document.getElementById(`chapter-${chapterIndex}`);

            if (chapter) {
                chapter.scrollIntoView({ behavior: 'smooth', block: 'start' });
                closeSidebar();
            }
        });
    });
}

/**
 * Initialize sidebar event listeners
 */
export function initSidebar() {
    const { toggleSidebarBtn, closeSidebarBtn, sidebar } = elements;

    if (toggleSidebarBtn) {
        toggleSidebarBtn.addEventListener('click', toggleSidebar);
    }

    if (closeSidebarBtn) {
        closeSidebarBtn.addEventListener('click', toggleSidebar);
    }

    // Close sidebar when clicking outside
    document.addEventListener('click', (e) => {
        if (sidebar && sidebar.classList.contains('open') &&
            !sidebar.contains(e.target) &&
            e.target !== toggleSidebarBtn) {
            closeSidebar();
        }
    });

    // Close sidebar on mobile when selecting TOC item
    document.querySelectorAll('.toc a').forEach(link => {
        link.addEventListener('click', closeSidebar);
    });

    initTocNavigation();
}
