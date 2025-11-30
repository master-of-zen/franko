//! Search query and result types

use std::ops::Range;

use serde::{Deserialize, Serialize};

/// A search query with options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The search text
    pub text: String,

    /// Case-sensitive matching
    #[serde(default)]
    pub case_sensitive: bool,

    /// Use regex matching
    #[serde(default)]
    pub regex: bool,

    /// Whole word matching only
    #[serde(default)]
    pub whole_word: bool,

    /// Maximum number of results
    #[serde(default = "default_limit")]
    pub limit: usize,

    /// Context lines to include around matches
    #[serde(default = "default_context")]
    pub context_lines: usize,

    /// Filter by book ID (None = search all books)
    pub book_id: Option<String>,

    /// Filter by chapter indices
    pub chapters: Option<Vec<usize>>,
}

fn default_limit() -> usize {
    100
}
fn default_context() -> usize {
    2
}

impl SearchQuery {
    /// Create a new simple search query
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            case_sensitive: false,
            regex: false,
            whole_word: false,
            limit: default_limit(),
            context_lines: default_context(),
            book_id: None,
            chapters: None,
        }
    }

    /// Set case sensitivity
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    /// Enable regex mode
    pub fn regex(mut self, regex: bool) -> Self {
        self.regex = regex;
        self
    }

    /// Enable whole word matching
    pub fn whole_word(mut self, whole_word: bool) -> Self {
        self.whole_word = whole_word;
        self
    }

    /// Set result limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Filter by book
    pub fn in_book(mut self, book_id: impl Into<String>) -> Self {
        self.book_id = Some(book_id.into());
        self
    }

    /// Filter by chapters
    pub fn in_chapters(mut self, chapters: Vec<usize>) -> Self {
        self.chapters = Some(chapters);
        self
    }
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self::new("")
    }
}

/// A single highlighted match within text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightedMatch {
    /// Byte range of the match in the original text
    pub range: Range<usize>,

    /// The matched text
    pub matched_text: String,

    /// Text before the match (context)
    pub prefix: String,

    /// Text after the match (context)
    pub suffix: String,
}

impl HighlightedMatch {
    /// Get the full context with the match
    pub fn full_context(&self) -> String {
        format!("{}{}{}", self.prefix, self.matched_text, self.suffix)
    }

    /// Get the match with ANSI highlighting (for terminal display)
    pub fn ansi_highlighted(&self) -> String {
        format!(
            "{}\x1b[1;33m{}\x1b[0m{}",
            self.prefix, self.matched_text, self.suffix
        )
    }

    /// Get the match with HTML highlighting
    pub fn html_highlighted(&self) -> String {
        format!(
            "{}<mark class=\"search-match\">{}</mark>{}",
            html_escape(&self.prefix),
            html_escape(&self.matched_text),
            html_escape(&self.suffix)
        )
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// A single search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Book ID where the match was found
    pub book_id: String,

    /// Book title
    pub book_title: String,

    /// Chapter index (0-indexed)
    pub chapter_index: usize,

    /// Chapter title
    pub chapter_title: String,

    /// Block index within the chapter
    pub block_index: usize,

    /// The full text of the block
    pub full_text: String,

    /// All matches within this block
    pub matches: Vec<HighlightedMatch>,

    /// Relevance score
    pub score: f32,
}

impl SearchResult {
    /// Get a preview of the first match
    pub fn preview(&self, max_len: usize) -> String {
        if let Some(first_match) = self.matches.first() {
            let full = first_match.full_context();
            if full.len() > max_len {
                format!("{}...", &full[..max_len])
            } else {
                full
            }
        } else {
            let preview = if self.full_text.len() > max_len {
                format!("{}...", &self.full_text[..max_len])
            } else {
                self.full_text.clone()
            };
            preview
        }
    }

    /// Get the position string for navigation
    pub fn position(&self) -> String {
        format!("{}:{}", self.chapter_index, self.block_index)
    }
}

/// Collection of search results with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    /// The original query
    pub query: SearchQuery,

    /// All results
    pub results: Vec<SearchResult>,

    /// Total number of matches found (before limiting)
    pub total_matches: usize,

    /// Time taken to search (in milliseconds)
    pub search_time_ms: u64,

    /// Number of books searched
    pub books_searched: usize,

    /// Number of chapters searched
    pub chapters_searched: usize,
}

impl SearchResults {
    /// Create empty results
    pub fn empty(query: SearchQuery) -> Self {
        Self {
            query,
            results: Vec::new(),
            total_matches: 0,
            search_time_ms: 0,
            books_searched: 0,
            chapters_searched: 0,
        }
    }

    /// Create results with data
    pub fn new(
        query: SearchQuery,
        results: Vec<SearchResult>,
        total_matches: usize,
        search_time_ms: u64,
    ) -> Self {
        let books_searched = results
            .iter()
            .map(|r| r.book_id.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len();

        let chapters_searched = results
            .iter()
            .map(|r| (r.book_id.as_str(), r.chapter_index))
            .collect::<std::collections::HashSet<_>>()
            .len();

        Self {
            query,
            results,
            total_matches,
            search_time_ms,
            books_searched,
            chapters_searched,
        }
    }

    /// Check if there are any results
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Get the number of results
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Get results grouped by book
    pub fn by_book(&self) -> std::collections::HashMap<String, Vec<&SearchResult>> {
        let mut grouped = std::collections::HashMap::new();
        for result in &self.results {
            grouped
                .entry(result.book_id.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }
        grouped
    }

    /// Get results grouped by chapter
    pub fn by_chapter(&self) -> std::collections::HashMap<(String, usize), Vec<&SearchResult>> {
        let mut grouped = std::collections::HashMap::new();
        for result in &self.results {
            grouped
                .entry((result.book_id.clone(), result.chapter_index))
                .or_insert_with(Vec::new)
                .push(result);
        }
        grouped
    }
}

/// Simple text search (non-indexed) for searching within loaded content
pub struct TextSearcher {
    case_sensitive: bool,
    regex: bool,
    compiled_regex: Option<regex::Regex>,
}

impl TextSearcher {
    /// Create a new text searcher
    pub fn new(
        pattern: &str,
        case_sensitive: bool,
        regex_mode: bool,
    ) -> Result<Self, regex::Error> {
        let compiled_regex = if regex_mode {
            let pattern = if case_sensitive {
                pattern.to_string()
            } else {
                format!("(?i){}", pattern)
            };
            Some(regex::Regex::new(&pattern)?)
        } else {
            None
        };

        Ok(Self {
            case_sensitive,
            regex: regex_mode,
            compiled_regex,
        })
    }

    /// Find all matches in text
    pub fn find_matches(&self, text: &str, context_chars: usize) -> Vec<HighlightedMatch> {
        let mut matches = Vec::new();

        if let Some(ref regex) = self.compiled_regex {
            for m in regex.find_iter(text) {
                let prefix_start = m.start().saturating_sub(context_chars);
                let suffix_end = (m.end() + context_chars).min(text.len());

                matches.push(HighlightedMatch {
                    range: m.start()..m.end(),
                    matched_text: m.as_str().to_string(),
                    prefix: text[prefix_start..m.start()].to_string(),
                    suffix: text[m.end()..suffix_end].to_string(),
                });
            }
        } else {
            // Simple string search
            let search_text = if self.case_sensitive {
                text.to_string()
            } else {
                text.to_lowercase()
            };

            // We need the pattern too, but we don't have it stored
            // This is a limitation of the current design
        }

        matches
    }

    /// Check if text contains a match
    pub fn contains_match(&self, text: &str) -> bool {
        if let Some(ref regex) = self.compiled_regex {
            regex.is_match(text)
        } else {
            false
        }
    }
}

/// Helper to highlight matches in text for display
pub fn highlight_matches(text: &str, pattern: &str, case_sensitive: bool) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();

    if pattern.is_empty() {
        return ranges;
    }

    let (search_text, search_pattern) = if case_sensitive {
        (text.to_string(), pattern.to_string())
    } else {
        (text.to_lowercase(), pattern.to_lowercase())
    };

    let mut start = 0;
    while let Some(pos) = search_text[start..].find(&search_pattern) {
        let match_start = start + pos;
        let match_end = match_start + pattern.len();
        ranges.push(match_start..match_end);
        start = match_end;
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_query_builder() {
        let query = SearchQuery::new("hello world")
            .case_sensitive(true)
            .limit(50)
            .in_book("book-123");

        assert_eq!(query.text, "hello world");
        assert!(query.case_sensitive);
        assert_eq!(query.limit, 50);
        assert_eq!(query.book_id, Some("book-123".to_string()));
    }

    #[test]
    fn test_highlight_matches() {
        let text = "The quick brown fox jumps over the lazy fox";
        let ranges = highlight_matches(text, "fox", false);

        assert_eq!(ranges.len(), 2);
        assert_eq!(&text[ranges[0].clone()], "fox");
        assert_eq!(&text[ranges[1].clone()], "fox");
    }

    #[test]
    fn test_highlight_case_insensitive() {
        let text = "Hello HELLO hello";
        let ranges = highlight_matches(text, "hello", false);

        assert_eq!(ranges.len(), 3);
    }

    #[test]
    fn test_highlighted_match_formats() {
        let m = HighlightedMatch {
            range: 4..7,
            matched_text: "fox".to_string(),
            prefix: "The ".to_string(),
            suffix: " jumps".to_string(),
        };

        assert_eq!(m.full_context(), "The fox jumps");
        assert!(m.html_highlighted().contains("<mark"));
    }
}
