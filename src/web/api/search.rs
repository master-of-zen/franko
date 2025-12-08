//! Search-related API handlers

use super::types::*;
use crate::web::AppState;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;

/// Search the library
pub async fn search_library(
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

/// Search within a book
pub async fn search_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(query): Query<SearchQuery>,
) -> Json<ApiResponse<Vec<SearchResult>>> {
    let library = state.library.read().await;

    match library.get_book(&id) {
        Some(entry) => match crate::formats::parse_book(&entry.path) {
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
        },
        None => Json(ApiResponse::err("Book not found")),
    }
}

/// Extract a snippet of text around a search query
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
