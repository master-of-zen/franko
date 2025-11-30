//! Full-text search module for Franko
//!
//! Provides fast book content searching using tantivy.

#[cfg(feature = "search")]
mod index;
mod query;

#[cfg(feature = "search")]
pub use index::{SearchIndex, IndexEntry};
pub use query::{SearchQuery, SearchResult, SearchResults, HighlightedMatch, highlight_matches, TextSearcher};
