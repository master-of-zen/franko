//! Web page handlers

use super::templates;
use super::AppState;
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ReaderQuery {
    chapter: Option<usize>,
}

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let library = state.library.read().await;
    let books = library.books();

    Html(templates::index(&state.config, &books))
}

pub async fn library(State(state): State<Arc<AppState>>) -> Html<String> {
    let library = state.library.read().await;
    let books = library.books();

    Html(templates::library(&state.config, &books))
}

pub async fn reader(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(_query): Query<ReaderQuery>,
) -> Html<String> {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => {
            // Use PDF viewer for PDF files
            if entry.format == "pdf" {
                return Html(templates::pdf_reader(&state.config, &entry.id, &entry.metadata.title));
            }

            match crate::formats::parse_book(&entry.path) {
                Ok(book) => Html(templates::reader(&state.config, &book, 0)),
                Err(e) => Html(templates::error(&format!("Failed to parse book: {}", e))),
            }
        }
        None => Html(templates::error("Book not found")),
    }
}

pub async fn reader_chapter(
    State(state): State<Arc<AppState>>,
    Path((id, chapter)): Path<(String, usize)>,
) -> Html<String> {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => match crate::formats::parse_book(&entry.path) {
            Ok(book) => Html(templates::reader(&state.config, &book, chapter)),
            Err(e) => Html(templates::error(&format!("Failed to parse book: {}", e))),
        },
        None => Html(templates::error("Book not found")),
    }
}

pub async fn book_info(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Html<String> {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => match crate::formats::parse_book(&entry.path) {
            Ok(book) => Html(templates::book_info(&state.config, &book)),
            Err(e) => Html(templates::error(&format!("Failed to parse book: {}", e))),
        },
        None => Html(templates::error("Book not found")),
    }
}

/// Handler for single-book mode (when opening a book directly with --web)
pub async fn single_book_reader(State(state): State<Arc<AppState>>) -> Html<String> {
    if let Some(ref book_lock) = state.current_book {
        let book = book_lock.read().await;
        Html(templates::reader(&state.config, &book, 0))
    } else {
        Html(templates::error("No book loaded"))
    }
}

/// Handler for single-book chapter navigation
pub async fn single_book_chapter(
    State(state): State<Arc<AppState>>,
    Path(chapter): Path<usize>,
) -> Html<String> {
    if let Some(ref book_lock) = state.current_book {
        let book = book_lock.read().await;
        Html(templates::reader(&state.config, &book, chapter))
    } else {
        Html(templates::error("No book loaded"))
    }
}

/// Handler for settings page
pub async fn settings(State(state): State<Arc<AppState>>) -> Html<String> {
    Html(templates::settings(&state.config))
}
