/**
 * Franko Reader - Layout Feature
 * Handles scroll/paged/dual layout modes
 */

import { saveSetting } from '../core/storage.js';
import { showToast } from '../core/toast.js';
import { elements } from '../core/dom.js';

// Layout state
let currentLayout = 'scroll';
let currentPage = 0;
let totalPages = 1;
let pagesPerView = 1;
let pageGap = 40;
let pageAnimation = 'slide';
let originalContent = '';
let dualPages = null;

/**
 * Initialize layout module
 * @param {string} [savedLayout] - Previously saved layout
 */
export function initLayout(savedLayout = 'scroll') {
    const { content } = elements;

    // Store original content for paged mode
    if (content) {
        originalContent = content.innerHTML;
    }

    // Apply saved layout after a short delay
    if (savedLayout && savedLayout !== 'scroll') {
        setTimeout(() => {
            setLayout(savedLayout);
        }, 100);
    }
}

/**
 * Get current layout state
 */
export function getLayoutState() {
    return {
        layout: currentLayout,
        page: currentPage,
        totalPages,
        pagesPerView,
        pageGap,
        pageAnimation
    };
}

/**
 * Set layout mode
 * @param {string} layout - Layout mode ('scroll', 'paged', 'dual')
 */
export function setLayout(layout) {
    const { readerContainer, pageControls } = elements;

    currentLayout = layout;

    if (readerContainer) {
        readerContainer.dataset.layout = layout;
    }

    // Show/hide page controls
    if (pageControls) {
        pageControls.style.display = layout === 'scroll' ? 'none' : 'flex';
    }

    // Handle layout-specific setup
    if (layout === 'scroll') {
        exitPagedMode();
    } else {
        enterPagedMode(layout);
    }

    saveSetting('layoutMode', layout);
    showToast(`Layout: ${layout.charAt(0).toUpperCase() + layout.slice(1)}`);

    // Update layout buttons
    document.querySelectorAll('.layout-btn').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.layout === layout);
    });
}

/**
 * Enter paged mode
 * @param {string} layout - 'paged' or 'dual'
 */
function enterPagedMode(layout) {
    const { content, readerContainer } = elements;
    if (!content || !readerContainer) return;

    pagesPerView = layout === 'dual' ? 2 : 1;
    readerContainer.dataset.pages = pagesPerView;

    // Disable scrolling
    readerContainer.style.overflow = 'hidden';

    // Paginate content based on layout
    if (layout === 'dual') {
        paginateDualMode();
    } else {
        paginateContent();
    }

    // Update page indicator
    updatePageIndicator();
}

/**
 * Exit paged mode
 */
function exitPagedMode() {
    const { content, readerContainer } = elements;
    if (!content || !readerContainer) return;

    // Remove dual page containers
    const dualContainer = readerContainer.querySelector('.dual-page-container');
    if (dualContainer) {
        dualContainer.remove();
    }
    readerContainer.classList.remove('dual-active');

    // Restore original content
    content.innerHTML = originalContent;
    content.style.cssText = '';
    content.style.display = '';

    // Re-enable scrolling
    readerContainer.style.overflow = '';
    readerContainer.dataset.pages = '1';

    // Reset state
    currentPage = 0;
    totalPages = 1;
}

/**
 * Paginate content for dual page mode
 */
function paginateDualMode() {
    const { content, readerContainer } = elements;
    if (!content) return;

    const containerHeight = readerContainer.clientHeight - 40;
    const containerWidth = (readerContainer.clientWidth - pageGap) / 2;

    // Hide original content
    content.style.display = 'none';
    readerContainer.classList.add('dual-active');

    // Remove existing dual container
    const existingDual = readerContainer.querySelector('.dual-page-container');
    if (existingDual) {
        existingDual.remove();
    }

    // Create dual page container
    const dualContainer = document.createElement('div');
    dualContainer.className = 'dual-page-container';

    const leftPage = document.createElement('div');
    leftPage.className = 'page-left';
    const rightPage = document.createElement('div');
    rightPage.className = 'page-right';

    dualContainer.appendChild(leftPage);
    dualContainer.appendChild(rightPage);
    readerContainer.appendChild(dualContainer);

    // Parse content into blocks
    const tempDiv = document.createElement('div');
    tempDiv.innerHTML = originalContent;
    const blocks = Array.from(tempDiv.children);

    // Calculate how much content fits per page
    const pages = [];
    let currentPageBlocks = [];
    let currentHeight = 0;

    const measureDiv = document.createElement('div');
    measureDiv.style.cssText = `
        position: absolute;
        visibility: hidden;
        width: ${containerWidth - 80}px;
        font-family: var(--font-family-reading);
        font-size: var(--font-size);
        line-height: var(--line-height);
        padding: 2.5rem;
    `;
    document.body.appendChild(measureDiv);

    blocks.forEach(block => {
        measureDiv.innerHTML = '';
        const clonedBlock = block.cloneNode(true);
        measureDiv.appendChild(clonedBlock);
        const blockHeight = measureDiv.offsetHeight;

        if (currentHeight + blockHeight > containerHeight && currentPageBlocks.length > 0) {
            pages.push(currentPageBlocks);
            currentPageBlocks = [];
            currentHeight = 0;
        }

        currentPageBlocks.push(block.outerHTML);
        currentHeight += blockHeight;
    });

    if (currentPageBlocks.length > 0) {
        pages.push(currentPageBlocks);
    }

    document.body.removeChild(measureDiv);

    // Store pages for navigation
    dualPages = pages;
    totalPages = Math.ceil(pages.length / 2);
    currentPage = 0;

    // Show first spread
    showDualSpread(0);
}

/**
 * Show a spread (two pages) in dual mode
 * @param {number} spreadIndex - Spread index
 */
function showDualSpread(spreadIndex) {
    const { readerContainer } = elements;
    const dualContainer = readerContainer.querySelector('.dual-page-container');
    if (!dualContainer || !dualPages) return;

    const leftPage = dualContainer.querySelector('.page-left');
    const rightPage = dualContainer.querySelector('.page-right');

    const leftIndex = spreadIndex * 2;
    const rightIndex = spreadIndex * 2 + 1;

    leftPage.innerHTML = dualPages[leftIndex] ? dualPages[leftIndex].join('') : '';
    rightPage.innerHTML = dualPages[rightIndex] ? dualPages[rightIndex].join('') : '';
}

/**
 * Paginate content for single paged mode
 */
function paginateContent() {
    const { content, readerContainer } = elements;
    if (!content) return;

    const containerHeight = readerContainer.clientHeight - 60;
    const containerWidth = readerContainer.clientWidth;

    // Create a temporary container to measure content
    const tempContainer = document.createElement('div');
    tempContainer.style.cssText = `
        position: absolute;
        visibility: hidden;
        width: ${containerWidth - 80}px;
        font-family: var(--font-family-reading);
        font-size: var(--font-size);
        line-height: var(--line-height);
        padding: 2rem;
    `;
    tempContainer.innerHTML = originalContent;
    document.body.appendChild(tempContainer);

    const contentHeight = tempContainer.scrollHeight;
    totalPages = Math.max(1, Math.ceil(contentHeight / containerHeight));

    document.body.removeChild(tempContainer);

    // Set up CSS columns for pagination
    content.style.cssText = `
        height: ${containerHeight}px;
        column-count: ${totalPages};
        column-gap: ${pageGap}px;
        column-fill: auto;
        overflow: hidden;
    `;

    currentPage = 0;
    goToPage(0);
}

/**
 * Go to specific page
 * @param {number} pageNum - Page number
 */
export function goToPage(pageNum) {
    const { content, readerContainer } = elements;

    if (currentLayout === 'dual') {
        pageNum = Math.max(0, Math.min(pageNum, totalPages - 1));
        currentPage = pageNum;
        showDualSpread(pageNum);
        updatePageIndicator();
        updatePageButtons();
        saveSetting('currentPage', currentPage);
        return;
    }

    if (!content) return;

    pageNum = Math.max(0, Math.min(pageNum, totalPages - 1));
    currentPage = pageNum;

    const containerWidth = readerContainer.clientWidth;
    const pageWidth = containerWidth - 40;
    const offset = pageNum * pageWidth;

    // Apply animation class
    content.classList.remove('page-animation-slide', 'page-animation-fade', 'page-animation-flip');
    if (pageAnimation !== 'none') {
        content.classList.add(`page-animation-${pageAnimation}`);
    }

    content.style.transform = `translateX(-${offset}px)`;
    content.style.transition = 'transform 0.4s cubic-bezier(0.4, 0, 0.2, 1)';

    updatePageIndicator();
    updatePageButtons();
    saveSetting('currentPage', currentPage);
}

/**
 * Navigate to previous page
 */
export function prevPage() {
    if (currentPage > 0) {
        goToPage(currentPage - 1);
    }
}

/**
 * Navigate to next page
 */
export function nextPage() {
    if (currentPage < totalPages - 1) {
        goToPage(currentPage + 1);
    }
}

/**
 * Update page indicator text
 */
function updatePageIndicator() {
    const { pageIndicator } = elements;
    if (!pageIndicator) return;

    if (currentLayout === 'dual' && dualPages) {
        const leftPage = currentPage * 2 + 1;
        const rightPage = Math.min(currentPage * 2 + 2, dualPages.length);
        const totalPageCount = dualPages.length;

        if (rightPage > leftPage) {
            pageIndicator.textContent = `Pages ${leftPage}-${rightPage} of ${totalPageCount}`;
        } else {
            pageIndicator.textContent = `Page ${leftPage} of ${totalPageCount}`;
        }
    } else {
        const displayPage = currentPage + 1;
        pageIndicator.textContent = `Page ${displayPage} of ${totalPages}`;
    }
}

/**
 * Update page navigation buttons
 */
function updatePageButtons() {
    const { pagePrevBtn, pageNextBtn } = elements;

    if (pagePrevBtn) {
        pagePrevBtn.disabled = currentPage === 0;
    }
    if (pageNextBtn) {
        pageNextBtn.disabled = currentPage >= totalPages - 1;
    }
}

/**
 * Recalculate pages on resize
 */
export function recalculatePages() {
    if (currentLayout === 'dual') {
        paginateDualMode();
    } else if (currentLayout !== 'scroll') {
        paginateContent();
    }
}

/**
 * Set page animation type
 * @param {string} animation - Animation type ('slide', 'fade', 'flip', 'none')
 */
export function setPageAnimation(animation) {
    pageAnimation = animation;
    saveSetting('pageAnimation', animation);
}

/**
 * Set page gap
 * @param {number} gap - Gap in pixels
 */
export function setPageGap(gap) {
    pageGap = gap;
    saveSetting('pageGap', gap);

    if (currentLayout !== 'scroll') {
        recalculatePages();
    }
}
