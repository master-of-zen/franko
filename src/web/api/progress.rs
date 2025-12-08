//! Progress-related API handlers

use super::types::*;
use crate::web::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

/// Get reading progress for a book
pub async fn get_progress(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<ProgressRequest>> {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => Json(ApiResponse::ok(ProgressRequest {
            chapter: entry.position_chapter,
            block: entry.position_block,
            scroll_offset: entry.position_offset,
            progress: entry.progress,
        })),
        None => Json(ApiResponse::err("Book not found")),
    }
}

/// Save reading progress for a book
pub async fn save_progress(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(progress): Json<ProgressRequest>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;

    match library.update_progress(
        &id,
        progress.chapter,
        progress.block,
        progress.scroll_offset,
        progress.progress,
    ) {
        Ok(_) => {
            // Save library to persist the progress
            if let Err(e) = library.save() {
                return Json(ApiResponse::err(format!("Failed to save progress: {}", e)));
            }
            Json(ApiResponse::ok(()))
        }
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// Update reading time for a book
pub async fn update_reading_time(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<ReadingTimeRequest>,
) -> Json<ApiResponse<()>> {
    let mut library = state.library.write().await;

    match library.update_reading_time(&id, request.seconds) {
        Ok(_) => {
            // Save library to persist the change
            if let Err(e) = library.save() {
                return Json(ApiResponse::err(format!("Failed to save: {}", e)));
            }
            Json(ApiResponse::ok(()))
        }
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
