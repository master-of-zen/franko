//! Search index implementation using tantivy

use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::RwLock;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, FAST, STORED, STRING, TEXT};
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument};

use crate::error::{FrankoError, Result};
use crate::formats::Book;

/// Entry in the search index
#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub book_id: String,
    pub chapter_index: usize,
    pub block_index: usize,
    pub content: String,
    pub chapter_title: String,
}

/// Search index for full-text book searching
pub struct SearchIndex {
    index: Index,
    schema: Schema,
    reader: IndexReader,
    writer: Arc<RwLock<IndexWriter>>,

    // Field handles
    book_id_field: Field,
    chapter_index_field: Field,
    block_index_field: Field,
    content_field: Field,
    chapter_title_field: Field,
}

impl SearchIndex {
    /// Create a new search index at the given path
    pub fn new(index_path: Option<PathBuf>) -> Result<Self> {
        let mut schema_builder = Schema::builder();

        let book_id_field = schema_builder.add_text_field("book_id", STRING | STORED);
        let chapter_index_field = schema_builder.add_u64_field("chapter_index", STORED | FAST);
        let block_index_field = schema_builder.add_u64_field("block_index", STORED | FAST);
        let content_field = schema_builder.add_text_field("content", TEXT | STORED);
        let chapter_title_field = schema_builder.add_text_field("chapter_title", TEXT | STORED);

        let schema = schema_builder.build();

        let index = if let Some(path) = index_path {
            std::fs::create_dir_all(&path).map_err(FrankoError::Io)?;
            Index::create_in_dir(&path, schema.clone())
                .or_else(|_| Index::open_in_dir(&path))
                .map_err(|e| FrankoError::Search(format!("Failed to open index: {}", e)))?
        } else {
            Index::create_in_ram(schema.clone())
        };

        let writer = index
            .writer(50_000_000)
            .map_err(|e| FrankoError::Search(format!("Failed to create writer: {}", e)))?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .map_err(|e| FrankoError::Search(format!("Failed to create reader: {}", e)))?;

        Ok(Self {
            index,
            schema,
            reader,
            writer: Arc::new(RwLock::new(writer)),
            book_id_field,
            chapter_index_field,
            block_index_field,
            content_field,
            chapter_title_field,
        })
    }

    /// Create an in-memory search index
    pub fn in_memory() -> Result<Self> {
        Self::new(None)
    }

    /// Index a single book
    pub fn index_book(&self, book: &Book) -> Result<()> {
        let mut writer = self.writer.write();

        for (chapter_idx, chapter) in book.content.chapters.iter().enumerate() {
            for (block_idx, block) in chapter.blocks.iter().enumerate() {
                let content = block.text();
                if content.trim().is_empty() {
                    continue;
                }

                let chapter_title = chapter
                    .title
                    .clone()
                    .unwrap_or_else(|| format!("Chapter {}", chapter_idx + 1));

                writer.add_document(doc!(
                    self.book_id_field => book.metadata.title.clone(),
                    self.chapter_index_field => chapter_idx as u64,
                    self.block_index_field => block_idx as u64,
                    self.content_field => content,
                    self.chapter_title_field => chapter_title,
                ))?;
            }
        }

        writer
            .commit()
            .map_err(|e| FrankoError::Search(format!("Failed to commit: {}", e)))?;

        Ok(())
    }

    /// Remove all entries for a book
    pub fn remove_book(&self, book_id: &str) -> Result<()> {
        let mut writer = self.writer.write();

        let term = tantivy::Term::from_field_text(self.book_id_field, book_id);
        writer.delete_term(term);

        writer
            .commit()
            .map_err(|e| FrankoError::Search(format!("Failed to commit: {}", e)))?;

        Ok(())
    }

    /// Search for text across all indexed books
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<IndexEntry>> {
        self.reader
            .reload()
            .map_err(|e| FrankoError::Search(format!("Failed to reload: {}", e)))?;

        let searcher = self.reader.searcher();

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.content_field, self.chapter_title_field],
        );

        let query = query_parser
            .parse_query(query)
            .map_err(|e| FrankoError::Search(format!("Failed to parse query: {}", e)))?;

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .map_err(|e| FrankoError::Search(format!("Failed to search: {}", e)))?;

        let mut results = Vec::new();

        for (_score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher
                .doc(doc_address)
                .map_err(|e| FrankoError::Search(format!("Failed to retrieve doc: {}", e)))?;

            let book_id = doc
                .get_first(self.book_id_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            let chapter_index = doc
                .get_first(self.chapter_index_field)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let block_index = doc
                .get_first(self.block_index_field)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let content = doc
                .get_first(self.content_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            let chapter_title = doc
                .get_first(self.chapter_title_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            results.push(IndexEntry {
                book_id,
                chapter_index,
                block_index,
                content,
                chapter_title,
            });
        }

        Ok(results)
    }

    /// Search within a specific book
    pub fn search_in_book(
        &self,
        book_id: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<IndexEntry>> {
        self.reader
            .reload()
            .map_err(|e| FrankoError::Search(format!("Failed to reload: {}", e)))?;

        let searcher = self.reader.searcher();

        // Build a query that filters by book_id and searches content
        let full_query = format!("+book_id:\"{}\" AND ({})", book_id, query);

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.book_id_field,
                self.content_field,
                self.chapter_title_field,
            ],
        );

        let parsed_query = query_parser
            .parse_query(&full_query)
            .map_err(|e| FrankoError::Search(format!("Failed to parse query: {}", e)))?;

        let top_docs = searcher
            .search(&parsed_query, &TopDocs::with_limit(limit))
            .map_err(|e| FrankoError::Search(format!("Failed to search: {}", e)))?;

        let mut results = Vec::new();

        for (_score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher
                .doc(doc_address)
                .map_err(|e| FrankoError::Search(format!("Failed to retrieve doc: {}", e)))?;

            let book_id = doc
                .get_first(self.book_id_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            let chapter_index = doc
                .get_first(self.chapter_index_field)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let block_index = doc
                .get_first(self.block_index_field)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let content = doc
                .get_first(self.content_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            let chapter_title = doc
                .get_first(self.chapter_title_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            results.push(IndexEntry {
                book_id,
                chapter_index,
                block_index,
                content,
                chapter_title,
            });
        }

        Ok(results)
    }

    /// Get the number of indexed documents
    pub fn document_count(&self) -> Result<u64> {
        self.reader
            .reload()
            .map_err(|e| FrankoError::Search(format!("Failed to reload: {}", e)))?;

        let searcher = self.reader.searcher();
        Ok(searcher.num_docs())
    }

    /// Clear the entire index
    pub fn clear(&self) -> Result<()> {
        let mut writer = self.writer.write();
        writer
            .delete_all_documents()
            .map_err(|e| FrankoError::Search(format!("Failed to clear: {}", e)))?;
        writer
            .commit()
            .map_err(|e| FrankoError::Search(format!("Failed to commit: {}", e)))?;
        Ok(())
    }
}

impl std::fmt::Debug for SearchIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchIndex")
            .field("schema", &self.schema)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::{BookMetadata, Chapter, ContentBlock};

    fn create_test_book() -> Book {
        Book {
            id: "test-book-1".to_string(),
            metadata: BookMetadata {
                title: "Test Book".to_string(),
                authors: vec!["Test Author".to_string()],
                ..Default::default()
            },
            chapters: vec![
                Chapter {
                    title: "Introduction".to_string(),
                    content: vec![ContentBlock::Paragraph {
                        text: "This is the introduction to the test book.".to_string(),
                        style: Default::default(),
                    }],
                    ..Default::default()
                },
                Chapter {
                    title: "Chapter One".to_string(),
                    content: vec![
                        ContentBlock::Paragraph {
                            text: "The quick brown fox jumps over the lazy dog.".to_string(),
                            style: Default::default(),
                        },
                        ContentBlock::Paragraph {
                            text: "Rust is a systems programming language.".to_string(),
                            style: Default::default(),
                        },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn test_index_and_search() {
        let index = SearchIndex::in_memory().unwrap();
        let book = create_test_book();

        index.index_book(&book).unwrap();

        let results = index.search("quick brown fox", 10).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].book_id, "test-book-1");
        assert_eq!(results[0].chapter_index, 1);
    }

    #[test]
    fn test_search_in_book() {
        let index = SearchIndex::in_memory().unwrap();
        let book = create_test_book();

        index.index_book(&book).unwrap();

        let results = index
            .search_in_book("test-book-1", "Rust programming", 10)
            .unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_remove_book() {
        let index = SearchIndex::in_memory().unwrap();
        let book = create_test_book();

        index.index_book(&book).unwrap();
        assert!(index.document_count().unwrap() > 0);

        index.remove_book("test-book-1").unwrap();
        // After removal, searching should return no results
        let results = index.search_in_book("test-book-1", "fox", 10).unwrap();
        assert!(results.is_empty());
    }
}
