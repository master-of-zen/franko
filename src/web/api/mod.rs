//! Web API routes

mod books;
mod handlers;
mod helpers;
mod progress;
mod search;
mod types;

use super::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        // Library API
        .route("/books", get(books::list_books))
        .route("/books", post(books::add_book))
        .route("/books/:id", get(books::get_book))
        .route("/books/:id", delete(books::remove_book))
        .route("/books/:id/content", get(books::get_book_content))
        .route("/books/:id/chapter/:chapter", get(books::get_chapter))
        .route("/books/:id/cover", get(books::get_book_cover))
        .route("/books/:id/pdf", get(books::get_pdf_file))
        // Progress API
        .route("/books/:id/progress", get(progress::get_progress))
        .route("/books/:id/progress", post(progress::save_progress))
        // Reading time API
        .route(
            "/books/:id/reading-time",
            post(progress::update_reading_time),
        )
        // Bookmarks API
        .route("/books/:id/bookmarks", get(handlers::list_bookmarks))
        .route("/books/:id/bookmarks", post(handlers::add_bookmark))
        .route(
            "/books/:id/bookmarks/:bookmark_id",
            delete(handlers::remove_bookmark),
        )
        // Annotations API
        .route("/books/:id/annotations", get(handlers::list_annotations))
        .route("/books/:id/annotations", post(handlers::add_annotation))
        .route(
            "/books/:id/annotations/:annotation_id",
            delete(handlers::remove_annotation),
        )
        // Search API
        .route("/search", get(search::search_library))
        .route("/books/:id/search", get(search::search_book))
        // Statistics API
        .route("/statistics", get(handlers::get_library_statistics))
        .route("/books/:id/statistics", get(handlers::get_book_statistics))
        // Folder scanning
        .route("/scan-folder", post(books::scan_folder))
        // Settings API
        .route("/settings", get(handlers::get_settings))
        .route("/settings", put(handlers::update_settings))
}
