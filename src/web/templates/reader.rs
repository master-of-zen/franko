//! Reader page templates

use crate::config::Config;
use crate::formats::Book;
use super::base::base;
use super::helpers::{chapter_to_html, escape_html, format_word_count};

/// Generate the reader page for books
pub fn reader(config: &Config, book: &Book, _chapter_index: usize) -> String {
    // Calculate word counts
    let chapter_word_counts: Vec<usize> = book
        .content
        .chapters
        .iter()
        .map(|ch| ch.word_count())
        .collect();
    let total_word_count: usize = chapter_word_counts.iter().sum();

    // Build continuous content from all chapters with markers
    let mut book_content = String::new();
    let mut cumulative_words = 0usize;
    for (i, chapter) in book.content.chapters.iter().enumerate() {
        let chapter_words = chapter_word_counts[i];
        // Add chapter marker/anchor with word count data
        book_content.push_str(&format!(
            r#"<section class="chapter" id="chapter-{}" data-chapter="{}" data-words="{}" data-cumulative-words="{}">"#,
            i, i, chapter_words, cumulative_words
        ));
        book_content.push_str(&chapter_to_html(chapter));
        book_content.push_str("</section>\n");
        cumulative_words += chapter_words;
    }

    // Build chapter word counts JSON for JavaScript
    let chapter_words_json: String = chapter_word_counts
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join(",");

    // Build TOC with anchor links and word counts
    let toc_items: String = book
        .content
        .chapters
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let words = chapter_word_counts[i];
            format!(
                "<li><a href=\"#chapter-{}\" data-chapter=\"{}\" data-words=\"{}\">{} <span class=\"toc-word-count\">({} words)</span></a></li>",
                i,
                i,
                words,
                escape_html(&ch.display_title()),
                format_word_count(words),
            )
        })
        .collect();

    let content = format!(
        r##"
        <div class="reader-layout">
            <aside class="reader-sidebar" id="sidebar">
                <div class="sidebar-header">
                    <h3>Contents</h3>
                    <button id="close-sidebar">×</button>
                </div>
                <nav class="toc">
                    <ul>
                        {toc_items}
                    </ul>
                </nav>
            </aside>

            {settings_panel}

            <main class="reader-main">
                <header class="reader-header">
                    <a href="/library" class="btn-icon" data-tooltip="Back to Library" title="Back to Library">←</a>
                    <button id="toggle-sidebar" class="btn-icon" data-tooltip="Table of Contents">☰</button>
                    <h1>{title}</h1>
                    <div class="reader-controls">
                        <button id="toggle-settings" class="btn-icon" data-tooltip="Reading Settings" title="Settings">⚙</button>
                        <button id="toggle-theme" class="btn-icon" data-tooltip="Toggle Theme">🌓</button>
                        <button id="toggle-fullscreen" class="btn-icon" data-tooltip="Fullscreen">⛶</button>
                    </div>
                </header>
                <div class="reader-container" id="reader-container" data-layout="scroll" data-book-id="{book_id}"
                     data-total-words="{total_words}" data-chapter-words="[{chapter_words}]">
                    <article class="reader-content" id="content">
                        {book_content}
                    </article>
                </div>
                <div class="page-controls" id="page-controls" style="display: none;">
                    <button id="page-prev" class="btn-icon page-nav">←</button>
                    <span class="page-indicator" id="page-indicator">Page 1</span>
                    <button id="page-next" class="btn-icon page-nav">→</button>
                </div>
                <footer class="reader-footer" id="reader-footer">
                    <div class="progress-stats">
                        <span class="progress-stat" id="progress-percent">0%</span>
                        <span class="progress-stat" id="progress-words-read">0 / {total_words_formatted} words</span>
                        <span class="progress-stat" id="progress-chapter">Chapter 1</span>
                        <span class="progress-stat" id="progress-chapter-words">0 / 0 words in chapter</span>
                    </div>
                </footer>
            </main>
        </div>
        <div class="reader-progress" id="progress">
            <div class="progress-fill" id="progress-fill"></div>
        </div>
    "##,
        toc_items = toc_items,
        title = escape_html(&book.metadata.title),
        book_content = book_content,
        book_id = book
            .source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown"),
        total_words = total_word_count,
        total_words_formatted = format_word_count(total_word_count),
        chapter_words = chapter_words_json,
        settings_panel = settings_panel(),
    );

    base(&book.metadata.title, &content, config)
}

/// PDF viewer using PDF.js
pub fn pdf_reader(config: &Config, book_id: &str, title: &str) -> String {
    let content = format!(
        r#"
        <div class="pdf-viewer-layout">
            <header class="pdf-header">
                <div class="pdf-header-left">
                    <a href="/library" class="btn-icon" title="Back to Library">←</a>
                    <h1>{title}</h1>
                </div>
                <div class="pdf-controls">
                    <button id="pdf-prev" class="btn-icon" title="Previous Page">◀</button>
                    <span class="pdf-page-info">
                        <input type="number" id="pdf-page-input" min="1" value="1"> / <span id="pdf-page-count">-</span>
                    </span>
                    <button id="pdf-next" class="btn-icon" title="Next Page">▶</button>
                    <span class="pdf-separator"></span>
                    <button id="pdf-zoom-out" class="btn-icon" title="Zoom Out">−</button>
                    <span id="pdf-zoom-level">100%</span>
                    <button id="pdf-zoom-in" class="btn-icon" title="Zoom In">+</button>
                    <button id="pdf-fit-width" class="btn-icon" title="Fit Width">↔</button>
                    <button id="pdf-fit-page" class="btn-icon" title="Fit Page">⛶</button>
                    <span class="pdf-separator"></span>
                    <button id="pdf-fullscreen" class="btn-icon" title="Fullscreen">⛶</button>
                    <a href="/api/books/{book_id}/pdf" download class="btn-icon" title="Download PDF">⬇</a>
                </div>
            </header>
            <div class="pdf-container" id="pdf-container">
                <div class="pdf-loading" id="pdf-loading">
                    <div class="spinner"></div>
                    <p>Loading PDF...</p>
                </div>
                <div class="pdf-canvas-container" id="pdf-canvas-container">
                    <canvas id="pdf-canvas"></canvas>
                </div>
            </div>
        </div>

        <script src="https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.min.js"></script>
        {pdf_script}
    "#,
        title = escape_html(title),
        book_id = book_id,
        pdf_script = pdf_viewer_script(book_id),
    );

    base(title, &content, config)
}

/// Generate book info page
pub fn book_info(config: &Config, book: &Book) -> String {
    let subjects: String = book
        .metadata
        .subjects
        .iter()
        .map(|s| format!(r#"<span class="tag">{}</span>"#, escape_html(s)))
        .collect::<Vec<_>>()
        .join(" ");

    let content = format!(
        r#"
        <header class="site-header">
            <h1>📖 Franko</h1>
            <nav>
                <a href="/">Home</a>
                <a href="/library">Library</a>
                <a href="/settings">Settings</a>
            </nav>
        </header>
        <main class="book-info-page">
            <div class="book-detail">
                <div class="book-cover-large">
                    <div class="placeholder-cover">📚</div>
                </div>
                <div class="book-meta">
                    <h1>{title}</h1>
                    <p class="author">by {author}</p>
                    {description}
                    <dl class="meta-list">
                        {publisher}
                        {published}
                        {language}
                        {word_count}
                        {reading_time}
                    </dl>
                    <div class="tags">
                        {subjects}
                    </div>
                    <div class="actions">
                        <a href="/read/{book_id}" class="btn primary">Start Reading</a>
                    </div>
                </div>
            </div>
        </main>
    "#,
        title = escape_html(&book.metadata.title),
        author = escape_html(&book.metadata.authors_string()),
        description = book
            .metadata
            .description
            .as_ref()
            .map(|d| format!(r#"<p class="description">{}</p>"#, escape_html(d)))
            .unwrap_or_default(),
        publisher = book
            .metadata
            .publisher
            .as_ref()
            .map(|p| format!(r#"<dt>Publisher</dt><dd>{}</dd>"#, escape_html(p)))
            .unwrap_or_default(),
        published = book
            .metadata
            .published
            .as_ref()
            .map(|p| format!(r#"<dt>Published</dt><dd>{}</dd>"#, escape_html(p)))
            .unwrap_or_default(),
        language = book
            .metadata
            .language
            .as_ref()
            .map(|l| format!(r#"<dt>Language</dt><dd>{}</dd>"#, escape_html(l)))
            .unwrap_or_default(),
        word_count = book
            .metadata
            .word_count
            .map(|w| format!(r#"<dt>Word Count</dt><dd>{}</dd>"#, w))
            .unwrap_or_default(),
        reading_time = book
            .metadata
            .reading_time
            .map(|t| format!(r#"<dt>Reading Time</dt><dd>~{} min</dd>"#, t))
            .unwrap_or_default(),
        subjects = subjects,
        book_id = "main", // TODO: Use actual book ID
    );

    base(&book.metadata.title, &content, config)
}

/// Generate error page
pub fn error(message: &str) -> String {
    let config = Config::default();
    let content = format!(
        r#"
        <header class="site-header">
            <h1>📖 Franko</h1>
            <nav>
                <a href="/">Home</a>
                <a href="/library">Library</a>
                <a href="/settings">Settings</a>
            </nav>
        </header>
        <main class="error-page">
            <h1>Error</h1>
            <p>{message}</p>
            <a href="/" class="btn">Go Home</a>
        </main>
    "#,
        message = escape_html(message),
    );

    base("Error", &content, &config)
}

/// Settings panel HTML for reader
fn settings_panel() -> &'static str {
    r##"
    <!-- Settings Panel -->
    <aside class="reader-settings-panel" id="settings-panel">
        <div class="settings-panel-resize-handle" id="settings-resize-handle"></div>
        <div class="settings-panel-header">
            <h3>Reading Settings</h3>
            <button id="close-settings" class="btn-icon">×</button>
        </div>
        <div class="settings-panel-content">
            <!-- Font Size -->
            <div class="setting-group">
                <div class="setting-label-row">
                    <label>Font Size</label>
                    <div class="setting-control-inline">
                        <button id="font-decrease" class="btn-icon">−</button>
                        <input type="number" id="font-size-input" class="setting-input" value="18">
                        <span class="setting-unit">px</span>
                        <button id="font-increase" class="btn-icon">+</button>
                    </div>
                </div>
            </div>

            <!-- Line Height -->
            <div class="setting-group">
                <div class="setting-label-row">
                    <label>Line Height</label>
                    <div class="setting-control-inline">
                        <button id="line-height-decrease" class="btn-icon">−</button>
                        <input type="number" id="line-height-input" class="setting-input" step="0.1" value="1.8">
                        <button id="line-height-increase" class="btn-icon">+</button>
                    </div>
                </div>
            </div>

            <!-- Text Width -->
            <div class="setting-group">
                <div class="setting-label-row">
                    <label>Text Width</label>
                    <div class="setting-control-inline">
                        <button id="text-width-decrease" class="btn-icon">−</button>
                        <input type="number" id="text-width-input" class="setting-input" step="50" value="800">
                        <span class="setting-unit">px</span>
                        <button id="text-width-increase" class="btn-icon">+</button>
                    </div>
                </div>
                <div class="setting-buttons">
                    <button class="setting-btn" data-width="narrow" data-width-value="600">Narrow</button>
                    <button class="setting-btn active" data-width="medium" data-width-value="800">Medium</button>
                    <button class="setting-btn" data-width="wide" data-width-value="1000">Wide</button>
                    <button class="setting-btn" data-width="full" data-width-value="1400">Full</button>
                </div>
            </div>

            <!-- Paragraph Spacing -->
            <div class="setting-group">
                <div class="setting-label-row">
                    <label>Paragraph Spacing</label>
                    <div class="setting-control-inline">
                        <button id="para-spacing-decrease" class="btn-icon">−</button>
                        <input type="number" id="para-spacing-input" class="setting-input" step="0.25" value="1">
                        <span class="setting-unit">em</span>
                        <button id="para-spacing-increase" class="btn-icon">+</button>
                    </div>
                </div>
            </div>

            <!-- Font Family -->
            <div class="setting-group">
                <label>Font Family</label>
                <select id="font-family-select">
                    <option value="serif">Serif (Georgia)</option>
                    <option value="sans">Sans-serif (System)</option>
                    <option value="mono">Monospace</option>
                </select>
            </div>

            <!-- Theme -->
            <div class="setting-group">
                <label>Theme</label>
                <select id="theme-select">
                    <optgroup label="Light Themes">
                        <option value="light">Light (Default)</option>
                        <option value="paper">Paper</option>
                        <option value="sepia">Sepia</option>
                        <option value="solarized-light">Solarized Light</option>
                        <option value="gruvbox-light">Gruvbox Light</option>
                        <option value="catppuccin-latte">Catppuccin Latte</option>
                        <option value="github-light">GitHub Light</option>
                        <option value="rose-pine-dawn">Rosé Pine Dawn</option>
                        <option value="everforest-light">Everforest Light</option>
                        <option value="atom-one-light">Atom One Light</option>
                        <option value="ayu-light">Ayu Light</option>
                        <option value="night-owl-light">Night Owl Light</option>
                        <option value="flexoki-light">Flexoki Light</option>
                    </optgroup>
                    <optgroup label="Dark Themes">
                        <option value="dark" selected>Dark (Default)</option>
                        <option value="tokyo-night">Tokyo Night</option>
                        <option value="dracula">Dracula</option>
                        <option value="nord">Nord</option>
                        <option value="one-dark">One Dark</option>
                        <option value="atom-one-dark">Atom One Dark</option>
                        <option value="monokai">Monokai</option>
                        <option value="solarized-dark">Solarized Dark</option>
                        <option value="gruvbox-dark">Gruvbox Dark</option>
                        <option value="catppuccin-mocha">Catppuccin Mocha</option>
                        <option value="catppuccin-macchiato">Catppuccin Macchiato</option>
                        <option value="catppuccin-frappe">Catppuccin Frappé</option>
                        <option value="github-dark">GitHub Dark</option>
                        <option value="rose-pine">Rosé Pine</option>
                        <option value="rose-pine-moon">Rosé Pine Moon</option>
                        <option value="everforest-dark">Everforest Dark</option>
                        <option value="kanagawa">Kanagawa</option>
                        <option value="material-dark">Material Dark</option>
                        <option value="night-owl">Night Owl</option>
                        <option value="palenight">Palenight</option>
                        <option value="shades-of-purple">Shades of Purple</option>
                        <option value="ayu-dark">Ayu Dark</option>
                        <option value="ayu-mirage">Ayu Mirage</option>
                        <option value="horizon">Horizon</option>
                        <option value="cobalt2">Cobalt2</option>
                        <option value="synthwave84">Synthwave '84</option>
                        <option value="iceberg">Iceberg</option>
                        <option value="zenburn">Zenburn</option>
                        <option value="poimandres">Poimandres</option>
                        <option value="vesper">Vesper</option>
                        <option value="flexoki-dark">Flexoki Dark</option>
                        <option value="oxocarbon-dark">Oxocarbon Dark</option>
                        <option value="amoled">AMOLED Black</option>
                    </optgroup>
                    <optgroup label="E-Reader">
                        <option value="kindle">Kindle</option>
                        <option value="kobo">Kobo</option>
                    </optgroup>
                    <optgroup label="Night Reading">
                        <option value="midnight-blue">Midnight Blue</option>
                        <option value="warm-night">Warm Night</option>
                    </optgroup>
                    <optgroup label="Accessibility">
                        <option value="high-contrast">High Contrast</option>
                    </optgroup>
                    <optgroup label="Custom">
                        <option value="custom">Custom...</option>
                    </optgroup>
                </select>
            </div>

            <!-- Custom Theme Colors (shown when custom theme is selected) -->
            <div class="setting-group custom-colors-group" id="custom-colors-group" style="display: none;">
                <label>Custom Colors</label>
                <div class="color-settings">
                    <div class="color-setting-row">
                        <span class="color-label">Background</span>
                        <input type="color" id="custom-color-background" class="color-picker" value="#1a1a2e">
                        <input type="text" id="custom-color-background-text" class="color-text-input" value="#1a1a2e" maxlength="7">
                    </div>
                    <div class="color-setting-row">
                        <span class="color-label">Text</span>
                        <input type="color" id="custom-color-text" class="color-picker" value="#eaeaea">
                        <input type="text" id="custom-color-text-text" class="color-text-input" value="#eaeaea" maxlength="7">
                    </div>
                    <div class="color-setting-row">
                        <span class="color-label">Accent</span>
                        <input type="color" id="custom-color-accent" class="color-picker" value="#6366f1">
                        <input type="text" id="custom-color-accent-text" class="color-text-input" value="#6366f1" maxlength="7">
                    </div>
                    <div class="color-setting-row">
                        <span class="color-label">Links</span>
                        <input type="color" id="custom-color-link" class="color-picker" value="#818cf8">
                        <input type="text" id="custom-color-link-text" class="color-text-input" value="#818cf8" maxlength="7">
                    </div>
                </div>
            </div>

            <!-- Panel Width Limits -->
            <div class="setting-group">
                <label>Panel Size Limits</label>
                <div class="setting-row-stack">
                    <div class="setting-pair">
                        <span class="setting-pair-label">Min</span>
                        <button id="panel-min-width-decrease" class="btn-icon btn-icon-sm">−</button>
                        <input type="number" id="panel-min-width-input" class="setting-input" step="50" value="250">
                        <span class="setting-unit">px</span>
                        <button id="panel-min-width-increase" class="btn-icon btn-icon-sm">+</button>
                    </div>
                    <div class="setting-pair">
                        <span class="setting-pair-label">Max</span>
                        <button id="panel-max-width-decrease" class="btn-icon btn-icon-sm">−</button>
                        <input type="number" id="panel-max-width-input" class="setting-input" step="50" value="600">
                        <span class="setting-unit">px</span>
                        <button id="panel-max-width-increase" class="btn-icon btn-icon-sm">+</button>
                    </div>
                </div>
            </div>
        </div>
    </aside>
    "##
}

/// PDF viewer JavaScript
fn pdf_viewer_script(book_id: &str) -> String {
    format!(
        r#"
        <script>
        (function() {{
            'use strict';

            // PDF.js worker
            pdfjsLib.GlobalWorkerOptions.workerSrc = 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.worker.min.js';

            // State
            let pdfDoc = null;
            let pageNum = 1;
            let pageRendering = false;
            let pageNumPending = null;
            let scale = 1.0;
            let fitMode = 'width'; // 'width', 'page', 'custom'

            // Elements
            const canvas = document.getElementById('pdf-canvas');
            const ctx = canvas.getContext('2d');
            const container = document.getElementById('pdf-container');
            const canvasContainer = document.getElementById('pdf-canvas-container');
            const loading = document.getElementById('pdf-loading');
            const pageInput = document.getElementById('pdf-page-input');
            const pageCount = document.getElementById('pdf-page-count');
            const zoomLevel = document.getElementById('pdf-zoom-level');

            // Load PDF
            const url = '/api/books/{book_id}/pdf';

            pdfjsLib.getDocument(url).promise.then(function(pdf) {{
                pdfDoc = pdf;
                pageCount.textContent = pdf.numPages;
                pageInput.max = pdf.numPages;
                loading.style.display = 'none';

                // Load saved page from server first, fallback to localStorage
                fetch('/api/books/{book_id}/progress')
                    .then(function(r) {{ return r.json(); }})
                    .then(function(data) {{
                        if (data.success && data.data && data.data.chapter) {{
                            pageNum = Math.min(data.data.chapter, pdf.numPages);
                            pageInput.value = pageNum;
                        }} else {{
                            const savedPage = localStorage.getItem('pdf-page-{book_id}');
                            if (savedPage) {{
                                pageNum = Math.min(parseInt(savedPage), pdf.numPages);
                                pageInput.value = pageNum;
                            }}
                        }}
                        renderPage(pageNum);
                    }})
                    .catch(function() {{
                        const savedPage = localStorage.getItem('pdf-page-{book_id}');
                        if (savedPage) {{
                            pageNum = Math.min(parseInt(savedPage), pdf.numPages);
                            pageInput.value = pageNum;
                        }}
                        renderPage(pageNum);
                    }});
            }}).catch(function(error) {{
                loading.innerHTML = '<p class="error">Failed to load PDF: ' + error.message + '</p>';
            }});

            // Render page
            function renderPage(num) {{
                pageRendering = true;

                pdfDoc.getPage(num).then(function(page) {{
                    // Calculate scale based on fit mode
                    const containerWidth = container.clientWidth - 40;
                    const containerHeight = container.clientHeight - 40;
                    const viewport = page.getViewport({{ scale: 1 }});

                    if (fitMode === 'width') {{
                        scale = containerWidth / viewport.width;
                    }} else if (fitMode === 'page') {{
                        const scaleX = containerWidth / viewport.width;
                        const scaleY = containerHeight / viewport.height;
                        scale = Math.min(scaleX, scaleY);
                    }}

                    const scaledViewport = page.getViewport({{ scale: scale }});

                    // Set canvas dimensions
                    canvas.height = scaledViewport.height;
                    canvas.width = scaledViewport.width;

                    // Render
                    const renderContext = {{
                        canvasContext: ctx,
                        viewport: scaledViewport
                    }};

                    page.render(renderContext).promise.then(function() {{
                        pageRendering = false;

                        if (pageNumPending !== null) {{
                            renderPage(pageNumPending);
                            pageNumPending = null;
                        }}
                    }});
                }});

                // Update UI
                pageInput.value = num;
                zoomLevel.textContent = Math.round(scale * 100) + '%';

                // Save progress to localStorage and server
                localStorage.setItem('pdf-page-{book_id}', num);
                fetch('/api/books/{book_id}/progress', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{ chapter: num, block: 0, scroll_offset: 0 }})
                }}).catch(function() {{}});
            }}

            // Queue page render
            function queueRenderPage(num) {{
                if (pageRendering) {{
                    pageNumPending = num;
                }} else {{
                    renderPage(num);
                }}
            }}

            // Previous page
            document.getElementById('pdf-prev').addEventListener('click', function() {{
                if (pageNum <= 1) return;
                pageNum--;
                queueRenderPage(pageNum);
            }});

            // Next page
            document.getElementById('pdf-next').addEventListener('click', function() {{
                if (pageNum >= pdfDoc.numPages) return;
                pageNum++;
                queueRenderPage(pageNum);
            }});

            // Page input
            pageInput.addEventListener('change', function() {{
                const num = parseInt(pageInput.value);
                if (num >= 1 && num <= pdfDoc.numPages) {{
                    pageNum = num;
                    queueRenderPage(pageNum);
                }} else {{
                    pageInput.value = pageNum;
                }}
            }});

            // Zoom controls
            document.getElementById('pdf-zoom-in').addEventListener('click', function() {{
                fitMode = 'custom';
                scale = Math.min(scale * 1.25, 5);
                queueRenderPage(pageNum);
            }});

            document.getElementById('pdf-zoom-out').addEventListener('click', function() {{
                fitMode = 'custom';
                scale = Math.max(scale / 1.25, 0.25);
                queueRenderPage(pageNum);
            }});

            document.getElementById('pdf-fit-width').addEventListener('click', function() {{
                fitMode = 'width';
                queueRenderPage(pageNum);
            }});

            document.getElementById('pdf-fit-page').addEventListener('click', function() {{
                fitMode = 'page';
                queueRenderPage(pageNum);
            }});

            // Fullscreen
            document.getElementById('pdf-fullscreen').addEventListener('click', function() {{
                const viewer = document.querySelector('.pdf-viewer-layout');
                if (document.fullscreenElement) {{
                    document.exitFullscreen();
                }} else {{
                    viewer.requestFullscreen();
                }}
            }});

            // Keyboard navigation
            document.addEventListener('keydown', function(e) {{
                if (e.target.tagName === 'INPUT') return;

                switch(e.key) {{
                    case 'ArrowLeft':
                    case 'ArrowUp':
                    case 'PageUp':
                        if (pageNum > 1) {{
                            pageNum--;
                            queueRenderPage(pageNum);
                        }}
                        e.preventDefault();
                        break;
                    case 'ArrowRight':
                    case 'ArrowDown':
                    case 'PageDown':
                    case ' ':
                        if (pageNum < pdfDoc.numPages) {{
                            pageNum++;
                            queueRenderPage(pageNum);
                        }}
                        e.preventDefault();
                        break;
                    case 'Home':
                        pageNum = 1;
                        queueRenderPage(pageNum);
                        e.preventDefault();
                        break;
                    case 'End':
                        pageNum = pdfDoc.numPages;
                        queueRenderPage(pageNum);
                        e.preventDefault();
                        break;
                    case '+':
                    case '=':
                        fitMode = 'custom';
                        scale = Math.min(scale * 1.25, 5);
                        queueRenderPage(pageNum);
                        break;
                    case '-':
                        fitMode = 'custom';
                        scale = Math.max(scale / 1.25, 0.25);
                        queueRenderPage(pageNum);
                        break;
                }}
            }});

            // Resize handler
            window.addEventListener('resize', function() {{
                if (fitMode !== 'custom') {{
                    queueRenderPage(pageNum);
                }}
            }});
        }})();
        </script>
    "#,
        book_id = book_id
    )
}
