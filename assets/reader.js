// Franko Reader - JavaScript

(function () {
    'use strict';

    // DOM Elements
    const sidebar = document.getElementById('sidebar');
    const toggleSidebarBtn = document.getElementById('toggle-sidebar');
    const closeSidebarBtn = document.getElementById('close-sidebar');
    const content = document.getElementById('content');
    const progressFill = document.getElementById('progress-fill');
    const increaseFontBtn = document.getElementById('increase-font');
    const decreaseFontBtn = document.getElementById('decrease-font');
    const toggleThemeBtn = document.getElementById('toggle-theme');
    const toggleFullscreenBtn = document.getElementById('toggle-fullscreen');
    const searchInput = document.getElementById('search');
    const readerContainer = document.getElementById('reader-container');
    const pageControls = document.getElementById('page-controls');
    const pageIndicator = document.getElementById('page-indicator');
    const pagePrevBtn = document.getElementById('page-prev');
    const pageNextBtn = document.getElementById('page-next');

    // State
    let fontSize = parseInt(getComputedStyle(document.documentElement).getPropertyValue('--font-size')) || 16;
    const minFontSize = 12;
    const maxFontSize = 32;

    // Layout state
    let currentLayout = 'scroll';
    let currentPage = 0;
    let totalPages = 1;
    let pagesPerView = 1;
    let autoScrollSpeed = 0;
    let autoScrollInterval = null;
    let pageGap = 40;
    let pageAnimation = 'slide';
    let originalContent = '';
    let dualPages = null; // For dual page mode content storage
    let currentChapter = 0; // Track current chapter based on scroll

    // Reading settings state
    let readingSettings = {
        fontSize: 18,
        lineHeight: 1.8,
        textWidth: 800,
        fontFamily: 'serif',
        theme: 'dark',
        paraSpacing: 1,
        panelMinWidth: 250,
        panelMaxWidth: 600
    };

    // Initialize
    function init() {
        setupEventListeners();
        updateProgress();
        loadSettings();
        initAnimations();
        initSearch();
        initLayoutControls();
        initChapterTracking();
        initTocNavigation();
        initSettingsPanel();

        // Store original content for paged mode
        if (content) {
            originalContent = content.innerHTML;
        }
    }

    // ========== Settings Panel ==========
    function initSettingsPanel() {
        const settingsPanel = document.getElementById('settings-panel');
        const toggleBtn = document.getElementById('toggle-settings');
        const closeBtn = document.getElementById('close-settings');
        const resizeHandle = document.getElementById('settings-resize-handle');

        if (!settingsPanel) return;

        // Toggle settings panel
        if (toggleBtn) {
            toggleBtn.addEventListener('click', toggleSettingsPanel);
        }
        if (closeBtn) {
            closeBtn.addEventListener('click', toggleSettingsPanel);
        }

        // Panel resize functionality
        if (resizeHandle) {
            let isResizing = false;
            let startX = 0;
            let startWidth = 0;

            resizeHandle.addEventListener('mousedown', (e) => {
                isResizing = true;
                startX = e.clientX;
                startWidth = settingsPanel.offsetWidth;
                settingsPanel.classList.add('resizing');
                resizeHandle.classList.add('active');
                e.preventDefault();
            });

            document.addEventListener('mousemove', (e) => {
                if (!isResizing) return;
                const deltaX = startX - e.clientX;
                const minW = readingSettings.panelMinWidth || 250;
                const maxW = readingSettings.panelMaxWidth || 600;
                const newWidth = Math.min(maxW, Math.max(minW, startWidth + deltaX));
                settingsPanel.style.width = newWidth + 'px';
            });

            document.addEventListener('mouseup', () => {
                if (isResizing) {
                    isResizing = false;
                    settingsPanel.classList.remove('resizing');
                    resizeHandle.classList.remove('active');
                    // Save width preference
                    localStorage.setItem('franko-settings-panel-width', settingsPanel.style.width);
                }
            });

            // Restore saved width
            const savedWidth = localStorage.getItem('franko-settings-panel-width');
            if (savedWidth) {
                settingsPanel.style.width = savedWidth;
            }
        }

        // Font size controls - input and buttons
        const fontSizeInput = document.getElementById('font-size-input');

        document.getElementById('font-decrease')?.addEventListener('click', () => {
            adjustSetting('fontSize', -1);
        });
        document.getElementById('font-increase')?.addEventListener('click', () => {
            adjustSetting('fontSize', 1);
        });
        fontSizeInput?.addEventListener('change', (e) => {
            setSettingValue('fontSize', parseInt(e.target.value));
        });

        // Line height controls - input and buttons
        const lineHeightInput = document.getElementById('line-height-input');

        document.getElementById('line-height-decrease')?.addEventListener('click', () => {
            adjustSetting('lineHeight', -0.1);
        });
        document.getElementById('line-height-increase')?.addEventListener('click', () => {
            adjustSetting('lineHeight', 0.1);
        });
        lineHeightInput?.addEventListener('change', (e) => {
            setSettingValue('lineHeight', parseFloat(e.target.value));
        });

        // Paragraph spacing controls - input and buttons
        const paraSpacingInput = document.getElementById('para-spacing-input');

        document.getElementById('para-spacing-decrease')?.addEventListener('click', () => {
            adjustSetting('paraSpacing', -0.25);
        });
        document.getElementById('para-spacing-increase')?.addEventListener('click', () => {
            adjustSetting('paraSpacing', 0.25);
        });
        paraSpacingInput?.addEventListener('change', (e) => {
            setSettingValue('paraSpacing', parseFloat(e.target.value));
        });

        // Text width - input and preset buttons
        const textWidthInput = document.getElementById('text-width-input');

        document.getElementById('text-width-decrease')?.addEventListener('click', () => {
            const current = readingSettings.textWidth || 800;
            setCustomTextWidth(current - 50);
        });
        document.getElementById('text-width-increase')?.addEventListener('click', () => {
            const current = readingSettings.textWidth || 800;
            setCustomTextWidth(current + 50);
        });
        textWidthInput?.addEventListener('change', (e) => {
            setCustomTextWidth(parseInt(e.target.value));
        });

        // Text width preset buttons
        document.querySelectorAll('[data-width]').forEach(btn => {
            btn.addEventListener('click', () => {
                const widthValue = parseInt(btn.dataset.widthValue);
                setCustomTextWidth(widthValue);
                document.querySelectorAll('[data-width]').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
            });
        });

        // Font family select
        document.getElementById('font-family-select')?.addEventListener('change', (e) => {
            setFontFamily(e.target.value);
        });

        // Theme buttons
        document.querySelectorAll('[data-theme]').forEach(btn => {
            btn.addEventListener('click', () => {
                setReaderTheme(btn.dataset.theme);
                document.querySelectorAll('[data-theme]').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
            });
        });

        // Panel width limit controls
        const panelMinWidthInput = document.getElementById('panel-min-width-input');
        const panelMaxWidthInput = document.getElementById('panel-max-width-input');

        // Panel min width +/- buttons
        document.getElementById('panel-min-width-decrease')?.addEventListener('click', () => {
            let value = (readingSettings.panelMinWidth || 250) - 50;
            value = Math.max(200, Math.min(value, readingSettings.panelMaxWidth - 50));
            readingSettings.panelMinWidth = value;
            applyPanelWidthLimits();
            updateSettingsDisplay();
            saveReadingSettings();
        });
        document.getElementById('panel-min-width-increase')?.addEventListener('click', () => {
            let value = (readingSettings.panelMinWidth || 250) + 50;
            value = Math.max(200, Math.min(value, readingSettings.panelMaxWidth - 50));
            readingSettings.panelMinWidth = value;
            applyPanelWidthLimits();
            updateSettingsDisplay();
            saveReadingSettings();
        });
        panelMinWidthInput?.addEventListener('change', (e) => {
            let value = parseInt(e.target.value) || 200;
            value = Math.max(200, Math.min(value, readingSettings.panelMaxWidth - 50));
            readingSettings.panelMinWidth = value;
            applyPanelWidthLimits();
            updateSettingsDisplay();
            saveReadingSettings();
        });

        // Panel max width +/- buttons
        document.getElementById('panel-max-width-decrease')?.addEventListener('click', () => {
            let value = (readingSettings.panelMaxWidth || 600) - 50;
            value = Math.max(readingSettings.panelMinWidth + 50, Math.min(value, 1200));
            readingSettings.panelMaxWidth = value;
            applyPanelWidthLimits();
            updateSettingsDisplay();
            saveReadingSettings();
        });
        document.getElementById('panel-max-width-increase')?.addEventListener('click', () => {
            let value = (readingSettings.panelMaxWidth || 600) + 50;
            value = Math.max(readingSettings.panelMinWidth + 50, Math.min(value, 1200));
            readingSettings.panelMaxWidth = value;
            applyPanelWidthLimits();
            updateSettingsDisplay();
            saveReadingSettings();
        });
        panelMaxWidthInput?.addEventListener('change', (e) => {
            let value = parseInt(e.target.value) || 800;
            value = Math.max(readingSettings.panelMinWidth + 50, Math.min(value, 1200));
            readingSettings.panelMaxWidth = value;
            applyPanelWidthLimits();
            updateSettingsDisplay();
            saveReadingSettings();
        });

        // Load saved settings
        loadReadingSettings();
    }

    // Apply panel width limits to CSS and current width
    function applyPanelWidthLimits() {
        const panel = document.getElementById('settings-panel');
        if (!panel) return;

        const minW = readingSettings.panelMinWidth || 250;
        const maxW = readingSettings.panelMaxWidth || 600;

        panel.style.minWidth = minW + 'px';
        panel.style.maxWidth = maxW + 'px';

        // Clamp current width if needed
        const currentWidth = panel.offsetWidth;
        if (currentWidth < minW) {
            panel.style.width = minW + 'px';
        } else if (currentWidth > maxW) {
            panel.style.width = maxW + 'px';
        }
    }

    // Set a setting to a specific value (used by inputs)
    function setSettingValue(setting, value) {
        const position = getReadingPosition();

        // Ensure positive values for font size and line height
        if (setting === 'fontSize' && value < 1) value = 1;
        if (setting === 'lineHeight' && value < 0.1) value = 0.1;
        if (setting === 'paraSpacing' && value < 0) value = 0;

        readingSettings[setting] = value;
        applySettings();
        updateSettingsDisplay();
        saveReadingSettings();

        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                restoreReadingPosition(position);
            });
        });
    }

    // Set custom text width in pixels
    function setCustomTextWidth(widthPx) {
        const position = getReadingPosition();

        widthPx = Math.max(400, Math.min(1400, widthPx));
        readingSettings.textWidth = widthPx;
        applySettings();
        updateSettingsDisplay();
        saveReadingSettings();

        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                restoreReadingPosition(position);
            });
        });
    }

    function toggleSettingsPanel() {
        const panel = document.getElementById('settings-panel');
        if (panel) {
            panel.classList.toggle('open');

            // Handle overlay
            let overlay = document.querySelector('.settings-overlay');
            if (panel.classList.contains('open')) {
                if (!overlay) {
                    overlay = document.createElement('div');
                    overlay.className = 'settings-overlay';
                    overlay.addEventListener('click', toggleSettingsPanel);
                    document.body.appendChild(overlay);
                }
                setTimeout(() => overlay.classList.add('active'), 10);
            } else if (overlay) {
                overlay.classList.remove('active');
                setTimeout(() => overlay.remove(), 300);
            }
        }
    }

    // Get reading position relative to content
    function getReadingPosition() {
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
                // Store the element's text content hash and position within it
                // This is more stable than height-based offset when font changes
                const textContent = el.textContent || '';
                const firstWords = textContent.substring(0, 50); // First 50 chars as identifier
                closestElement = {
                    element: el,
                    index: index,
                    firstWords: firstWords,
                    // Calculate how far into the element the viewport center is (as line count estimate)
                    viewportCenterY: viewportCenter,
                    elementTop: elementTop,
                    pixelOffset: viewportCenter - elementTop
                };
            }
        });

        return closestElement;
    }

    // Restore reading position after settings change
    function restoreReadingPosition(position) {
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
            // Add a small offset based on how far into the element we were
            const targetScroll = elementTop - viewportHeight / 3;

            window.scrollTo({ top: Math.max(0, targetScroll), behavior: 'instant' });
        }
    }

    function adjustSetting(setting, delta) {
        const position = getReadingPosition();

        let newValue = readingSettings[setting] + delta;
        // Round to avoid floating point issues
        newValue = Math.round(newValue * 100) / 100;

        // Ensure minimum values
        if (setting === 'fontSize' && newValue < 1) newValue = 1;
        if (setting === 'lineHeight' && newValue < 0.1) newValue = 0.1;
        if (setting === 'paraSpacing' && newValue < 0) newValue = 0;

        readingSettings[setting] = newValue;
        applySettings();
        updateSettingsDisplay();
        saveReadingSettings();

        // Restore position after layout reflow
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                restoreReadingPosition(position);
            });
        });
    }

    function setFontFamily(family) {
        const position = getReadingPosition();

        readingSettings.fontFamily = family;
        applySettings();
        saveReadingSettings();

        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                restoreReadingPosition(position);
            });
        });
    }

    function setReaderTheme(theme) {
        readingSettings.theme = theme;
        applySettings();
        saveReadingSettings();
    }

    function applySettings() {
        const root = document.documentElement;

        // Font size
        root.style.setProperty('--font-size', readingSettings.fontSize + 'px');

        // Line height
        root.style.setProperty('--line-height', readingSettings.lineHeight);

        // Paragraph spacing
        root.style.setProperty('--para-spacing', readingSettings.paraSpacing + 'em');

        // Text width - now stores pixel value directly
        const textWidth = typeof readingSettings.textWidth === 'number'
            ? readingSettings.textWidth + 'px'
            : readingSettings.textWidth;
        root.style.setProperty('--text-width', textWidth);

        // Font family
        const fonts = {
            serif: 'Georgia, Cambria, "Times New Roman", Times, serif',
            sans: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
            mono: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace'
        };
        root.style.setProperty('--font-family-reading', fonts[readingSettings.fontFamily] || fonts.serif);

        // Theme
        root.classList.remove('light', 'dark', 'sepia');
        root.classList.add(readingSettings.theme);
    }

    function updateSettingsDisplay() {
        // Update inputs
        const fontSizeInput = document.getElementById('font-size-input');
        if (fontSizeInput) fontSizeInput.value = readingSettings.fontSize;

        const lineHeightInput = document.getElementById('line-height-input');
        if (lineHeightInput) lineHeightInput.value = readingSettings.lineHeight.toFixed(1);

        const paraSpacingInput = document.getElementById('para-spacing-input');
        if (paraSpacingInput) paraSpacingInput.value = readingSettings.paraSpacing;

        // Text width input
        const textWidthInput = document.getElementById('text-width-input');
        const textWidthValue = typeof readingSettings.textWidth === 'number'
            ? readingSettings.textWidth
            : parseInt(readingSettings.textWidth) || 800;
        if (textWidthInput) textWidthInput.value = textWidthValue;

        // Update text width preset buttons based on value
        const presetWidths = { 600: 'narrow', 800: 'medium', 1000: 'wide', 1400: 'full' };
        const matchingPreset = presetWidths[textWidthValue];
        document.querySelectorAll('[data-width]').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.width === matchingPreset);
        });

        // Update font family select
        const fontSelect = document.getElementById('font-family-select');
        if (fontSelect) fontSelect.value = readingSettings.fontFamily;

        // Update theme buttons
        document.querySelectorAll('[data-theme]').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.theme === readingSettings.theme);
        });

        // Update panel width limit inputs
        const panelMinWidthInput = document.getElementById('panel-min-width-input');
        const panelMaxWidthInput = document.getElementById('panel-max-width-input');
        if (panelMinWidthInput) panelMinWidthInput.value = readingSettings.panelMinWidth || 250;
        if (panelMaxWidthInput) panelMaxWidthInput.value = readingSettings.panelMaxWidth || 600;
    }

    function saveReadingSettings() {
        localStorage.setItem('franko-reading-settings', JSON.stringify(readingSettings));
    }

    function loadReadingSettings() {
        const saved = localStorage.getItem('franko-reading-settings');
        if (saved) {
            try {
                const parsed = JSON.parse(saved);
                readingSettings = { ...readingSettings, ...parsed };

                // Migrate old string textWidth to numeric
                if (typeof readingSettings.textWidth === 'string') {
                    const widthMap = { narrow: 600, medium: 800, wide: 1000, full: 1400 };
                    readingSettings.textWidth = widthMap[readingSettings.textWidth] || 800;
                }
            } catch (e) {
                console.error('Failed to load reading settings', e);
            }
        }
        applySettings();
        applyPanelWidthLimits();
        updateSettingsDisplay();
    }

    // Word count and progress tracking
    let totalBookWords = 0;
    let chapterWordCounts = [];

    function initWordCounts() {
        const container = document.getElementById('reader-container');
        if (!container) return;

        totalBookWords = parseInt(container.dataset.totalWords) || 0;
        try {
            chapterWordCounts = JSON.parse(container.dataset.chapterWords || '[]');
        } catch (e) {
            chapterWordCounts = [];
        }
    }

    // Chapter tracking based on scroll position
    function initChapterTracking() {
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

    // Update all progress indicators
    function updateProgressDisplay(stats) {
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

    // Format word count for display
    function formatWordCount(count) {
        if (count >= 1000000) {
            return (count / 1000000).toFixed(1) + 'M';
        } else if (count >= 1000) {
            return (count / 1000).toFixed(1) + 'k';
        }
        return count.toString();
    }

    // Update chapter indicator in footer (legacy, now handled by updateProgressDisplay)
    function updateChapterIndicator() {
        // Handled by updateProgressDisplay
    }

    // Update TOC highlight
    function updateTocHighlight() {
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

    // TOC navigation with smooth scroll
    function initTocNavigation() {
        const tocLinks = document.querySelectorAll('.toc a');
        tocLinks.forEach(link => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                const chapterIndex = parseInt(link.dataset.chapter);
                const chapter = document.getElementById(`chapter-${chapterIndex}`);
                if (chapter) {
                    chapter.scrollIntoView({ behavior: 'smooth', block: 'start' });
                    // Close sidebar on mobile
                    if (sidebar && sidebar.classList.contains('open')) {
                        toggleSidebar();
                    }
                }
            });
        });
    }

    // Event Listeners
    function setupEventListeners() {
        // Sidebar toggle
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
                sidebar.classList.remove('open');
            }
        });

        // Font size controls
        if (increaseFontBtn) {
            increaseFontBtn.addEventListener('click', () => changeFontSize(2));
        }
        if (decreaseFontBtn) {
            decreaseFontBtn.addEventListener('click', () => changeFontSize(-2));
        }

        // Theme toggle
        if (toggleThemeBtn) {
            toggleThemeBtn.addEventListener('click', toggleTheme);
        }

        // Fullscreen toggle
        if (toggleFullscreenBtn) {
            toggleFullscreenBtn.addEventListener('click', toggleFullscreen);
        }

        // Scroll progress
        if (content) {
            window.addEventListener('scroll', throttle(updateProgress, 50));
        }

        // Keyboard shortcuts
        document.addEventListener('keydown', handleKeyboard);

        // Save progress on leave
        window.addEventListener('beforeunload', saveProgress);

        // Smooth scroll for TOC links
        document.querySelectorAll('.toc a').forEach(link => {
            link.addEventListener('click', () => {
                if (sidebar) {
                    sidebar.classList.remove('open');
                }
            });
        });

        // Page navigation
        if (pagePrevBtn) {
            pagePrevBtn.addEventListener('click', prevPage);
        }
        if (pageNextBtn) {
            pageNextBtn.addEventListener('click', nextPage);
        }

        // Handle window resize for paged layout
        window.addEventListener('resize', debounce(() => {
            if (currentLayout !== 'scroll') {
                recalculatePages();
            }
        }, 250));
    }

    // Initialize layout controls
    function initLayoutControls() {
        const layoutBtns = document.querySelectorAll('.layout-btn');
        layoutBtns.forEach(btn => {
            btn.addEventListener('click', () => {
                const layout = btn.dataset.layout;
                setLayout(layout);

                // Update active state
                layoutBtns.forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
            });
        });
    }

    // Set layout mode
    function setLayout(layout) {
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
    }

    // Enter paged mode
    function enterPagedMode(layout) {
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

    // Exit paged mode
    function exitPagedMode() {
        if (!content || !readerContainer) return;

        // Remove dual page containers if they exist
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

    // Paginate for dual page mode (two side-by-side pages)
    function paginateDualMode() {
        if (!content) return;

        const containerHeight = readerContainer.clientHeight - 40;
        const containerWidth = (readerContainer.clientWidth - pageGap) / 2;

        // Hide original content
        content.style.display = 'none';
        readerContainer.classList.add('dual-active');

        // Remove existing dual container if any
        const existingDual = readerContainer.querySelector('.dual-page-container');
        if (existingDual) {
            existingDual.remove();
        }

        // Create dual page container
        const dualContainer = document.createElement('div');
        dualContainer.className = 'dual-page-container';

        // Create left and right page divs
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

    // Show a spread (two pages) in dual mode
    function showDualSpread(spreadIndex) {
        const dualContainer = readerContainer.querySelector('.dual-page-container');
        if (!dualContainer || !dualPages) return;

        const leftPage = dualContainer.querySelector('.page-left');
        const rightPage = dualContainer.querySelector('.page-right');

        const leftIndex = spreadIndex * 2;
        const rightIndex = spreadIndex * 2 + 1;

        leftPage.innerHTML = dualPages[leftIndex] ? dualPages[leftIndex].join('') : '';
        rightPage.innerHTML = dualPages[rightIndex] ? dualPages[rightIndex].join('') : '';
    }

    // Paginate content for single paged mode
    function paginateContent() {
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

        // Calculate approximate number of pages
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

    // Recalculate pages on resize
    function recalculatePages() {
        if (currentLayout === 'dual') {
            paginateDualMode();
        } else if (currentLayout !== 'scroll') {
            paginateContent();
        }
    }

    // Go to specific page
    function goToPage(pageNum) {
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

        // Save current page
        saveSetting('currentPage', currentPage);
    }

    // Navigate to previous page
    function prevPage() {
        if (currentPage > 0) {
            goToPage(currentPage - 1);
        }
    }

    // Navigate to next page
    function nextPage() {
        if (currentPage < totalPages - 1) {
            goToPage(currentPage + 1);
        }
    }

    // Update page indicator
    function updatePageIndicator() {
        if (pageIndicator) {
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
    }

    // Update page navigation buttons
    function updatePageButtons() {
        if (pagePrevBtn) {
            pagePrevBtn.disabled = currentPage === 0;
        }
        if (pageNextBtn) {
            pageNextBtn.disabled = currentPage >= totalPages - 1;
        }
    }

    // Toggle fullscreen
    function toggleFullscreen() {
        const readerLayout = document.querySelector('.reader-layout');
        if (!readerLayout) return;

        if (document.fullscreenElement) {
            document.exitFullscreen();
            readerLayout.classList.remove('fullscreen');
        } else {
            readerLayout.requestFullscreen().catch(() => { });
            readerLayout.classList.add('fullscreen');
        }
    }

    // Auto-scroll functionality
    function startAutoScroll(speed) {
        stopAutoScroll();
        autoScrollSpeed = speed;

        if (speed > 0 && currentLayout === 'scroll') {
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

    function stopAutoScroll() {
        if (autoScrollInterval) {
            clearInterval(autoScrollInterval);
            autoScrollInterval = null;
        }
        autoScrollSpeed = 0;
        showAutoScrollIndicator(false);
    }

    function showAutoScrollIndicator(show) {
        let indicator = document.querySelector('.auto-scroll-indicator');

        if (show) {
            if (!indicator) {
                indicator = document.createElement('div');
                indicator.className = 'auto-scroll-indicator';
                indicator.innerHTML = `
                    <span>Auto-scrolling</span>
                    <button onclick="window.frankoStopAutoScroll()">Stop</button>
                `;
                document.body.appendChild(indicator);
            }
            indicator.classList.add('active');
        } else if (indicator) {
            indicator.classList.remove('active');
        }
    }

    // Expose stop function globally
    window.frankoStopAutoScroll = stopAutoScroll;

    // Throttle function for performance
    function throttle(func, limit) {
        let inThrottle;
        return function (...args) {
            if (!inThrottle) {
                func.apply(this, args);
                inThrottle = true;
                setTimeout(() => inThrottle = false, limit);
            }
        };
    }

    // Initialize animations
    function initAnimations() {
        // Fade in book cards
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
        }
    }

    // Initialize search functionality
    function initSearch() {
        if (searchInput) {
            searchInput.addEventListener('input', debounce(handleSearch, 300));
        }
    }

    // Debounce function
    function debounce(func, wait) {
        let timeout;
        return function (...args) {
            clearTimeout(timeout);
            timeout = setTimeout(() => func.apply(this, args), wait);
        };
    }

    // Handle search input
    function handleSearch(e) {
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

    // Sidebar
    function toggleSidebar() {
        if (sidebar) {
            sidebar.classList.toggle('open');

            // Add overlay
            let overlay = document.querySelector('.sidebar-overlay');
            if (sidebar.classList.contains('open')) {
                if (!overlay) {
                    overlay = document.createElement('div');
                    overlay.className = 'sidebar-overlay';
                    overlay.style.cssText = `
                        position: fixed;
                        inset: 0;
                        background: rgba(0, 0, 0, 0.5);
                        z-index: 150;
                        opacity: 0;
                        transition: opacity 0.3s ease;
                    `;
                    document.body.appendChild(overlay);
                    setTimeout(() => overlay.style.opacity = '1', 10);
                    overlay.addEventListener('click', toggleSidebar);
                }
            } else if (overlay) {
                overlay.style.opacity = '0';
                setTimeout(() => overlay.remove(), 300);
            }
        }
    }

    // Font size
    function changeFontSize(delta) {
        fontSize = Math.max(minFontSize, Math.min(maxFontSize, fontSize + delta));
        document.documentElement.style.setProperty('--font-size', fontSize + 'px');

        // Show feedback
        showToast(`Font size: ${fontSize}px`);
        saveSettings();
    }

    // Theme
    function toggleTheme() {
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

        saveSettings();
    }

    // Toast notification
    function showToast(message) {
        let toast = document.querySelector('.toast');
        if (!toast) {
            toast = document.createElement('div');
            toast.className = 'toast';
            toast.style.cssText = `
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
            document.body.appendChild(toast);
        }

        toast.textContent = message;
        toast.style.opacity = '1';
        toast.style.transform = 'translateX(-50%) translateY(0)';

        setTimeout(() => {
            toast.style.opacity = '0';
            toast.style.transform = 'translateX(-50%) translateY(20px)';
        }, 2000);
    }

    // Progress
    function updateProgress() {
        if (!progressFill) return;

        const scrollTop = window.scrollY;
        const docHeight = document.documentElement.scrollHeight - window.innerHeight;
        const progress = docHeight > 0 ? (scrollTop / docHeight) * 100 : 0;

        progressFill.style.width = progress + '%';
    }

    // Keyboard shortcuts
    function handleKeyboard(e) {
        // Don't handle if in input
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') {
            return;
        }

        switch (e.key) {
            case 'ArrowLeft':
            case 'h':
                if (currentLayout !== 'scroll') {
                    e.preventDefault();
                    prevPage();
                } else {
                    navigatePrev();
                }
                break;
            case 'ArrowRight':
            case 'l':
                if (currentLayout !== 'scroll') {
                    e.preventDefault();
                    nextPage();
                } else {
                    navigateNext();
                }
                break;
            case 'j':
                if (currentLayout === 'scroll') {
                    window.scrollBy({ top: 100, behavior: 'smooth' });
                }
                break;
            case 'k':
                if (currentLayout === 'scroll') {
                    window.scrollBy({ top: -100, behavior: 'smooth' });
                }
                break;
            case ' ':
                e.preventDefault();
                if (currentLayout !== 'scroll') {
                    if (e.shiftKey) {
                        prevPage();
                    } else {
                        nextPage();
                    }
                } else {
                    if (!e.shiftKey) {
                        window.scrollBy({ top: window.innerHeight - 100, behavior: 'smooth' });
                    } else {
                        window.scrollBy({ top: -(window.innerHeight - 100), behavior: 'smooth' });
                    }
                }
                break;
            case 'g':
                if (currentLayout !== 'scroll') {
                    if (e.shiftKey) {
                        goToPage(totalPages - 1);
                    } else {
                        goToPage(0);
                    }
                } else if (e.shiftKey) {
                    window.scrollTo({ top: document.documentElement.scrollHeight, behavior: 'smooth' });
                } else {
                    window.scrollTo({ top: 0, behavior: 'smooth' });
                }
                break;
            case 't':
                toggleSidebar();
                break;
            case '=':
            case '+':
                changeFontSize(2);
                break;
            case '-':
                changeFontSize(-2);
                break;
            case 'd':
                toggleTheme();
                break;
            case 'f':
                toggleFullscreen();
                break;
            case '1':
                setLayout('scroll');
                break;
            case '2':
                setLayout('paged');
                break;
            case '3':
                setLayout('dual');
                break;
            case '/':
                e.preventDefault();
                openSearch();
                break;
            case 's':
                toggleSettingsPanel();
                break;
            case 'Escape':
                if (sidebar && sidebar.classList.contains('open')) {
                    toggleSidebar();
                }
                const settingsPanel = document.getElementById('settings-panel');
                if (settingsPanel && settingsPanel.classList.contains('open')) {
                    toggleSettingsPanel();
                }
                stopAutoScroll();
                break;
        }
    }

    // Navigation
    function navigatePrev() {
        const prevLink = document.querySelector('.nav-prev');
        if (prevLink) {
            prevLink.click();
        }
    }

    function navigateNext() {
        const nextLink = document.querySelector('.nav-next');
        if (nextLink) {
            nextLink.click();
        }
    }

    // Search
    function openSearch() {
        const searchInput = document.getElementById('search');
        if (searchInput) {
            searchInput.focus();
            searchInput.select();
        } else {
            showToast('Press / to search');
        }
    }

    // Settings persistence
    function saveSettings() {
        const settings = {
            fontSize: fontSize,
            theme: document.documentElement.classList.contains('light') ? 'light' : 'dark'
        };
        localStorage.setItem('franko-settings', JSON.stringify(settings));
    }

    function loadSettings() {
        const saved = localStorage.getItem('franko-settings');
        if (saved) {
            try {
                const settings = JSON.parse(saved);

                if (settings.fontSize) {
                    fontSize = settings.fontSize;
                    document.documentElement.style.setProperty('--font-size', fontSize + 'px');
                }

                if (settings.theme) {
                    document.documentElement.classList.remove('dark', 'light');
                    document.documentElement.classList.add(settings.theme);
                }

                // Load font family
                if (settings.fontFamily) {
                    const families = {
                        system: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
                        serif: 'Georgia, Cambria, "Times New Roman", Times, serif',
                        mono: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
                        inter: '"Inter", -apple-system, sans-serif',
                        merriweather: '"Merriweather", Georgia, serif',
                        literata: '"Literata", Georgia, serif',
                        jetbrains: '"JetBrains Mono", ui-monospace, monospace',
                        fira: '"Fira Code", ui-monospace, monospace',
                        opendyslexic: '"OpenDyslexic", sans-serif'
                    };
                    if (families[settings.fontFamily]) {
                        document.documentElement.style.setProperty('--font-family-reading', families[settings.fontFamily]);
                    }
                }

                // Load line height
                if (settings.lineHeight) {
                    document.documentElement.style.setProperty('--line-height', settings.lineHeight);
                }

                // Load text width
                if (settings.textWidth) {
                    const widths = { narrow: '600px', medium: '800px', wide: '1000px', full: '100%' };
                    if (widths[settings.textWidth]) {
                        document.documentElement.style.setProperty('--text-width', widths[settings.textWidth]);
                    }
                }

                // Load accent color
                if (settings.accentColor) {
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
                    if (colors[settings.accentColor]) {
                        document.documentElement.style.setProperty('--accent-primary', colors[settings.accentColor].primary);
                        document.documentElement.style.setProperty('--accent-secondary', colors[settings.accentColor].secondary);
                    }
                }

                // Load layout mode
                if (settings.layoutMode) {
                    currentLayout = settings.layoutMode;
                    // Apply after DOM is ready
                    setTimeout(() => {
                        setLayout(settings.layoutMode);
                        // Update active button
                        document.querySelectorAll('.layout-btn').forEach(btn => {
                            btn.classList.toggle('active', btn.dataset.layout === settings.layoutMode);
                        });
                    }, 100);
                }

                // Load page gap
                if (settings.pageGap) {
                    pageGap = settings.pageGap;
                }

                // Load page animation
                if (settings.pageAnimation) {
                    pageAnimation = settings.pageAnimation;
                }

                // Load current page for paged mode
                if (settings.currentPage) {
                    setTimeout(() => {
                        if (currentLayout !== 'scroll') {
                            goToPage(settings.currentPage);
                        }
                    }, 200);
                }
            } catch (e) {
                console.error('Failed to load settings', e);
            }
        }
    }

    // Progress persistence
    function saveProgress() {
        const bookId = getBookId();
        if (!bookId) return;

        const scrollPosition = window.scrollY;

        // Calculate progress as a percentage (0.0 - 1.0)
        const docHeight = document.documentElement.scrollHeight - window.innerHeight;
        const progressPercent = docHeight > 0 ? Math.min(1.0, scrollPosition / docHeight) : 0;

        const progress = {
            scroll: scrollPosition,
            chapter: currentChapter,
            timestamp: Date.now()
        };

        localStorage.setItem(`franko-progress-${bookId}`, JSON.stringify(progress));

        // Also try to save to API
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

    function loadProgress() {
        const bookId = getBookId();
        if (!bookId) return;

        // Try to load from server first, fall back to localStorage
        fetch(`/api/books/${bookId}/progress`)
            .then(response => response.json())
            .then(data => {
                if (data.success && data.data) {
                    const serverProgress = data.data;
                    // Restore scroll position
                    if (serverProgress.scroll_offset) {
                        setTimeout(() => {
                            window.scrollTo(0, serverProgress.scroll_offset);
                        }, 100);
                    }
                } else {
                    // Fall back to localStorage
                    loadProgressFromLocalStorage();
                }
            })
            .catch(() => {
                // Fall back to localStorage on error
                loadProgressFromLocalStorage();
            });
    }

    function loadProgressFromLocalStorage() {
        const bookId = getBookId();
        if (!bookId) return;

        const saved = localStorage.getItem(`franko-progress-${bookId}`);
        if (saved) {
            try {
                const progress = JSON.parse(saved);
                if (progress.scroll) {
                    window.scrollTo(0, progress.scroll);
                }
            } catch (e) {
                console.error('Failed to load progress', e);
            }
        }
    }

    function getBookId() {
        const match = window.location.pathname.match(/\/read\/([^\/]+)/);
        return match ? match[1] : null;
    }

    function getCurrentChapter() {
        return currentChapter;
    }

    // Auto-save progress periodically
    setInterval(() => {
        if (getBookId()) {
            saveProgress();
        }
    }, 30000); // Every 30 seconds

    // ========== Settings Page Functions ==========

    function initSettingsPage() {
        if (!document.querySelector('.settings-page')) return;

        // Theme buttons
        document.querySelectorAll('.theme-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const theme = btn.dataset.theme;
                setTheme(theme);

                // Update active state
                document.querySelectorAll('.theme-btn').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
            });
        });

        // Color picker
        document.querySelectorAll('.color-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const color = btn.dataset.color;
                setAccentColor(color);

                document.querySelectorAll('.color-btn').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
            });
        });

        // Font family select
        const fontFamilySelect = document.getElementById('font-family');
        if (fontFamilySelect) {
            fontFamilySelect.addEventListener('change', () => {
                setFontFamily(fontFamilySelect.value);
            });
        }

        // Font size range
        const fontSizeRange = document.getElementById('font-size-range');
        if (fontSizeRange) {
            fontSizeRange.addEventListener('input', () => {
                const size = fontSizeRange.value;
                setFontSizeValue(parseInt(size));
            });
        }

        // Line height select
        const lineHeightSelect = document.getElementById('line-height');
        if (lineHeightSelect) {
            lineHeightSelect.addEventListener('change', () => {
                setLineHeight(lineHeightSelect.value);
            });
        }

        // Text width select
        const textWidthSelect = document.getElementById('text-width');
        if (textWidthSelect) {
            textWidthSelect.addEventListener('change', () => {
                setTextWidth(textWidthSelect.value);
            });
        }

        // Toggle switches
        document.querySelectorAll('.toggle input').forEach(toggle => {
            toggle.addEventListener('change', () => {
                const setting = toggle.id;
                const value = toggle.checked;
                saveSetting(setting, value);
                showToast(`${formatSettingName(setting)}: ${value ? 'On' : 'Off'}`);
            });
        });

        // Save button
        const saveBtn = document.getElementById('save-settings');
        if (saveBtn) {
            saveBtn.addEventListener('click', () => {
                saveAllSettings();
                showToast('Settings saved!');
            });
        }

        // Reset button
        const resetBtn = document.getElementById('reset-settings');
        if (resetBtn) {
            resetBtn.addEventListener('click', () => {
                if (confirm('Are you sure you want to reset all settings to defaults?')) {
                    resetSettings();
                    showToast('Settings reset to defaults');
                    location.reload();
                }
            });
        }

        // Export button
        const exportBtn = document.getElementById('export-settings');
        if (exportBtn) {
            exportBtn.addEventListener('click', exportSettings);
        }

        // Import button
        const importBtn = document.getElementById('import-settings');
        if (importBtn) {
            importBtn.addEventListener('click', () => {
                const input = document.createElement('input');
                input.type = 'file';
                input.accept = '.json';
                input.onchange = (e) => importSettings(e.target.files[0]);
                input.click();
            });
        }

        // Load current settings
        loadSettingsPage();
    }

    function setTheme(theme) {
        document.documentElement.classList.remove('dark', 'light', 'auto');
        if (theme !== 'auto') {
            document.documentElement.classList.add(theme);
        }
        saveSetting('theme', theme);
        showToast(`Theme: ${theme.charAt(0).toUpperCase() + theme.slice(1)}`);
    }

    function setAccentColor(color) {
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
            document.documentElement.style.setProperty('--accent-primary', colors[color].primary);
            document.documentElement.style.setProperty('--accent-secondary', colors[color].secondary);
            saveSetting('accentColor', color);
            showToast(`Accent: ${color.charAt(0).toUpperCase() + color.slice(1)}`);
        }
    }

    function setFontFamily(family) {
        const families = {
            system: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
            serif: 'Georgia, Cambria, "Times New Roman", Times, serif',
            mono: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
            inter: '"Inter", -apple-system, sans-serif',
            merriweather: '"Merriweather", Georgia, serif',
            literata: '"Literata", Georgia, serif',
            jetbrains: '"JetBrains Mono", ui-monospace, monospace',
            fira: '"Fira Code", ui-monospace, monospace',
            opendyslexic: '"OpenDyslexic", sans-serif'
        };

        const familyNames = {
            system: 'System',
            serif: 'Serif',
            mono: 'Monospace',
            inter: 'Inter',
            merriweather: 'Merriweather',
            literata: 'Literata',
            jetbrains: 'JetBrains Mono',
            fira: 'Fira Code',
            opendyslexic: 'OpenDyslexic'
        };

        if (families[family]) {
            document.documentElement.style.setProperty('--font-family-reading', families[family]);
            saveSetting('fontFamily', family);
            showToast(`Font: ${familyNames[family] || family}`);
        }
    }

    function setFontSizeValue(size) {
        document.documentElement.style.setProperty('--font-size', size + 'px');
        fontSize = size;
        saveSetting('fontSize', size);

        const display = document.querySelector('.font-size-display');
        if (display) display.textContent = size + 'px';
    }

    function setLineHeight(height) {
        document.documentElement.style.setProperty('--line-height', height);
        saveSetting('lineHeight', height);
    }

    function setTextWidth(width) {
        const widths = {
            narrow: '600px',
            medium: '800px',
            wide: '1000px',
            full: '100%'
        };

        if (widths[width]) {
            document.documentElement.style.setProperty('--text-width', widths[width]);
            saveSetting('textWidth', width);
        }
    }

    function saveSetting(key, value) {
        const settings = JSON.parse(localStorage.getItem('franko-settings') || '{}');
        settings[key] = value;
        localStorage.setItem('franko-settings', JSON.stringify(settings));
    }

    function saveAllSettings() {
        // Gather all settings from the page
        const settings = {};

        // Theme
        const activeTheme = document.querySelector('.theme-btn.active');
        if (activeTheme) settings.theme = activeTheme.dataset.theme;

        // Color
        const activeColor = document.querySelector('.color-btn.active');
        if (activeColor) settings.accentColor = activeColor.dataset.color;

        // Font family
        const fontFamily = document.getElementById('font-family');
        if (fontFamily) settings.fontFamily = fontFamily.value;

        // Font size
        const fontSizeRange = document.getElementById('font-size-range');
        if (fontSizeRange) settings.fontSize = parseInt(fontSizeRange.value);

        // Line height
        const lineHeight = document.getElementById('line-height');
        if (lineHeight) settings.lineHeight = lineHeight.value;

        // Text width
        const textWidth = document.getElementById('text-width');
        if (textWidth) settings.textWidth = textWidth.value;

        // Toggles
        document.querySelectorAll('.toggle input').forEach(toggle => {
            settings[toggle.id] = toggle.checked;
        });

        // Sync URL
        const syncUrl = document.getElementById('sync-url');
        if (syncUrl) settings.syncUrl = syncUrl.value;

        // Sync interval
        const syncInterval = document.getElementById('sync-interval');
        if (syncInterval) settings.syncInterval = syncInterval.value;

        localStorage.setItem('franko-settings', JSON.stringify(settings));
    }

    function loadSettingsPage() {
        const saved = localStorage.getItem('franko-settings');
        if (!saved) return;

        try {
            const settings = JSON.parse(saved);

            // Theme
            if (settings.theme) {
                document.querySelectorAll('.theme-btn').forEach(btn => {
                    btn.classList.toggle('active', btn.dataset.theme === settings.theme);
                });
            }

            // Color
            if (settings.accentColor) {
                document.querySelectorAll('.color-btn').forEach(btn => {
                    btn.classList.toggle('active', btn.dataset.color === settings.accentColor);
                });
                setAccentColor(settings.accentColor);
            }

            // Font family
            if (settings.fontFamily) {
                const fontFamily = document.getElementById('font-family');
                if (fontFamily) fontFamily.value = settings.fontFamily;
            }

            // Font size
            if (settings.fontSize) {
                const fontSizeRange = document.getElementById('font-size-range');
                if (fontSizeRange) fontSizeRange.value = settings.fontSize;
                const display = document.querySelector('.font-size-display');
                if (display) display.textContent = settings.fontSize + 'px';
            }

            // Line height
            if (settings.lineHeight) {
                const lineHeight = document.getElementById('line-height');
                if (lineHeight) lineHeight.value = settings.lineHeight;
            }

            // Text width
            if (settings.textWidth) {
                const textWidth = document.getElementById('text-width');
                if (textWidth) textWidth.value = settings.textWidth;
            }

            // Toggles
            document.querySelectorAll('.toggle input').forEach(toggle => {
                if (settings[toggle.id] !== undefined) {
                    toggle.checked = settings[toggle.id];
                }
            });

            // Sync URL
            if (settings.syncUrl) {
                const syncUrl = document.getElementById('sync-url');
                if (syncUrl) syncUrl.value = settings.syncUrl;
            }

            // Sync interval
            if (settings.syncInterval) {
                const syncInterval = document.getElementById('sync-interval');
                if (syncInterval) syncInterval.value = settings.syncInterval;
            }
        } catch (e) {
            console.error('Failed to load settings', e);
        }
    }

    function resetSettings() {
        localStorage.removeItem('franko-settings');
        document.documentElement.removeAttribute('style');
        document.documentElement.classList.remove('light');
        document.documentElement.classList.add('dark');
    }

    function exportSettings() {
        const settings = localStorage.getItem('franko-settings') || '{}';
        const blob = new Blob([settings], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'franko-settings.json';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
        showToast('Settings exported');
    }

    function importSettings(file) {
        const reader = new FileReader();
        reader.onload = (e) => {
            try {
                const settings = JSON.parse(e.target.result);
                localStorage.setItem('franko-settings', JSON.stringify(settings));
                showToast('Settings imported');
                location.reload();
            } catch (err) {
                showToast('Invalid settings file');
            }
        };
        reader.readAsText(file);
    }

    function formatSettingName(name) {
        return name
            .replace(/-/g, ' ')
            .replace(/([A-Z])/g, ' $1')
            .replace(/^./, str => str.toUpperCase())
            .trim();
    }

    // Initialize when DOM is ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            init();
            initSettingsPage();
        });
    } else {
        init();
        initSettingsPage();
    }

    // Load reading progress after a short delay
    setTimeout(loadProgress, 100);
})();
