//! HTML templates for the web interface

use crate::config::Config;
use crate::formats::{Book, ContentBlock};
use crate::library::LibraryEntry;

pub fn base(title: &str, content: &str, config: &Config) -> String {
    let theme_class = if config.web.dark_mode {
        "dark"
    } else {
        "light"
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en" class="{theme_class}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Franko</title>
    <link rel="stylesheet" href="/static/style.css">
    <style>
        :root {{
            --font-family: {font_family};
            --font-size: {font_size}px;
            --line-height: {line_height};
        }}
    </style>
</head>
<body>
    <div id="app">
        {content}
    </div>
    <script src="/static/reader.js"></script>
</body>
</html>"#,
        theme_class = theme_class,
        title = escape_html(title),
        content = content,
        font_family = config.web.font_family,
        font_size = config.web.font_size,
        line_height = config.web.line_height,
    )
}

pub fn index(config: &Config, books: &[LibraryEntry]) -> String {
    let book_cards: String = books
        .iter()
        .take(10)
        .map(|book| {
            format!(
                r#"
            <a href="/read/{id}" class="book-card">
                <div class="book-cover">
                    {cover}
                </div>
                <div class="book-info">
                    <h3>{title}</h3>
                    <p class="author">{author}</p>
                    <div class="progress-bar">
                        <div class="progress" style="width: {progress}%"></div>
                    </div>
                </div>
            </a>
            "#,
                id = book.id,
                cover = if book.cover_path.is_some() {
                    format!(r#"<img src="/api/books/{}/cover" alt="Cover">"#, book.id)
                } else {
                    r#"<div class="placeholder-cover">📚</div>"#.to_string()
                },
                title = escape_html(&book.metadata.title),
                author = escape_html(&book.metadata.authors_string()),
                progress = (book.progress * 100.0) as i32,
            )
        })
        .collect();

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
        <main class="home">
            <section class="hero">
                <h2>Your Reading Companion</h2>
                <p>A powerful book reader for power users</p>
            </section>
            <section class="recent-books">
                <h2>Continue Reading</h2>
                <div class="book-grid">
                    {book_cards}
                </div>
                <a href="/library" class="view-all">View all books →</a>
            </section>
        </main>
    "#,
        book_cards = book_cards,
    );

    base("Home", &content, config)
}

pub fn library(config: &Config, books: &[LibraryEntry]) -> String {
    let book_rows: String = books
        .iter()
        .map(|book| {
            format!(
                r#"
            <tr>
                <td><a href="/read/{id}">{title}</a></td>
                <td>{author}</td>
                <td>{format}</td>
                <td>
                    <div class="progress-bar small">
                        <div class="progress" style="width: {progress}%"></div>
                    </div>
                    <span class="progress-text">{progress}%</span>
                </td>
                <td>
                    <a href="/book/{id}" class="btn-icon" title="Info">ℹ️</a>
                    <a href="/read/{id}" class="btn-icon" title="Read">📖</a>
                </td>
            </tr>
            "#,
                id = book.id,
                title = escape_html(&book.metadata.title),
                author = escape_html(&book.metadata.authors_string()),
                format = book.format.to_uppercase(),
                progress = (book.progress * 100.0) as i32,
            )
        })
        .collect();

    let content = format!(
        r#"
        <header class="site-header">
            <h1>📖 Franko</h1>
            <nav>
                <a href="/">Home</a>
                <a href="/library" class="active">Library</a>
                <a href="/settings">Settings</a>
            </nav>
        </header>
        <main class="library-page">
            <div class="library-header">
                <h2>Your Library</h2>
                <div class="library-controls">
                    <input type="search" id="search" placeholder="Search books...">
                    <select id="sort">
                        <option value="title">Sort by Title</option>
                        <option value="author">Sort by Author</option>
                        <option value="recent">Recently Read</option>
                        <option value="progress">Progress</option>
                    </select>
                    <button id="add-book-btn" class="btn btn-primary">
                        <span>+ Add Book</span>
                    </button>
                </div>
            </div>
            <table class="library-table">
                <thead>
                    <tr>
                        <th>Title</th>
                        <th>Author</th>
                        <th>Format</th>
                        <th>Progress</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {book_rows}
                </tbody>
            </table>

            <!-- Add Book Modal -->
            <div id="add-book-modal" class="modal">
                <div class="modal-backdrop"></div>
                <div class="modal-content">
                    <div class="modal-header">
                        <h3>Add Books to Library</h3>
                        <button class="modal-close" id="close-modal">×</button>
                    </div>
                    <div class="modal-body">
                        <div class="add-tabs">
                            <button class="add-tab active" data-tab="file">Single Book</button>
                            <button class="add-tab" data-tab="folder">Scan Folder</button>
                        </div>

                        <div class="add-panel active" id="file-panel">
                            <div class="form-group">
                                <label for="book-path">Book Path</label>
                                <input type="text" id="book-path" placeholder="/path/to/book.epub">
                                <small>Enter the full path to the book file</small>
                            </div>
                            <div class="form-group">
                                <label for="book-tags">Tags (optional)</label>
                                <input type="text" id="book-tags" placeholder="fiction, fantasy, favorite">
                                <small>Comma-separated list of tags</small>
                            </div>
                        </div>

                        <div class="add-panel" id="folder-panel">
                            <div class="form-group">
                                <label for="folder-path">Folder Path</label>
                                <input type="text" id="folder-path" placeholder="/path/to/books">
                                <small>Enter the path to a folder containing books</small>
                            </div>
                            <div class="form-group">
                                <label class="checkbox-label">
                                    <input type="checkbox" id="recursive-scan" checked>
                                    <span>Scan subfolders recursively</span>
                                </label>
                            </div>
                            <div class="form-group">
                                <label for="folder-tags">Tags (optional)</label>
                                <input type="text" id="folder-tags" placeholder="imported">
                                <small>Tags to apply to all imported books</small>
                            </div>
                        </div>

                        <div id="add-result" class="add-result"></div>
                    </div>
                    <div class="modal-footer">
                        <button class="btn btn-secondary" id="cancel-add">Cancel</button>
                        <button class="btn btn-primary" id="confirm-add">Add</button>
                    </div>
                </div>
            </div>
        </main>
        <script>
        (function() {{
            const modal = document.getElementById('add-book-modal');
            const addBtn = document.getElementById('add-book-btn');
            const closeBtn = document.getElementById('close-modal');
            const cancelBtn = document.getElementById('cancel-add');
            const confirmBtn = document.getElementById('confirm-add');
            const backdrop = modal.querySelector('.modal-backdrop');
            const tabs = document.querySelectorAll('.add-tab');
            const panels = document.querySelectorAll('.add-panel');
            const resultDiv = document.getElementById('add-result');

            let currentTab = 'file';

            // Open modal
            addBtn.addEventListener('click', () => {{
                modal.classList.add('open');
                document.body.style.overflow = 'hidden';
            }});

            // Close modal
            function closeModal() {{
                modal.classList.remove('open');
                document.body.style.overflow = '';
                resultDiv.innerHTML = '';
                resultDiv.className = 'add-result';
            }}

            closeBtn.addEventListener('click', closeModal);
            cancelBtn.addEventListener('click', closeModal);
            backdrop.addEventListener('click', closeModal);

            // Tab switching
            tabs.forEach(tab => {{
                tab.addEventListener('click', () => {{
                    currentTab = tab.dataset.tab;
                    tabs.forEach(t => t.classList.remove('active'));
                    tab.classList.add('active');
                    panels.forEach(p => p.classList.remove('active'));
                    document.getElementById(currentTab + '-panel').classList.add('active');
                    resultDiv.innerHTML = '';
                    resultDiv.className = 'add-result';
                }});
            }});

            // Add book/folder
            confirmBtn.addEventListener('click', async () => {{
                confirmBtn.disabled = true;
                confirmBtn.textContent = 'Adding...';
                resultDiv.innerHTML = '';
                resultDiv.className = 'add-result';

                try {{
                    if (currentTab === 'file') {{
                        const path = document.getElementById('book-path').value.trim();
                        const tagsInput = document.getElementById('book-tags').value.trim();
                        const tags = tagsInput ? tagsInput.split(',').map(t => t.trim()).filter(t => t) : [];

                        if (!path) {{
                            throw new Error('Please enter a book path');
                        }}

                        const response = await fetch('/api/books', {{
                            method: 'POST',
                            headers: {{ 'Content-Type': 'application/json' }},
                            body: JSON.stringify({{ path, tags: tags.length ? tags : null }})
                        }});

                        const data = await response.json();

                        if (data.success) {{
                            resultDiv.className = 'add-result success';
                            resultDiv.innerHTML = `<p>✓ Added: ${{data.data.title}}</p>`;
                            document.getElementById('book-path').value = '';
                            document.getElementById('book-tags').value = '';
                            setTimeout(() => location.reload(), 1500);
                        }} else {{
                            throw new Error(data.error || 'Failed to add book');
                        }}
                    }} else {{
                        const path = document.getElementById('folder-path').value.trim();
                        const recursive = document.getElementById('recursive-scan').checked;
                        const tagsInput = document.getElementById('folder-tags').value.trim();
                        const tags = tagsInput ? tagsInput.split(',').map(t => t.trim()).filter(t => t) : [];

                        if (!path) {{
                            throw new Error('Please enter a folder path');
                        }}

                        const response = await fetch('/api/scan-folder', {{
                            method: 'POST',
                            headers: {{ 'Content-Type': 'application/json' }},
                            body: JSON.stringify({{ path, recursive, tags: tags.length ? tags : null }})
                        }});

                        const data = await response.json();

                        if (data.success) {{
                            const result = data.data;
                            resultDiv.className = 'add-result success';
                            let html = `<p>✓ Added ${{result.added}} book(s)`;
                            if (result.failed > 0) {{
                                html += ` (${{result.failed}} failed)`;
                            }}
                            html += `</p>`;

                            if (result.books.length > 0) {{
                                html += '<ul class="added-books">';
                                result.books.slice(0, 5).forEach(book => {{
                                    html += `<li>${{book.title}}</li>`;
                                }});
                                if (result.books.length > 5) {{
                                    html += `<li>... and ${{result.books.length - 5}} more</li>`;
                                }}
                                html += '</ul>';
                            }}

                            resultDiv.innerHTML = html;
                            document.getElementById('folder-path').value = '';
                            document.getElementById('folder-tags').value = '';
                            setTimeout(() => location.reload(), 2000);
                        }} else {{
                            throw new Error(data.error || 'Failed to scan folder');
                        }}
                    }}
                }} catch (err) {{
                    resultDiv.className = 'add-result error';
                    resultDiv.innerHTML = `<p>✗ ${{err.message}}</p>`;
                }} finally {{
                    confirmBtn.disabled = false;
                    confirmBtn.textContent = 'Add';
                }}
            }});
        }})();
        </script>
    "#,
        book_rows = book_rows,
    );

    base("Library", &content, config)
}

pub fn reader(config: &Config, book: &Book, chapter_index: usize) -> String {
    let chapter = book.content.chapters.get(chapter_index);

    let chapter_content = if let Some(ch) = chapter {
        chapter_to_html(ch)
    } else {
        "<p>Chapter not found</p>".to_string()
    };

    let toc_items: String = book
        .content
        .chapters
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let active = if i == chapter_index {
                " class=\"active\""
            } else {
                ""
            };
            format!(
                r#"<li{active}><a href="?chapter={i}">{title}</a></li>"#,
                active = active,
                i = i,
                title = escape_html(&ch.display_title()),
            )
        })
        .collect();

    let prev_link = if chapter_index > 0 {
        format!(
            r#"<a href="?chapter={}" class="nav-prev">← Previous</a>"#,
            chapter_index - 1
        )
    } else {
        String::new()
    };

    let next_link = if chapter_index < book.content.chapters.len() - 1 {
        format!(
            r#"<a href="?chapter={}" class="nav-next">Next →</a>"#,
            chapter_index + 1
        )
    } else {
        String::new()
    };

    let content = format!(
        r#"
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
            <main class="reader-main">
                <header class="reader-header">
                    <a href="/library" class="btn-icon" data-tooltip="Back to Library" title="Back to Library">←</a>
                    <button id="toggle-sidebar" class="btn-icon" data-tooltip="Table of Contents">☰</button>
                    <h1>{title}</h1>
                    <div class="reader-controls">
                        <div class="layout-switcher" data-tooltip="Reading Layout">
                            <button id="layout-scroll" class="btn-icon layout-btn active" data-layout="scroll" title="Continuous Scroll">📜</button>
                            <button id="layout-paged" class="btn-icon layout-btn" data-layout="paged" title="Paged View">📄</button>
                            <button id="layout-dual" class="btn-icon layout-btn" data-layout="dual" title="Dual Page">📖</button>
                        </div>
                        <button id="decrease-font" class="btn-icon" data-tooltip="Decrease Font">A-</button>
                        <button id="increase-font" class="btn-icon" data-tooltip="Increase Font">A+</button>
                        <button id="toggle-theme" class="btn-icon" data-tooltip="Toggle Theme">🌓</button>
                        <button id="toggle-fullscreen" class="btn-icon" data-tooltip="Fullscreen">⛶</button>
                    </div>
                </header>
                <div class="reader-container" id="reader-container" data-layout="scroll">
                    <article class="reader-content" id="content">
                        {chapter_content}
                    </article>
                </div>
                <div class="page-controls" id="page-controls" style="display: none;">
                    <button id="page-prev" class="btn-icon page-nav">←</button>
                    <span class="page-indicator" id="page-indicator">Page 1</span>
                    <button id="page-next" class="btn-icon page-nav">→</button>
                </div>
                <nav class="chapter-nav">
                    {prev_link}
                    <span class="chapter-indicator">Chapter {chapter_num} of {total_chapters}</span>
                    {next_link}
                </nav>
            </main>
        </div>
        <div class="reader-progress" id="progress">
            <div class="progress-fill" id="progress-fill"></div>
        </div>
    "#,
        toc_items = toc_items,
        title = escape_html(&book.metadata.title),
        chapter_content = chapter_content,
        prev_link = prev_link,
        next_link = next_link,
        chapter_num = chapter_index + 1,
        total_chapters = book.content.chapters.len(),
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
                
                // Load saved page
                const savedPage = localStorage.getItem('pdf-page-{book_id}');
                if (savedPage) {{
                    pageNum = Math.min(parseInt(savedPage), pdf.numPages);
                    pageInput.value = pageNum;
                }}
                
                renderPage(pageNum);
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
                
                // Save progress
                localStorage.setItem('pdf-page-{book_id}', num);
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
        title = escape_html(title),
        book_id = book_id,
    );

    base(title, &content, config)
}

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

pub fn settings(config: &Config) -> String {
    let _dark_checked = if config.web.dark_mode { "checked" } else { "" };
    let justify_checked = if config.reader.justify { "checked" } else { "" };
    let hyphenate_checked = if config.reader.hyphenation {
        "checked"
    } else {
        ""
    };
    let show_progress_checked = if config.tui.progress_bar {
        "checked"
    } else {
        ""
    };

    let content = format!(
        r#"
        <header class="site-header">
            <h1>📖 Franko</h1>
            <nav>
                <a href="/">Home</a>
                <a href="/library">Library</a>
                <a href="/settings" class="active">Settings</a>
            </nav>
        </header>
        <main class="settings-page">
            <div class="settings-header">
                <h2>Settings</h2>
                <p class="settings-subtitle">Customize your reading experience</p>
            </div>

            <div class="settings-grid">
                <!-- Appearance Section -->
                <section class="settings-card">
                    <div class="settings-card-header">
                        <span class="settings-icon">🎨</span>
                        <h3>Appearance</h3>
                    </div>
                    <div class="settings-card-body">
                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="theme">Theme</label>
                                <p class="setting-description">Choose your preferred color scheme</p>
                            </div>
                            <div class="setting-control">
                                <div class="theme-switcher">
                                    <button class="theme-btn {dark_active}" data-theme="dark" title="Dark">🌙</button>
                                    <button class="theme-btn {light_active}" data-theme="light" title="Light">☀️</button>
                                    <button class="theme-btn" data-theme="auto" title="Auto">🌓</button>
                                </div>
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="accent-color">Accent Color</label>
                                <p class="setting-description">Primary color for highlights and buttons</p>
                            </div>
                            <div class="setting-control">
                                <div class="color-picker">
                                    <button class="color-btn active" data-color="indigo" style="--btn-color: #6366f1"></button>
                                    <button class="color-btn" data-color="purple" style="--btn-color: #a855f7"></button>
                                    <button class="color-btn" data-color="blue" style="--btn-color: #3b82f6"></button>
                                    <button class="color-btn" data-color="green" style="--btn-color: #22c55e"></button>
                                    <button class="color-btn" data-color="orange" style="--btn-color: #f97316"></button>
                                    <button class="color-btn" data-color="pink" style="--btn-color: #ec4899"></button>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <!-- Typography Section -->
                <section class="settings-card">
                    <div class="settings-card-header">
                        <span class="settings-icon">📝</span>
                        <h3>Typography</h3>
                    </div>
                    <div class="settings-card-body">
                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="font-family">Font Family</label>
                                <p class="setting-description">Choose a comfortable reading font</p>
                            </div>
                            <div class="setting-control">
                                <select id="font-family" class="setting-select">
                                    <option value="system" {font_system}>System Default</option>
                                    <option value="serif" {font_serif}>Georgia (Serif)</option>
                                    <option value="sans" {font_sans}>Inter (Sans-serif)</option>
                                    <option value="mono" {font_mono}>JetBrains Mono</option>
                                    <option value="literata" {font_literata}>Literata</option>
                                    <option value="merriweather" {font_merriweather}>Merriweather</option>
                                </select>
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="font-size">Font Size</label>
                                <p class="setting-description">Base text size: <span id="font-size-value">{font_size}px</span></p>
                            </div>
                            <div class="setting-control">
                                <input type="range" id="font-size" class="setting-range"
                                       min="12" max="32" value="{font_size}" step="1">
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="line-height">Line Height</label>
                                <p class="setting-description">Spacing between lines: <span id="line-height-value">{line_height}</span></p>
                            </div>
                            <div class="setting-control">
                                <input type="range" id="line-height" class="setting-range"
                                       min="1.2" max="2.4" value="{line_height}" step="0.1">
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="text-width">Text Width</label>
                                <p class="setting-description">Maximum width of text column</p>
                            </div>
                            <div class="setting-control">
                                <select id="text-width" class="setting-select">
                                    <option value="narrow">Narrow (600px)</option>
                                    <option value="medium" selected>Medium (720px)</option>
                                    <option value="wide">Wide (900px)</option>
                                    <option value="full">Full Width</option>
                                </select>
                            </div>
                        </div>
                    </div>
                </section>

                <!-- Reading Section -->
                <section class="settings-card">
                    <div class="settings-card-header">
                        <span class="settings-icon">📖</span>
                        <h3>Reading</h3>
                    </div>
                    <div class="settings-card-body">
                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="justify-text">Justify Text</label>
                                <p class="setting-description">Align text to both margins</p>
                            </div>
                            <div class="setting-control">
                                <label class="toggle">
                                    <input type="checkbox" id="justify-text" {justify_checked}>
                                    <span class="toggle-slider"></span>
                                </label>
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="hyphenation">Hyphenation</label>
                                <p class="setting-description">Break long words at line ends</p>
                            </div>
                            <div class="setting-control">
                                <label class="toggle">
                                    <input type="checkbox" id="hyphenation" {hyphenate_checked}>
                                    <span class="toggle-slider"></span>
                                </label>
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="show-progress">Show Progress Bar</label>
                                <p class="setting-description">Display reading progress at bottom</p>
                            </div>
                            <div class="setting-control">
                                <label class="toggle">
                                    <input type="checkbox" id="show-progress" {show_progress_checked}>
                                    <span class="toggle-slider"></span>
                                </label>
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="scroll-behavior">Scroll Behavior</label>
                                <p class="setting-description">How pages transition while reading</p>
                            </div>
                            <div class="setting-control">
                                <select id="scroll-behavior" class="setting-select">
                                    <option value="smooth">Smooth Scroll</option>
                                    <option value="instant">Instant</option>
                                    <option value="paginated">Paginated</option>
                                </select>
                            </div>
                        </div>
                    </div>
                </section>

                <!-- Keyboard Section -->
                <section class="settings-card">
                    <div class="settings-card-header">
                        <span class="settings-icon">⌨️</span>
                        <h3>Keyboard Shortcuts</h3>
                    </div>
                    <div class="settings-card-body">
                        <div class="shortcuts-list">
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>j</kbd> / <kbd>k</kbd></span>
                                <span class="shortcut-desc">Scroll down / up</span>
                            </div>
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>h</kbd> / <kbd>l</kbd></span>
                                <span class="shortcut-desc">Previous / Next chapter</span>
                            </div>
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>Space</kbd></span>
                                <span class="shortcut-desc">Page down</span>
                            </div>
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>g</kbd> / <kbd>G</kbd></span>
                                <span class="shortcut-desc">Go to top / bottom</span>
                            </div>
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>t</kbd></span>
                                <span class="shortcut-desc">Toggle table of contents</span>
                            </div>
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>d</kbd></span>
                                <span class="shortcut-desc">Toggle dark mode</span>
                            </div>
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>+</kbd> / <kbd>-</kbd></span>
                                <span class="shortcut-desc">Increase / Decrease font size</span>
                            </div>
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>/</kbd></span>
                                <span class="shortcut-desc">Search</span>
                            </div>
                            <div class="shortcut-item">
                                <span class="shortcut-keys"><kbd>Esc</kbd></span>
                                <span class="shortcut-desc">Close sidebar / dialogs</span>
                            </div>
                        </div>

                        <div class="setting-item" style="margin-top: 1.5rem;">
                            <div class="setting-info">
                                <label for="keybinding-preset">Keybinding Preset</label>
                                <p class="setting-description">Choose your preferred key layout</p>
                            </div>
                            <div class="setting-control">
                                <select id="keybinding-preset" class="setting-select">
                                    <option value="vim" selected>Vim-style</option>
                                    <option value="emacs">Emacs-style</option>
                                    <option value="reader">Reader-style</option>
                                    <option value="custom">Custom</option>
                                </select>
                            </div>
                        </div>
                    </div>
                </section>

                <!-- Library Section -->
                <section class="settings-card">
                    <div class="settings-card-header">
                        <span class="settings-icon">📚</span>
                        <h3>Library</h3>
                    </div>
                    <div class="settings-card-body">
                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="library-path">Library Path</label>
                                <p class="setting-description">Where your books are stored</p>
                            </div>
                            <div class="setting-control">
                                <input type="text" id="library-path" class="setting-input"
                                       value="{library_path}" placeholder="~/Books">
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="auto-scan">Auto-scan Library</label>
                                <p class="setting-description">Automatically detect new books</p>
                            </div>
                            <div class="setting-control">
                                <label class="toggle">
                                    <input type="checkbox" id="auto-scan" checked>
                                    <span class="toggle-slider"></span>
                                </label>
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="default-view">Default View</label>
                                <p class="setting-description">How to display your library</p>
                            </div>
                            <div class="setting-control">
                                <select id="default-view" class="setting-select">
                                    <option value="grid">Grid</option>
                                    <option value="list" selected>List</option>
                                    <option value="compact">Compact</option>
                                </select>
                            </div>
                        </div>
                    </div>
                </section>

                <!-- Advanced Section -->
                <section class="settings-card">
                    <div class="settings-card-header">
                        <span class="settings-icon">⚙️</span>
                        <h3>Advanced</h3>
                    </div>
                    <div class="settings-card-body">
                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="web-port">Web Server Port</label>
                                <p class="setting-description">Port for the web interface</p>
                            </div>
                            <div class="setting-control">
                                <input type="number" id="web-port" class="setting-input"
                                       value="{web_port}" min="1024" max="65535">
                            </div>
                        </div>

                        <div class="setting-item">
                            <div class="setting-info">
                                <label for="open-browser">Open Browser on Start</label>
                                <p class="setting-description">Automatically open in browser when server starts</p>
                            </div>
                            <div class="setting-control">
                                <label class="toggle">
                                    <input type="checkbox" id="open-browser" checked>
                                    <span class="toggle-slider"></span>
                                </label>
                            </div>
                        </div>

                        <div class="setting-actions">
                            <button class="btn" id="export-config">📤 Export Config</button>
                            <button class="btn" id="import-config">📥 Import Config</button>
                            <button class="btn danger" id="reset-settings">🔄 Reset to Defaults</button>
                        </div>
                    </div>
                </section>
            </div>

            <div class="settings-footer">
                <button class="btn primary" id="save-settings">💾 Save All Settings</button>
                <p class="settings-note">Settings are automatically saved to your browser. Click save to persist to server.</p>
            </div>
        </main>
    "#,
        dark_active = if config.web.dark_mode { "active" } else { "" },
        light_active = if !config.web.dark_mode { "active" } else { "" },
        font_size = config.web.font_size,
        line_height = config.web.line_height,
        font_system = if config.web.font_family.contains("system") {
            "selected"
        } else {
            ""
        },
        font_serif = if config.web.font_family.contains("Georgia") {
            "selected"
        } else {
            ""
        },
        font_sans = if config.web.font_family.contains("Inter") {
            "selected"
        } else {
            ""
        },
        font_mono = if config.web.font_family.contains("JetBrains") {
            "selected"
        } else {
            ""
        },
        font_literata = if config.web.font_family.contains("Literata") {
            "selected"
        } else {
            ""
        },
        font_merriweather = if config.web.font_family.contains("Merriweather") {
            "selected"
        } else {
            ""
        },
        justify_checked = justify_checked,
        hyphenate_checked = hyphenate_checked,
        show_progress_checked = show_progress_checked,
        library_path = config
            .library
            .books_dir
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        web_port = config.web.port,
    );

    base("Settings", &content, config)
}

fn chapter_to_html(chapter: &crate::formats::Chapter) -> String {
    let mut html = String::new();

    if let Some(title) = &chapter.title {
        html.push_str(&format!(
            "<h2 class=\"chapter-title\">{}</h2>\n",
            escape_html(title)
        ));
    }

    for block in &chapter.blocks {
        match block {
            ContentBlock::Paragraph { text, .. } => {
                html.push_str(&format!("<p>{}</p>\n", escape_html(text)));
            }
            ContentBlock::Heading { level, text } => {
                let tag_level = (*level + 1).min(6); // Offset by 1 since chapter title is h2
                html.push_str(&format!(
                    "<h{l}>{t}</h{l}>\n",
                    l = tag_level,
                    t = escape_html(text)
                ));
            }
            ContentBlock::Quote { text, attribution } => {
                html.push_str("<blockquote>\n");
                html.push_str(&format!("<p>{}</p>\n", escape_html(text)));
                if let Some(attr) = attribution {
                    html.push_str(&format!("<cite>— {}</cite>\n", escape_html(attr)));
                }
                html.push_str("</blockquote>\n");
            }
            ContentBlock::Code { language, code } => {
                let lang_class = language
                    .as_ref()
                    .map(|l| format!(" class=\"language-{}\"", l))
                    .unwrap_or_default();
                html.push_str(&format!(
                    "<pre><code{}>{}</code></pre>\n",
                    lang_class,
                    escape_html(code)
                ));
            }
            ContentBlock::List { ordered, items } => {
                let tag = if *ordered { "ol" } else { "ul" };
                html.push_str(&format!("<{}>\n", tag));
                for item in items {
                    html.push_str(&format!("<li>{}</li>\n", escape_html(item)));
                }
                html.push_str(&format!("</{}>\n", tag));
            }
            ContentBlock::Separator => {
                html.push_str("<hr>\n");
            }
            ContentBlock::Image {
                src, alt, caption, ..
            } => {
                html.push_str("<figure>\n");
                let alt_attr = alt
                    .as_ref()
                    .map(|a| format!(" alt=\"{}\"", escape_html(a)))
                    .unwrap_or_default();
                html.push_str(&format!("<img src=\"{}\"{}>\n", escape_html(src), alt_attr));
                if let Some(cap) = caption {
                    html.push_str(&format!("<figcaption>{}</figcaption>\n", escape_html(cap)));
                }
                html.push_str("</figure>\n");
            }
            ContentBlock::Table { headers, rows } => {
                html.push_str("<table>\n<thead>\n<tr>\n");
                for header in headers {
                    html.push_str(&format!("<th>{}</th>\n", escape_html(header)));
                }
                html.push_str("</tr>\n</thead>\n<tbody>\n");
                for row in rows {
                    html.push_str("<tr>\n");
                    for cell in row {
                        html.push_str(&format!("<td>{}</td>\n", escape_html(cell)));
                    }
                    html.push_str("</tr>\n");
                }
                html.push_str("</tbody>\n</table>\n");
            }
            ContentBlock::Footnote { id, content } => {
                html.push_str(&format!(
                    r#"<aside class="footnote" id="fn-{id}"><sup>{id}</sup> {content}</aside>"#,
                    id = escape_html(id),
                    content = escape_html(content),
                ));
            }
            _ => {}
        }
    }

    html
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
