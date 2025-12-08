//! Book-related API handlers

use super::types::*;
use crate::web::AppState;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use std::sync::Arc;

/// Supported book extensions
const BOOK_EXTENSIONS: &[&str] = &["epub", "pdf", "md", "markdown", "txt", "text"];

/// List all books
pub async fn list_books(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListBooksQuery>,
) -> Json<ApiResponse<Vec<BookSummary>>> {
    let library = state.library.read().await;

    let mut books: Vec<BookSummary> = library
        .books()
        .iter()
        .map(|entry| BookSummary {
            id: entry.id.clone(),
            title: entry.metadata.title.clone(),
            authors: entry.metadata.authors.clone(),
            format: entry.format.clone(),
            progress: entry.progress,
            cover_url: entry
                .cover_path
                .as_ref()
                .map(|_p| format!("/api/books/{}/cover", entry.id)),
        })
        .collect();

    // Apply filters from query
    if let Some(format_filter) = &query.format {
        books.retain(|b| b.format.eq_ignore_ascii_case(format_filter));
    }
    if let Some(tag_filter) = &query.tag {
        // Tag filtering would require LibraryEntry to be accessible here
        // For now, we don't filter by tag in the API
        let _ = tag_filter;
    }

    // Apply sorting
    if let Some(sort_by) = &query.sort {
        match sort_by.as_str() {
            "title" => books.sort_by(|a, b| a.title.cmp(&b.title)),
            "author" => books.sort_by(|a, b| {
                let a_author = a.authors.first().map(|s| s.as_str()).unwrap_or("");
                let b_author = b.authors.first().map(|s| s.as_str()).unwrap_or("");
                a_author.cmp(b_author)
            }),
            "progress" => books.sort_by(|a, b| {
                b.progress
                    .partial_cmp(&a.progress)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            _ => {}
        }
    }

    // Apply pagination
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(books.len());
    let books: Vec<BookSummary> = books.into_iter().skip(offset).take(limit).collect();

    Json(ApiResponse::ok(books))
}

/// Add a single book
pub async fn add_book(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AddBookRequest>,
) -> Json<ApiResponse<BookSummary>> {
    let mut library = state.library.write().await;

    match library.add_book(&std::path::Path::new(&request.path), request.tags) {
        Ok(entry) => {
            let summary = BookSummary {
                id: entry.id.clone(),
                title: entry.metadata.title.clone(),
                authors: entry.metadata.authors.clone(),
                format: entry.format.clone(),
                progress: 0.0,
                cover_url: None,
            };

            // Save the library after adding the book
            if let Err(e) = library.save() {
                return Json(ApiResponse::err(format!(
                    "Book added but failed to save library: {}",
                    e
                )));
            }

            Json(ApiResponse::ok(summary))
        }
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// Scan a folder for books
pub async fn scan_folder(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ScanFolderRequest>,
) -> Json<ApiResponse<ScanResult>> {
    let path = std::path::Path::new(&request.path);

    if !path.exists() {
        return Json(ApiResponse::err("Path does not exist"));
    }

    if !path.is_dir() {
        return Json(ApiResponse::err("Path is not a directory"));
    }

    let recursive = request.recursive.unwrap_or(true);
    let tags = request.tags.unwrap_or_default();

    let mut library = state.library.write().await;
    let mut added = 0;
    let mut failed = 0;
    let mut books = Vec::new();
    let mut errors = Vec::new();

    let walker = if recursive {
        walkdir::WalkDir::new(path)
    } else {
        walkdir::WalkDir::new(path).max_depth(1)
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let file_path = entry.path();
        if file_path.is_file() {
            if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                if BOOK_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                    match library.add_book(file_path, Some(tags.clone())) {
                        Ok(entry) => {
                            books.push(BookSummary {
                                id: entry.id.clone(),
                                title: entry.metadata.title.clone(),
                                authors: entry.metadata.authors.clone(),
                                format: entry.format.clone(),
                                progress: 0.0,
                                cover_url: None,
                            });
                            added += 1;
                        }
                        Err(e) => {
                            errors.push(format!("{}: {}", file_path.display(), e));
                            failed += 1;
                        }
                    }
                }
            }
        }
    }

    // Save the library after adding books
    if added > 0 {
        if let Err(e) = library.save() {
            errors.push(format!("Failed to save library: {}", e));
        }
    }

    Json(ApiResponse::ok(ScanResult {
        added,
        failed,
        books,
        errors,
    }))
}

/// Get a single book's details
pub async fn get_book(
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

/// Remove a book from the library
pub async fn remove_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;

    match library.remove_book(&id) {
        Ok(_) => Json(ApiResponse::ok(())),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// Get all chapters content for a book
pub async fn get_book_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Vec<ChapterContent>>> {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => match crate::formats::parse_book(&entry.path) {
            Ok(book) => {
                let chapters: Vec<ChapterContent> = book
                    .content
                    .chapters
                    .iter()
                    .enumerate()
                    .map(|(i, ch)| {
                        let content_html = super::helpers::chapter_to_html(ch);
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
        },
        None => Json(ApiResponse::err("Book not found")),
    }
}

/// Get a single chapter's content
pub async fn get_chapter(
    State(state): State<Arc<AppState>>,
    Path((id, chapter_idx)): Path<(String, usize)>,
) -> Json<ApiResponse<ChapterContent>> {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => match crate::formats::parse_book(&entry.path) {
            Ok(book) => match book.content.chapters.get(chapter_idx) {
                Some(ch) => {
                    let content_html = super::helpers::chapter_to_html(ch);
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
                        next_chapter: book
                            .content
                            .chapters
                            .get(chapter_idx + 1)
                            .map(|c| c.id.clone()),
                    }))
                }
                None => Json(ApiResponse::err("Chapter not found")),
            },
            Err(e) => Json(ApiResponse::err(e.to_string())),
        },
        None => Json(ApiResponse::err("Book not found")),
    }
}

/// Get book cover image
pub async fn get_book_cover(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => {
            // Try to extract cover from the book file
            match crate::formats::extract_cover(&entry.path) {
                Ok(Some((data, mime))) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, mime)
                    .header(header::CACHE_CONTROL, "public, max-age=86400")
                    .body(Body::from(data))
                    .unwrap(),
                Ok(None) => {
                    // No cover found, return a placeholder or 404
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("No cover available"))
                        .unwrap()
                }
                Err(e) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Error extracting cover: {}", e)))
                    .unwrap(),
            }
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Book not found"))
            .unwrap(),
    }
}

/// Serve the raw PDF file for the PDF viewer
pub async fn get_pdf_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response<Body> {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => {
            if entry.format != "pdf" {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Not a PDF file"))
                    .unwrap();
            }

            match std::fs::read(&entry.path) {
                Ok(data) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/pdf")
                    .header(
                        header::CONTENT_DISPOSITION,
                        format!(
                            "inline; filename=\"{}\"",
                            entry
                                .path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("document.pdf")
                        ),
                    )
                    .body(Body::from(data))
                    .unwrap(),
                Err(e) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Error reading PDF: {}", e)))
                    .unwrap(),
            }
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Book not found"))
            .unwrap(),
    }
}
