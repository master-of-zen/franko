//! Book format parsers
//!
//! Supports multiple book formats with a unified interface

mod book;

#[cfg(feature = "epub")]
mod epub;

#[cfg(feature = "pdf")]
mod pdf;

#[cfg(feature = "markdown")]
mod markdown;

#[cfg(feature = "txt")]
mod txt;

pub use book::{Book, BookContent, BookMetadata, Chapter, ContentBlock, TextStyle, TocEntry, StyleType};

use anyhow::{Context, Result};
use std::path::Path;

/// Supported book formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookFormat {
    Epub,
    Pdf,
    Markdown,
    PlainText,
    Html,
    Unknown,
}

impl BookFormat {
    /// Detect format from file extension
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()).as_deref() {
            Some("epub") => BookFormat::Epub,
            Some("pdf") => BookFormat::Pdf,
            Some("md") | Some("markdown") => BookFormat::Markdown,
            Some("txt") | Some("text") => BookFormat::PlainText,
            Some("html") | Some("htm") | Some("xhtml") => BookFormat::Html,
            _ => BookFormat::Unknown,
        }
    }

    /// Get the format name
    pub fn name(&self) -> &'static str {
        match self {
            BookFormat::Epub => "EPUB",
            BookFormat::Pdf => "PDF",
            BookFormat::Markdown => "Markdown",
            BookFormat::PlainText => "Plain Text",
            BookFormat::Html => "HTML",
            BookFormat::Unknown => "Unknown",
        }
    }

    /// Check if format is supported
    pub fn is_supported(&self) -> bool {
        match self {
            #[cfg(feature = "epub")]
            BookFormat::Epub => true,
            #[cfg(not(feature = "epub"))]
            BookFormat::Epub => false,

            #[cfg(feature = "pdf")]
            BookFormat::Pdf => true,
            #[cfg(not(feature = "pdf"))]
            BookFormat::Pdf => false,

            #[cfg(feature = "markdown")]
            BookFormat::Markdown => true,
            #[cfg(not(feature = "markdown"))]
            BookFormat::Markdown => false,

            BookFormat::PlainText => true,
            BookFormat::Html => true,
            BookFormat::Unknown => false,
        }
    }
}

/// Parse a book from a file path
pub fn parse_book(path: &Path) -> Result<Book> {
    let format = BookFormat::from_path(path);

    if !format.is_supported() {
        anyhow::bail!("Unsupported format: {} ({})", format.name(), path.display());
    }

    match format {
        #[cfg(feature = "epub")]
        BookFormat::Epub => epub::parse(path),

        #[cfg(feature = "pdf")]
        BookFormat::Pdf => pdf::parse(path),

        #[cfg(feature = "markdown")]
        BookFormat::Markdown => markdown::parse(path),

        BookFormat::PlainText | BookFormat::Html => txt::parse(path),

        _ => anyhow::bail!("Format not available: {}", format.name()),
    }
    .with_context(|| format!("Failed to parse book: {}", path.display()))
}

/// Get metadata without parsing full content
pub fn get_metadata(path: &Path) -> Result<BookMetadata> {
    let format = BookFormat::from_path(path);

    match format {
        #[cfg(feature = "epub")]
        BookFormat::Epub => epub::metadata(path),

        #[cfg(feature = "pdf")]
        BookFormat::Pdf => pdf::metadata(path),

        _ => {
            // For other formats, do a quick parse
            let book = parse_book(path)?;
            Ok(book.metadata)
        }
    }
}
