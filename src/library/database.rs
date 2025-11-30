//! Library database and data structures

use crate::config::Config;
use crate::formats::{self, BookMetadata};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::info;

/// A library entry representing a book in the collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryEntry {
    /// Unique identifier
    pub id: String,

    /// Path to the book file
    pub path: PathBuf,

    /// Book format (epub, pdf, etc.)
    pub format: String,

    /// Book metadata
    pub metadata: BookMetadata,

    /// User-assigned tags
    pub tags: Vec<String>,

    /// Reading progress (0.0 - 1.0)
    pub progress: f64,

    /// Current position - chapter index
    pub position_chapter: usize,

    /// Current position - block index
    pub position_block: usize,

    /// Current position - scroll offset
    pub position_offset: usize,

    /// Reading status
    pub status: ReadingStatus,

    /// Date added to library
    pub added_at: DateTime<Utc>,

    /// Last read date
    pub last_read: Option<DateTime<Utc>>,

    /// Total reading time in seconds
    pub reading_time: u64,

    /// Cover image path (if extracted)
    pub cover_path: Option<PathBuf>,

    /// Bookmarks
    pub bookmarks: Vec<Bookmark>,

    /// Annotations
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReadingStatus {
    Unread,
    Reading,
    Finished,
    Abandoned,
}

impl Default for ReadingStatus {
    fn default() -> Self {
        ReadingStatus::Unread
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub name: String,
    pub chapter: usize,
    pub block: usize,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub text: String,
    pub note: Option<String>,
    pub chapter: usize,
    pub block: usize,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

/// The library database
#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    /// Library entries indexed by ID
    books: HashMap<String, LibraryEntry>,

    /// Path to the database file
    #[serde(skip)]
    db_path: PathBuf,

    /// Configuration reference
    #[serde(skip)]
    config: Config,
}

impl Library {
    /// Create a new library or load existing one
    pub fn new(config: &Config) -> Result<Self> {
        let db_path = config.database_path()?;

        if db_path.exists() {
            Self::load(&db_path, config.clone())
        } else {
            Ok(Self {
                books: HashMap::new(),
                db_path,
                config: config.clone(),
            })
        }
    }

    /// Load library from file
    fn load(path: &Path, config: Config) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read library file: {}", path.display()))?;

        let mut library: Library = serde_json::from_str(&content)
            .with_context(|| "Failed to parse library file")?;

        library.db_path = path.to_path_buf();
        library.config = config;

        info!("Loaded library with {} books", library.books.len());
        Ok(library)
    }

    /// Save library to file
    pub fn save(&self) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = self.db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&self)?;
        std::fs::write(&self.db_path, content)?;

        info!("Saved library to {}", self.db_path.display());
        Ok(())
    }

    /// Get all books
    pub fn books(&self) -> Vec<LibraryEntry> {
        self.books.values().cloned().collect()
    }

    /// Get a book by ID
    pub fn get_book(&self, id: &str) -> Option<LibraryEntry> {
        self.books.get(id).cloned()
    }

    /// Add a book to the library
    pub fn add_book(&mut self, path: &Path, tags: Option<Vec<String>>) -> Result<LibraryEntry> {
        // Check if already exists
        for entry in self.books.values() {
            if entry.path == path {
                anyhow::bail!("Book already in library: {}", path.display());
            }
        }

        // Parse metadata
        let metadata = formats::get_metadata(path)?;

        // Generate ID
        let id = generate_id(&metadata.title);

        // Detect format
        let format = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_else(|| "unknown".to_string());

        let entry = LibraryEntry {
            id: id.clone(),
            path: path.to_path_buf(),
            format,
            metadata,
            tags: tags.unwrap_or_default(),
            progress: 0.0,
            position_chapter: 0,
            position_block: 0,
            position_offset: 0,
            status: ReadingStatus::Unread,
            added_at: Utc::now(),
            last_read: None,
            reading_time: 0,
            cover_path: None,
            bookmarks: Vec::new(),
            annotations: Vec::new(),
        };

        self.books.insert(id, entry.clone());
        Ok(entry)
    }

    /// Remove a book from the library
    pub fn remove_book(&mut self, id: &str) -> Result<()> {
        if self.books.remove(id).is_some() {
            Ok(())
        } else {
            anyhow::bail!("Book not found: {}", id)
        }
    }

    /// List books with optional filters
    pub fn list_books(
        &self,
        format: Option<&str>,
        tag: Option<&str>,
        status: Option<crate::cli::ReadingStatus>,
    ) -> Result<Vec<LibraryEntry>> {
        let books: Vec<LibraryEntry> = self
            .books
            .values()
            .filter(|b| {
                if let Some(fmt) = format {
                    if b.format != fmt {
                        return false;
                    }
                }
                if let Some(t) = tag {
                    if !b.tags.contains(&t.to_string()) {
                        return false;
                    }
                }
                if let Some(s) = status {
                    let matches = match s {
                        crate::cli::ReadingStatus::Unread => b.status == ReadingStatus::Unread,
                        crate::cli::ReadingStatus::Reading => b.status == ReadingStatus::Reading,
                        crate::cli::ReadingStatus::Finished => b.status == ReadingStatus::Finished,
                        crate::cli::ReadingStatus::Abandoned => b.status == ReadingStatus::Abandoned,
                    };
                    if !matches {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        Ok(books)
    }

    /// Search books by query
    pub fn search(&self, query: &str) -> Vec<LibraryEntry> {
        let query_lower = query.to_lowercase();

        self.books
            .values()
            .filter(|b| {
                b.metadata.title.to_lowercase().contains(&query_lower)
                    || b.metadata
                        .authors
                        .iter()
                        .any(|a| a.to_lowercase().contains(&query_lower))
                    || b.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
                    || b.metadata
                        .subjects
                        .iter()
                        .any(|s| s.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }

    /// Import books from a directory
    pub fn import_directory(&mut self, path: &Path, recursive: bool) -> Result<usize> {
        let mut count = 0;

        let entries: Vec<_> = if recursive {
            walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .map(|e| e.path().to_path_buf())
                .collect()
        } else {
            std::fs::read_dir(path)?
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
                .map(|e| e.path())
                .collect()
        };

        for entry_path in entries {
            let format = formats::BookFormat::from_path(&entry_path);
            if format.is_supported() {
                match self.add_book(&entry_path, None) {
                    Ok(_) => count += 1,
                    Err(e) => {
                        tracing::warn!("Failed to import {}: {}", entry_path.display(), e);
                    }
                }
            }
        }

        Ok(count)
    }

    /// Export library data
    pub fn export(&self, path: &Path, format: &str) -> Result<()> {
        let content = match format {
            "json" => serde_json::to_string_pretty(&self.books)?,
            "csv" => {
                let mut csv = String::from("id,title,author,format,progress,status\n");
                for book in self.books.values() {
                    csv.push_str(&format!(
                        "{},{},{},{},{:.1},{:?}\n",
                        book.id,
                        escape_csv(&book.metadata.title),
                        escape_csv(&book.metadata.authors_string()),
                        book.format,
                        book.progress * 100.0,
                        book.status
                    ));
                }
                csv
            }
            _ => anyhow::bail!("Unsupported export format: {}", format),
        };

        std::fs::write(path, content)?;
        Ok(())
    }

    /// Update reading progress
    pub fn update_progress(
        &mut self,
        id: &str,
        chapter: usize,
        block: usize,
        offset: usize,
    ) -> Result<()> {
        if let Some(entry) = self.books.get_mut(id) {
            entry.position_chapter = chapter;
            entry.position_block = block;
            entry.position_offset = offset;
            entry.last_read = Some(Utc::now());

            // Update status if needed
            if entry.status == ReadingStatus::Unread {
                entry.status = ReadingStatus::Reading;
            }

            Ok(())
        } else {
            anyhow::bail!("Book not found: {}", id)
        }
    }

    /// Get bookmarks for a book
    pub fn get_bookmarks(&self, book_id: &str) -> Result<Vec<Bookmark>> {
        self.books
            .get(book_id)
            .map(|b| b.bookmarks.clone())
            .ok_or_else(|| anyhow::anyhow!("Book not found: {}", book_id))
    }

    /// Add a bookmark
    pub fn add_bookmark(
        &mut self,
        book_id: &str,
        name: Option<String>,
        chapter: usize,
        block: usize,
    ) -> Result<Bookmark> {
        if let Some(entry) = self.books.get_mut(book_id) {
            let id = uuid::Uuid::new_v4().to_string();
            let name = name.unwrap_or_else(|| format!("Bookmark at Ch.{}", chapter + 1));

            let bookmark = Bookmark {
                id: id.clone(),
                name,
                chapter,
                block,
                created_at: Utc::now(),
            };

            entry.bookmarks.push(bookmark.clone());
            Ok(bookmark)
        } else {
            anyhow::bail!("Book not found: {}", book_id)
        }
    }

    /// Remove a bookmark
    pub fn remove_bookmark(&mut self, book_id: &str, bookmark_id: &str) -> Result<()> {
        if let Some(entry) = self.books.get_mut(book_id) {
            let len_before = entry.bookmarks.len();
            entry.bookmarks.retain(|b| b.id != bookmark_id);
            
            if entry.bookmarks.len() < len_before {
                Ok(())
            } else {
                anyhow::bail!("Bookmark not found: {}", bookmark_id)
            }
        } else {
            anyhow::bail!("Book not found: {}", book_id)
        }
    }

    /// Get annotations for a book
    pub fn get_annotations(&self, book_id: &str) -> Result<Vec<Annotation>> {
        self.books
            .get(book_id)
            .map(|b| b.annotations.clone())
            .ok_or_else(|| anyhow::anyhow!("Book not found: {}", book_id))
    }

    /// Add an annotation
    pub fn add_annotation(
        &mut self,
        book_id: &str,
        text: String,
        note: Option<String>,
        chapter: usize,
        block: usize,
        color: Option<String>,
    ) -> Result<Annotation> {
        if let Some(entry) = self.books.get_mut(book_id) {
            let id = uuid::Uuid::new_v4().to_string();

            let annotation = Annotation {
                id: id.clone(),
                text,
                note,
                chapter,
                block,
                color: color.unwrap_or_else(|| "yellow".to_string()),
                created_at: Utc::now(),
            };

            entry.annotations.push(annotation.clone());
            Ok(annotation)
        } else {
            anyhow::bail!("Book not found: {}", book_id)
        }
    }

    /// Remove an annotation
    pub fn remove_annotation(&mut self, book_id: &str, annotation_id: &str) -> Result<()> {
        if let Some(entry) = self.books.get_mut(book_id) {
            let len_before = entry.annotations.len();
            entry.annotations.retain(|a| a.id != annotation_id);
            
            if entry.annotations.len() < len_before {
                Ok(())
            } else {
                anyhow::bail!("Annotation not found: {}", annotation_id)
            }
        } else {
            anyhow::bail!("Book not found: {}", book_id)
        }
    }
}

/// Generate a unique ID from a title
fn generate_id(title: &str) -> String {
    let base: String = title
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
        .split_whitespace()
        .take(4)
        .collect::<Vec<_>>()
        .join("-");

    let suffix = &uuid::Uuid::new_v4().to_string()[..8];
    
    if base.is_empty() {
        suffix.to_string()
    } else {
        format!("{}-{}", base, suffix)
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
