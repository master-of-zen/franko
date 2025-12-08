//! Index/home page template

use crate::config::Config;
use crate::library::LibraryEntry;
use super::base::base;
use super::helpers::escape_html;

/// Generate the home page
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
