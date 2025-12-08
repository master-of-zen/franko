//! Miscellaneous API handlers (bookmarks, annotations, statistics, settings)

use super::types::*;
use crate::web::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

/// List bookmarks for a book
pub async fn list_bookmarks(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Vec<crate::library::Bookmark>>> {
    let library = state.library.read().await;

    match library.get_bookmarks(&id) {
        Ok(bookmarks) => Json(ApiResponse::ok(bookmarks)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// Add a bookmark to a book
pub async fn add_bookmark(
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

/// Remove a bookmark from a book
pub async fn remove_bookmark(
    State(state): State<Arc<AppState>>,
    Path((id, bookmark_id)): Path<(String, String)>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;

    match library.remove_bookmark(&id, &bookmark_id) {
        Ok(_) => Json(ApiResponse::ok(())),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// List annotations for a book
pub async fn list_annotations(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Vec<crate::library::Annotation>>> {
    let library = state.library.read().await;

    match library.get_annotations(&id) {
        Ok(annotations) => Json(ApiResponse::ok(annotations)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// Add an annotation to a book
pub async fn add_annotation(
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

/// Remove an annotation from a book
pub async fn remove_annotation(
    State(state): State<Arc<AppState>>,
    Path((id, annotation_id)): Path<(String, String)>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;

    match library.remove_annotation(&id, &annotation_id) {
        Ok(_) => Json(ApiResponse::ok(())),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// Get statistics for a specific book
pub async fn get_book_statistics(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<crate::library::BookStats>> {
    let library = state.library.read().await;

    match library.get_book_stats(&id) {
        Some(stats) => Json(ApiResponse::ok(stats)),
        None => Json(ApiResponse::err("Book not found")),
    }
}

/// Get overall library statistics
pub async fn get_library_statistics(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<crate::library::LibraryStats>> {
    let library = state.library.read().await;
    Json(ApiResponse::ok(library.get_library_stats()))
}

/// Get current settings
pub async fn get_settings(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(
        serde_json::to_value(&state.config).unwrap_or_default(),
    ))
}

/// Update settings
pub async fn update_settings(
    State(_state): State<Arc<AppState>>,
    Json(_settings): Json<serde_json::Value>,
) -> Json<ApiResponse<()>> {
    // TODO: Implement settings update
    Json(ApiResponse::ok(()))
}
