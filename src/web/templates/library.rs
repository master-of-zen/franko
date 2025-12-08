//! Library page template

use crate::config::Config;
use crate::library::LibraryEntry;
use super::base::base;
use super::helpers::escape_html;

/// Generate the library page
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

            {add_book_modal}
        </main>
        {library_script}
    "#,
        book_rows = book_rows,
        add_book_modal = add_book_modal(),
        library_script = library_script(),
    );

    base("Library", &content, config)
}

/// Generate the add book modal HTML
fn add_book_modal() -> &'static str {
    r#"
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
    "#
}

/// Generate the library page JavaScript
fn library_script() -> &'static str {
    r#"
    <script>
    (function() {
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
        addBtn.addEventListener('click', () => {
            modal.classList.add('open');
            document.body.style.overflow = 'hidden';
        });

        // Close modal
        function closeModal() {
            modal.classList.remove('open');
            document.body.style.overflow = '';
            resultDiv.innerHTML = '';
            resultDiv.className = 'add-result';
        }

        closeBtn.addEventListener('click', closeModal);
        cancelBtn.addEventListener('click', closeModal);
        backdrop.addEventListener('click', closeModal);

        // Tab switching
        tabs.forEach(tab => {
            tab.addEventListener('click', () => {
                currentTab = tab.dataset.tab;
                tabs.forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                panels.forEach(p => p.classList.remove('active'));
                document.getElementById(currentTab + '-panel').classList.add('active');
                resultDiv.innerHTML = '';
                resultDiv.className = 'add-result';
            });
        });

        // Add book/folder
        confirmBtn.addEventListener('click', async () => {
            confirmBtn.disabled = true;
            confirmBtn.textContent = 'Adding...';
            resultDiv.innerHTML = '';
            resultDiv.className = 'add-result';

            try {
                if (currentTab === 'file') {
                    const path = document.getElementById('book-path').value.trim();
                    const tagsInput = document.getElementById('book-tags').value.trim();
                    const tags = tagsInput ? tagsInput.split(',').map(t => t.trim()).filter(t => t) : [];

                    if (!path) {
                        throw new Error('Please enter a book path');
                    }

                    const response = await fetch('/api/books', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ path, tags: tags.length ? tags : null })
                    });

                    const data = await response.json();

                    if (data.success) {
                        resultDiv.className = 'add-result success';
                        resultDiv.innerHTML = `<p>✓ Added: ${data.data.title}</p>`;
                        document.getElementById('book-path').value = '';
                        document.getElementById('book-tags').value = '';
                        setTimeout(() => location.reload(), 1500);
                    } else {
                        throw new Error(data.error || 'Failed to add book');
                    }
                } else {
                    const path = document.getElementById('folder-path').value.trim();
                    const recursive = document.getElementById('recursive-scan').checked;
                    const tagsInput = document.getElementById('folder-tags').value.trim();
                    const tags = tagsInput ? tagsInput.split(',').map(t => t.trim()).filter(t => t) : [];

                    if (!path) {
                        throw new Error('Please enter a folder path');
                    }

                    const response = await fetch('/api/scan-folder', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ path, recursive, tags: tags.length ? tags : null })
                    });

                    const data = await response.json();

                    if (data.success) {
                        const result = data.data;
                        resultDiv.className = 'add-result success';
                        let html = `<p>✓ Added ${result.added} book(s)`;
                        if (result.failed > 0) {
                            html += ` (${result.failed} failed)`;
                        }
                        html += `</p>`;

                        if (result.books.length > 0) {
                            html += '<ul class="added-books">';
                            result.books.slice(0, 5).forEach(book => {
                                html += `<li>${book.title}</li>`;
                            });
                            if (result.books.length > 5) {
                                html += `<li>... and ${result.books.length - 5} more</li>`;
                            }
                            html += '</ul>';
                        }

                        resultDiv.innerHTML = html;
                        document.getElementById('folder-path').value = '';
                        document.getElementById('folder-tags').value = '';
                        setTimeout(() => location.reload(), 2000);
                    } else {
                        throw new Error(data.error || 'Failed to scan folder');
                    }
                }
            } catch (err) {
                resultDiv.className = 'add-result error';
                resultDiv.innerHTML = `<p>✗ ${err.message}</p>`;
            } finally {
                confirmBtn.disabled = false;
                confirmBtn.textContent = 'Add';
            }
        });
    })();
    </script>
    "#
}
