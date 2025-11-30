//! PDF format parser

use super::{Book, BookContent, BookMetadata, Chapter, ContentBlock};
use anyhow::{Context, Result};
use std::path::Path;

/// Parse a PDF file
pub fn parse(path: &Path) -> Result<Book> {
    let metadata = metadata(path)?;
    let content = extract_content(path)?;

    Ok(Book {
        metadata,
        content,
        source_path: path.to_path_buf(),
        format: "pdf".to_string(),
    })
}

/// Extract metadata from PDF
pub fn metadata(path: &Path) -> Result<BookMetadata> {
    let doc = lopdf::Document::load(path)
        .with_context(|| format!("Failed to load PDF: {}", path.display()))?;

    let mut metadata = BookMetadata::default();

    // Try to get metadata from document info dictionary
    if let Ok(info) = doc.trailer.get(b"Info") {
        if let Ok(info_ref) = info.as_reference() {
            if let Ok(info_dict) = doc.get_dictionary(info_ref) {
                // Title
                if let Ok(title) = info_dict.get(b"Title") {
                    if let Ok(title_cow) = title.as_string() {
                        metadata.title = title_cow.into_owned();
                    }
                }

                // Author
                if let Ok(author) = info_dict.get(b"Author") {
                    if let Ok(author_cow) = author.as_string() {
                        metadata.authors = vec![author_cow.into_owned()];
                    }
                }

                // Subject/Description
                if let Ok(subject) = info_dict.get(b"Subject") {
                    if let Ok(subject_cow) = subject.as_string() {
                        metadata.description = Some(subject_cow.into_owned());
                    }
                }

                // Producer
                if let Ok(producer) = info_dict.get(b"Producer") {
                    if let Ok(producer_cow) = producer.as_string() {
                        metadata.publisher = Some(producer_cow.into_owned());
                    }
                }
            }
        }
    }

    // Use filename as title if no title found
    if metadata.title.is_empty() {
        metadata.title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();
    }

    Ok(metadata)
}

fn extract_content(path: &Path) -> Result<BookContent> {
    // Use pdf-extract for text extraction
    let text = pdf_extract::extract_text(path)
        .with_context(|| format!("Failed to extract text from PDF: {}", path.display()))?;

    let mut chapters = Vec::new();
    let blocks = parse_text_content(&text);

    // For PDFs, we treat the whole document as one chapter initially
    // TODO: Implement page-based chapter detection
    let mut chapter = Chapter::new("main".to_string(), 0);
    chapter.title = Some("Document".to_string());
    chapter.blocks = blocks;
    chapters.push(chapter);

    Ok(BookContent {
        chapters,
        toc: Vec::new(),
    })
}

fn parse_text_content(text: &str) -> Vec<ContentBlock> {
    let mut blocks = Vec::new();

    // Split by double newlines (paragraphs)
    for para in text.split("\n\n") {
        let trimmed = para.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Skip very short lines that are likely page numbers or headers
        if trimmed.len() < 5 && trimmed.chars().all(|c| c.is_numeric() || c.is_whitespace()) {
            continue;
        }

        // Detect potential headings (short lines, often uppercase)
        let is_likely_heading = trimmed.len() < 80 
            && !trimmed.ends_with('.')
            && (trimmed.chars().filter(|c| c.is_uppercase()).count() as f32 
                / trimmed.chars().filter(|c| c.is_alphabetic()).count().max(1) as f32 > 0.7);

        if is_likely_heading {
            blocks.push(ContentBlock::Heading {
                level: 2,
                text: trimmed.to_string(),
            });
        } else {
            // Normalize whitespace within paragraphs
            let normalized: String = trimmed
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            blocks.push(ContentBlock::Paragraph {
                text: normalized,
                styles: Vec::new(),
            });
        }
    }

    blocks
}
