/**
 * Franko Reader - Progress Feature
 * Tracks reading progress, chapters, and word counts
 */

import { throttle, formatWordCount, getBookId } from '../core/utils.js';
import { saveProgress as saveProgressToStorage, loadProgress as loadProgressFromStorage } from '../core/storage.js';
import { elements } from '../core/dom.js';

// State
let currentChapter = 0;
let totalBookWords = 0;
let chapterWordCounts = [];

/**
 * Initialize word counts from container data attributes
 */
export function initWordCounts() {
    const { readerContainer } = elements;
    if (!readerContainer) return;

    totalBookWords = parseInt(readerContainer.dataset.totalWords) || 0;
    try {
        chapterWordCounts = JSON.parse(readerContainer.dataset.chapterWords || '[]');
    } catch (e) {
        chapterWordCounts = [];
    }
}

/**
 * Get current chapter index
 */
export function getCurrentChapter() {
    return currentChapter;
}

/**
 * Initialize chapter tracking
 */
export function initChapterTracking() {
    initWordCounts();

    const chapters = document.querySelectorAll('.chapter');
    if (chapters.length === 0) return;

    const updateProgress = throttle(() => {
        const scrollTop = window.scrollY;
        const scrollHeight = document.documentElement.scrollHeight - window.innerHeight;
        const scrollPercent = scrollHeight > 0 ? scrollTop / scrollHeight : 0;

        // Update current chapter
        let newChapter = 0;
        let chapterScrollPercent = 0;

        chapters.forEach((chapter, index) => {
            if (chapter.offsetTop <= scrollTop + 100) {
                newChapter = index;
            }
        });

        // Calculate chapter-specific progress
        const currentChapterEl = chapters[newChapter];
        const nextChapterEl = chapters[newChapter + 1];

        if (currentChapterEl) {
            const chapterTop = currentChapterEl.offsetTop;
            const chapterBottom = nextChapterEl
                ? nextChapterEl.offsetTop
                : document.documentElement.scrollHeight;
            const chapterHeight = chapterBottom - chapterTop;
            const scrollInChapter = scrollTop - chapterTop + 100;
            chapterScrollPercent = Math.max(0, Math.min(1, scrollInChapter / chapterHeight));
        }

        // Calculate words read
        let wordsReadInPreviousChapters = 0;
        for (let i = 0; i < newChapter; i++) {
            wordsReadInPreviousChapters += chapterWordCounts[i] || 0;
        }

        const currentChapterWords = chapterWordCounts[newChapter] || 0;
        const wordsReadInCurrentChapter = Math.round(currentChapterWords * chapterScrollPercent);
        const totalWordsRead = wordsReadInPreviousChapters + wordsReadInCurrentChapter;

        // Update progress display
        updateProgressDisplay({
            percent: scrollPercent * 100,
            wordsRead: totalWordsRead,
            totalWords: totalBookWords,
            chapter: newChapter,
            chapterCount: chapters.length,
            chapterWordsRead: wordsReadInCurrentChapter,
            chapterTotalWords: currentChapterWords
        });

        if (newChapter !== currentChapter) {
            currentChapter = newChapter;
            updateTocHighlight();
        }
    }, 100);

    window.addEventListener('scroll', updateProgress);
    updateProgress(); // Initial call
}

/**
 * Update all progress indicators
 * @param {Object} stats - Progress statistics
 */
export function updateProgressDisplay(stats) {
    const { progressFill } = elements;

    // Update progress bar
    if (progressFill) {
        progressFill.style.width = stats.percent + '%';
    }

    // Update percentage
    const percentEl = document.getElementById('progress-percent');
    if (percentEl) {
        percentEl.textContent = Math.round(stats.percent) + '%';
    }

    // Update words read
    const wordsReadEl = document.getElementById('progress-words-read');
    if (wordsReadEl) {
        wordsReadEl.textContent = `${formatWordCount(stats.wordsRead)} / ${formatWordCount(stats.totalWords)} words`;
    }

    // Update chapter indicator
    const chapterEl = document.getElementById('progress-chapter');
    if (chapterEl) {
        const chapters = document.querySelectorAll('.chapter');
        const chapter = chapters[stats.chapter];
        const title = chapter?.querySelector('.chapter-title');
        const titleText = title ? title.textContent : `Chapter ${stats.chapter + 1}`;
        chapterEl.textContent = `${titleText} (${stats.chapter + 1}/${stats.chapterCount})`;
    }

    // Update chapter words
    const chapterWordsEl = document.getElementById('progress-chapter-words');
    if (chapterWordsEl) {
        chapterWordsEl.textContent = `${formatWordCount(stats.chapterWordsRead)} / ${formatWordCount(stats.chapterTotalWords)} in chapter`;
    }
}

/**
 * Update TOC highlight based on current chapter
 */
export function updateTocHighlight() {
    const tocLinks = document.querySelectorAll('.toc a');
    tocLinks.forEach((link, index) => {
        const li = link.parentElement;
        if (index === currentChapter) {
            li.classList.add('active');
        } else {
            li.classList.remove('active');
        }
    });
}

/**
 * Update basic scroll progress
 */
export function updateScrollProgress() {
    const { progressFill } = elements;
    if (!progressFill) return;

    const scrollTop = window.scrollY;
    const docHeight = document.documentElement.scrollHeight - window.innerHeight;
    const progress = docHeight > 0 ? (scrollTop / docHeight) * 100 : 0;

    progressFill.style.width = progress + '%';
}

/**
 * Save reading progress to storage and API
 */
export function saveBookProgress() {
    const bookId = getBookId();
    if (!bookId) return;

    const scrollPosition = window.scrollY;
    const docHeight = document.documentElement.scrollHeight - window.innerHeight;
    const progressPercent = docHeight > 0 ? Math.min(1.0, scrollPosition / docHeight) : 0;

    const progress = {
        scroll: scrollPosition,
        chapter: currentChapter
    };

    // Save to local storage
    saveProgressToStorage(bookId, progress);

    // Try to save to API
    fetch(`/api/books/${bookId}/progress`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            chapter: currentChapter,
            block: 0,
            scroll_offset: scrollPosition,
            progress: progressPercent
        })
    }).catch(() => {
        // Ignore errors, local storage is the fallback
    });
}

/**
 * Load reading progress from API or storage
 */
export function loadBookProgress() {
    const bookId = getBookId();
    if (!bookId) return;

    // Try to load from server first
    fetch(`/api/books/${bookId}/progress`)
        .then(response => response.json())
        .then(data => {
            if (data.success && data.data && data.data.scroll_offset) {
                setTimeout(() => {
                    window.scrollTo(0, data.data.scroll_offset);
                }, 100);
            } else {
                loadProgressFromLocal();
            }
        })
        .catch(() => {
            loadProgressFromLocal();
        });
}

/**
 * Load progress from local storage
 */
function loadProgressFromLocal() {
    const bookId = getBookId();
    if (!bookId) return;

    const progress = loadProgressFromStorage(bookId);
    if (progress && progress.scroll) {
        window.scrollTo(0, progress.scroll);
    }
}

/**
 * Start periodic progress saving
 * @param {number} [interval=30000] - Save interval in milliseconds
 */
export function startProgressAutoSave(interval = 30000) {
    setInterval(() => {
        if (getBookId()) {
            saveBookProgress();
        }
    }, interval);
}
