//! Full-text search module for Franko
//!
//! Provides fast book content searching using tantivy.

#[cfg(feature = "search")]
mod index;
mod query;

#[cfg(feature = "search")]
pub use index::{IndexEntry, SearchIndex};
pub use query::{
    highlight_matches, HighlightedMatch, SearchQuery, SearchResult, SearchResults, TextSearcher,
};
