//! Web API routes

use super::AppState;
use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        // Library API
        .route("/books", get(list_books))
        .route("/books", post(add_book))
        .route("/books/:id", get(get_book))
        .route("/books/:id", delete(remove_book))
        .route("/books/:id/content", get(get_book_content))
        .route("/books/:id/chapter/:chapter", get(get_chapter))
        // Progress API
        .route("/books/:id/progress", get(get_progress))
        .route("/books/:id/progress", post(save_progress))
        // Bookmarks API
        .route("/books/:id/bookmarks", get(list_bookmarks))
        .route("/books/:id/bookmarks", post(add_bookmark))
        .route("/books/:id/bookmarks/:bookmark_id", delete(remove_bookmark))
        // Annotations API
        .route("/books/:id/annotations", get(list_annotations))
        .route("/books/:id/annotations", post(add_annotation))
        .route("/books/:id/annotations/:annotation_id", delete(remove_annotation))
        // Search API
        .route("/search", get(search_library))
        .route("/books/:id/search", get(search_book))
        // Settings API
        .route("/settings", get(get_settings))
        .route("/settings", put(update_settings))
}

// Response types

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

#[derive(Serialize)]
pub struct BookSummary {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub format: String,
    pub progress: f64,
    pub cover_url: Option<String>,
}

#[derive(Serialize)]
pub struct BookDetail {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub published: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub subjects: Vec<String>,
    pub format: String,
    pub word_count: Option<usize>,
    pub reading_time: Option<usize>,
    pub chapter_count: usize,
    pub progress: f64,
}

#[derive(Serialize)]
pub struct ChapterContent {
    pub id: String,
    pub title: Option<String>,
    pub number: Option<usize>,
    pub content_html: String,
    pub word_count: usize,
    pub prev_chapter: Option<String>,
    pub next_chapter: Option<String>,
}

#[derive(Deserialize)]
pub struct ListBooksQuery {
    pub format: Option<String>,
    pub tag: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Deserialize)]
pub struct AddBookRequest {
    pub path: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct ProgressRequest {
    pub chapter: usize,
    pub block: usize,
    pub scroll_offset: usize,
}

#[derive(Deserialize)]
pub struct BookmarkRequest {
    pub name: Option<String>,
    pub chapter: usize,
    pub block: usize,
}

#[derive(Deserialize)]
pub struct AnnotationRequest {
    pub text: String,
    pub note: Option<String>,
    pub chapter: usize,
    pub block: usize,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

// Handlers

async fn list_books(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListBooksQuery>,
) -> Json<ApiResponse<Vec<BookSummary>>> {
    let library = state.library.read().await;
    
    let books: Vec<BookSummary> = library
        .books()
        .iter()
        .map(|entry| BookSummary {
            id: entry.id.clone(),
            title: entry.metadata.title.clone(),
            authors: entry.metadata.authors.clone(),
            format: entry.format.clone(),
            progress: entry.progress,
            cover_url: entry.cover_path.as_ref().map(|p| format!("/api/books/{}/cover", entry.id)),
        })
        .collect();

    Json(ApiResponse::ok(books))
}

async fn add_book(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AddBookRequest>,
) -> Json<ApiResponse<BookSummary>> {
    let mut library = state.library.write().await;
    
    match library.add_book(&std::path::Path::new(&request.path), request.tags) {
        Ok(entry) => Json(ApiResponse::ok(BookSummary {
            id: entry.id.clone(),
            title: entry.metadata.title.clone(),
            authors: entry.metadata.authors.clone(),
            format: entry.format.clone(),
            progress: 0.0,
            cover_url: None,
        })),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn get_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<BookDetail>> {
    let library = state.library.read().await;
    
    match library.get_book(&id) {
        Some(entry) => {
            // Parse the book to get chapter count
            let chapter_count = if let Ok(book) = crate::formats::parse_book(&entry.path) {
                book.content.chapters.len()
            } else {
                0
            };

            Json(ApiResponse::ok(BookDetail {
                id: entry.id.clone(),
                title: entry.metadata.title.clone(),
                authors: entry.metadata.authors.clone(),
                publisher: entry.metadata.publisher.clone(),
                published: entry.metadata.published.clone(),
                description: entry.metadata.description.clone(),
                language: entry.metadata.language.clone(),
                subjects: entry.metadata.subjects.clone(),
                format: entry.format.clone(),
                word_count: entry.metadata.word_count,
                reading_time: entry.metadata.reading_time,
                chapter_count,
                progress: entry.progress,
            }))
        }
        None => Json(ApiResponse::err("Book not found")),
    }
}

async fn remove_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;
    
    match library.remove_book(&id) {
        Ok(_) => Json(ApiResponse::ok(())),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn get_book_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Vec<ChapterContent>>> {
    let library = state.library.read().await;
    
    match library.get_book(&id) {
        Some(entry) => {
            match crate::formats::parse_book(&entry.path) {
                Ok(book) => {
                    let chapters: Vec<ChapterContent> = book
                        .content
                        .chapters
                        .iter()
                        .enumerate()
                        .map(|(i, ch)| {
                            let content_html = chapter_to_html(ch);
                            ChapterContent {
                                id: ch.id.clone(),
                                title: ch.title.clone(),
                                number: ch.number,
                                content_html,
                                word_count: ch.word_count(),
                                prev_chapter: if i > 0 {
                                    Some(book.content.chapters[i - 1].id.clone())
                                } else {
                                    None
                                },
                                next_chapter: book.content.chapters.get(i + 1).map(|c| c.id.clone()),
                            }
                        })
                        .collect();
                    Json(ApiResponse::ok(chapters))
                }
                Err(e) => Json(ApiResponse::err(e.to_string())),
            }
        }
        None => Json(ApiResponse::err("Book not found")),
    }
}

async fn get_chapter(
    State(state): State<Arc<AppState>>,
    Path((id, chapter_idx)): Path<(String, usize)>,
) -> Json<ApiResponse<ChapterContent>> {
    let library = state.library.read().await;
    
    match library.get_book(&id) {
        Some(entry) => {
            match crate::formats::parse_book(&entry.path) {
                Ok(book) => {
                    match book.content.chapters.get(chapter_idx) {
                        Some(ch) => {
                            let content_html = chapter_to_html(ch);
                            Json(ApiResponse::ok(ChapterContent {
                                id: ch.id.clone(),
                                title: ch.title.clone(),
                                number: ch.number,
                                content_html,
                                word_count: ch.word_count(),
                                prev_chapter: if chapter_idx > 0 {
                                    Some(book.content.chapters[chapter_idx - 1].id.clone())
                                } else {
                                    None
                                },
                                next_chapter: book.content.chapters.get(chapter_idx + 1).map(|c| c.id.clone()),
                            }))
                        }
                        None => Json(ApiResponse::err("Chapter not found")),
                    }
                }
                Err(e) => Json(ApiResponse::err(e.to_string())),
            }
        }
        None => Json(ApiResponse::err("Book not found")),
    }
}

async fn get_progress(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<ProgressRequest>> {
    let library = state.library.read().await;
    
    match library.get_book(&id) {
        Some(entry) => {
            Json(ApiResponse::ok(ProgressRequest {
                chapter: entry.position_chapter,
                block: entry.position_block,
                scroll_offset: entry.position_offset,
            }))
        }
        None => Json(ApiResponse::err("Book not found")),
    }
}

async fn save_progress(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(progress): Json<ProgressRequest>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;
    
    match library.update_progress(&id, progress.chapter, progress.block, progress.scroll_offset) {
        Ok(_) => Json(ApiResponse::ok(())),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn list_bookmarks(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Vec<crate::library::Bookmark>>> {
    let library = state.library.read().await;
    
    match library.get_bookmarks(&id) {
        Ok(bookmarks) => Json(ApiResponse::ok(bookmarks)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn add_bookmark(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<BookmarkRequest>,
) -> Json<ApiResponse<crate::library::Bookmark>> {
    let mut library = state.library.write().await;
    
    match library.add_bookmark(&id, request.name, request.chapter, request.block) {
        Ok(bookmark) => Json(ApiResponse::ok(bookmark)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn remove_bookmark(
    State(state): State<Arc<AppState>>,
    Path((id, bookmark_id)): Path<(String, String)>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;
    
    match library.remove_bookmark(&id, &bookmark_id) {
        Ok(_) => Json(ApiResponse::ok(())),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn list_annotations(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Vec<crate::library::Annotation>>> {
    let library = state.library.read().await;
    
    match library.get_annotations(&id) {
        Ok(annotations) => Json(ApiResponse::ok(annotations)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn add_annotation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<AnnotationRequest>,
) -> Json<ApiResponse<crate::library::Annotation>> {
    let mut library = state.library.write().await;
    
    match library.add_annotation(
        &id,
        request.text,
        request.note,
        request.chapter,
        request.block,
        request.color,
    ) {
        Ok(annotation) => Json(ApiResponse::ok(annotation)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn remove_annotation(
    State(state): State<Arc<AppState>>,
    Path((id, annotation_id)): Path<(String, String)>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;
    
    match library.remove_annotation(&id, &annotation_id) {
        Ok(_) => Json(ApiResponse::ok(())),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

async fn search_library(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Json<ApiResponse<Vec<BookSummary>>> {
    let library = state.library.read().await;
    
    let results: Vec<BookSummary> = library
        .search(&query.q)
        .iter()
        .take(query.limit.unwrap_or(20))
        .map(|entry| BookSummary {
            id: entry.id.clone(),
            title: entry.metadata.title.clone(),
            authors: entry.metadata.authors.clone(),
            format: entry.format.clone(),
            progress: entry.progress,
            cover_url: None,
        })
        .collect();

    Json(ApiResponse::ok(results))
}

async fn search_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(query): Query<SearchQuery>,
) -> Json<ApiResponse<Vec<SearchResult>>> {
    let library = state.library.read().await;
    
    match library.get_book(&id) {
        Some(entry) => {
            match crate::formats::parse_book(&entry.path) {
                Ok(book) => {
                    let mut results = Vec::new();
                    let query_lower = query.q.to_lowercase();

                    for (ch_idx, chapter) in book.content.chapters.iter().enumerate() {
                        for (block_idx, block) in chapter.blocks.iter().enumerate() {
                            let text = block.text();
                            if text.to_lowercase().contains(&query_lower) {
                                results.push(SearchResult {
                                    chapter_id: chapter.id.clone(),
                                    chapter_title: chapter.title.clone(),
                                    chapter_index: ch_idx,
                                    block_index: block_idx,
                                    snippet: get_snippet(&text, &query.q, 100),
                                });
                            }
                        }
                    }

                    Json(ApiResponse::ok(results))
                }
                Err(e) => Json(ApiResponse::err(e.to_string())),
            }
        }
        None => Json(ApiResponse::err("Book not found")),
    }
}

#[derive(Serialize)]
pub struct SearchResult {
    pub chapter_id: String,
    pub chapter_title: Option<String>,
    pub chapter_index: usize,
    pub block_index: usize,
    pub snippet: String,
}

async fn get_settings(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::to_value(&state.config).unwrap_or_default()))
}

async fn update_settings(
    State(_state): State<Arc<AppState>>,
    Json(_settings): Json<serde_json::Value>,
) -> Json<ApiResponse<()>> {
    // TODO: Implement settings update
    Json(ApiResponse::ok(()))
}

// Helper functions

fn chapter_to_html(chapter: &crate::formats::Chapter) -> String {
    use crate::formats::ContentBlock;
    
    let mut html = String::new();

    for block in &chapter.blocks {
        match block {
            ContentBlock::Paragraph { text, .. } => {
                html.push_str(&format!("<p>{}</p>\n", escape_html(text)));
            }
            ContentBlock::Heading { level, text } => {
                html.push_str(&format!("<h{}>{}</h{}>\n", level, escape_html(text), level));
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
                let lang_attr = language.as_ref().map(|l| format!(" class=\"language-{}\"", l)).unwrap_or_default();
                html.push_str(&format!("<pre><code{}>{}</code></pre>\n", lang_attr, escape_html(code)));
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
            ContentBlock::Image { src, alt, caption, .. } => {
                html.push_str("<figure>\n");
                let alt_attr = alt.as_ref().map(|a| format!(" alt=\"{}\"", escape_html(a))).unwrap_or_default();
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

fn get_snippet(text: &str, query: &str, max_len: usize) -> String {
    let lower = text.to_lowercase();
    let query_lower = query.to_lowercase();
    
    if let Some(pos) = lower.find(&query_lower) {
        let start = pos.saturating_sub(max_len / 2);
        let end = (pos + query.len() + max_len / 2).min(text.len());
        
        let mut snippet = String::new();
        if start > 0 {
            snippet.push_str("...");
        }
        snippet.push_str(&text[start..end]);
        if end < text.len() {
            snippet.push_str("...");
        }
        snippet
    } else {
        text.chars().take(max_len).collect()
    }
}
