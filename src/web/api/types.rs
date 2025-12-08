//! API response and request types

use serde::{Deserialize, Serialize};

/// Generic API response wrapper
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

/// Book summary for list views
#[derive(Serialize)]
pub struct BookSummary {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub format: String,
    pub progress: f64,
    pub cover_url: Option<String>,
}

/// Detailed book information
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

/// Chapter content response
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

/// Query parameters for listing books
#[derive(Deserialize)]
pub struct ListBooksQuery {
    pub format: Option<String>,
    pub tag: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Request to add a book
#[derive(Deserialize)]
pub struct AddBookRequest {
    pub path: String,
    pub tags: Option<Vec<String>>,
}

/// Request to scan a folder for books
#[derive(Deserialize)]
pub struct ScanFolderRequest {
    pub path: String,
    pub recursive: Option<bool>,
    pub tags: Option<Vec<String>>,
}

/// Result of folder scanning
#[derive(Serialize)]
pub struct ScanResult {
    pub added: usize,
    pub failed: usize,
    pub books: Vec<BookSummary>,
    pub errors: Vec<String>,
}

/// Progress request/response
#[derive(Deserialize, Serialize)]
pub struct ProgressRequest {
    pub chapter: usize,
    pub block: usize,
    pub scroll_offset: usize,
    #[serde(default)]
    pub progress: f64,
}

/// Bookmark request
#[derive(Deserialize)]
pub struct BookmarkRequest {
    pub name: Option<String>,
    pub chapter: usize,
    pub block: usize,
}

/// Annotation request
#[derive(Deserialize)]
pub struct AnnotationRequest {
    pub text: String,
    pub note: Option<String>,
    pub chapter: usize,
    pub block: usize,
    pub color: Option<String>,
}

/// Search query parameters
#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

/// Reading time update request
#[derive(Deserialize)]
pub struct ReadingTimeRequest {
    pub seconds: u64,
}

/// Search result for in-book search
#[derive(Serialize)]
pub struct SearchResult {
    pub chapter_id: String,
    pub chapter_title: Option<String>,
    pub chapter_index: usize,
    pub block_index: usize,
    pub snippet: String,
}
