/**
 * Franko Reader - Search Feature
 * Handles search functionality in library and book content
 */

import { debounce } from '../core/utils.js';
import { elements } from '../core/dom.js';

/**
 * Handle search input for library cards
 * @param {Event} e - Input event
 */
function handleLibrarySearch(e) {
    const query = e.target.value.toLowerCase().trim();
    const cards = document.querySelectorAll('.book-card, .library-card, .library-table tbody tr');

    cards.forEach(card => {
        const title = card.querySelector('h3, .title, td:first-child')?.textContent?.toLowerCase() || '';
        const author = card.querySelector('.author, td:nth-child(2)')?.textContent?.toLowerCase() || '';

        if (title.includes(query) || author.includes(query) || query === '') {
            card.style.display = '';
            card.style.opacity = '1';
        } else {
            card.style.display = 'none';
        }
    });
}

/**
 * Initialize search functionality
 */
export function initSearch() {
    const { searchInput } = elements;

    if (searchInput) {
        searchInput.addEventListener('input', debounce(handleLibrarySearch, 300));
    }
}

/**
 * Highlight search terms in content
 * @param {string} text - Text content
 * @param {string} query - Search query
 * @returns {string} HTML with highlighted terms
 */
export function highlightSearchTerms(text, query) {
    if (!query || !text) return text;

    const regex = new RegExp(`(${escapeRegExp(query)})`, 'gi');
    return text.replace(regex, '<mark class="search-highlight">$1</mark>');
}

/**
 * Escape special regex characters
 * @param {string} string - String to escape
 * @returns {string} Escaped string
 */
function escapeRegExp(string) {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

/**
 * Search within reader content
 * @param {string} query - Search query
 * @returns {Array} Array of match positions
 */
export function searchInContent(query) {
    const content = document.getElementById('content');
    if (!content || !query) return [];

    const matches = [];
    const walker = document.createTreeWalker(
        content,
        NodeFilter.SHOW_TEXT,
        null,
        false
    );

    let node;
    while (node = walker.nextNode()) {
        const text = node.textContent.toLowerCase();
        let index = text.indexOf(query.toLowerCase());

        while (index !== -1) {
            matches.push({
                node,
                index,
                text: node.textContent.substring(index, index + query.length)
            });
            index = text.indexOf(query.toLowerCase(), index + 1);
        }
    }

    return matches;
}

/**
 * Navigate to a search match
 * @param {Object} match - Match object from searchInContent
 */
export function navigateToMatch(match) {
    if (!match || !match.node) return;

    const range = document.createRange();
    range.setStart(match.node, match.index);
    range.setEnd(match.node, match.index + match.text.length);

    const rect = range.getBoundingClientRect();
    window.scrollTo({
        top: window.scrollY + rect.top - window.innerHeight / 3,
        behavior: 'smooth'
    });
}
