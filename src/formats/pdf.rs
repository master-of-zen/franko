//! PDF format parser

use super::{Book, BookContent, BookMetadata, Chapter, ContentBlock, TocEntry};
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
                        let title_str = decode_pdf_string(&title_cow);
                        if !title_str.trim().is_empty() {
                            metadata.title = title_str;
                        }
                    }
                }

                // Author
                if let Ok(author) = info_dict.get(b"Author") {
                    if let Ok(author_cow) = author.as_string() {
                        let author_str = decode_pdf_string(&author_cow);
                        if !author_str.trim().is_empty() {
                            metadata.authors = vec![author_str];
                        }
                    }
                }

                // Subject/Description
                if let Ok(subject) = info_dict.get(b"Subject") {
                    if let Ok(subject_cow) = subject.as_string() {
                        let subject_str = decode_pdf_string(&subject_cow);
                        if !subject_str.trim().is_empty() {
                            metadata.description = Some(subject_str);
                        }
                    }
                }

                // Producer/Publisher
                if let Ok(producer) = info_dict.get(b"Producer") {
                    if let Ok(producer_cow) = producer.as_string() {
                        let producer_str = decode_pdf_string(&producer_cow);
                        if !producer_str.trim().is_empty() {
                            metadata.publisher = Some(producer_str);
                        }
                    }
                }

                // Creation date
                if let Ok(date) = info_dict.get(b"CreationDate") {
                    if let Ok(date_cow) = date.as_string() {
                        if let Some(parsed_date) = parse_pdf_date(&date_cow) {
                            metadata.published = Some(parsed_date);
                        }
                    }
                }

                // Keywords as subjects
                if let Ok(keywords) = info_dict.get(b"Keywords") {
                    if let Ok(keywords_cow) = keywords.as_string() {
                        let keywords_str = decode_pdf_string(&keywords_cow);
                        metadata.subjects = keywords_str
                            .split(&[',', ';'][..])
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
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

/// Decode PDF string, handling various encodings
fn decode_pdf_string(s: &str) -> String {
    // PDF strings can be in various encodings
    // This is a simplified decoder that handles common cases
    let result: String = s
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect();
    result.trim().to_string()
}

/// Parse PDF date format (D:YYYYMMDDHHmmSS) to human readable
fn parse_pdf_date(date: &str) -> Option<String> {
    let cleaned = date.trim_start_matches("D:");
    if cleaned.len() >= 4 {
        let year = &cleaned[0..4];
        if cleaned.len() >= 6 {
            let month = &cleaned[4..6];
            if cleaned.len() >= 8 {
                let day = &cleaned[6..8];
                return Some(format!("{}-{}-{}", year, month, day));
            }
            return Some(format!("{}-{}", year, month));
        }
        return Some(year.to_string());
    }
    None
}

fn extract_content(path: &Path) -> Result<BookContent> {
    // Try pdf-extract first
    match pdf_extract::extract_text(path) {
        Ok(text) if !text.trim().is_empty() => {
            let (chapters, toc) = parse_text_to_chapters(&text);
            Ok(BookContent { chapters, toc })
        }
        Ok(_) => {
            // Empty text - try lopdf directly
            extract_content_lopdf(path)
        }
        Err(e) => {
            // pdf-extract failed, try lopdf as fallback
            tracing::warn!("pdf-extract failed, trying lopdf fallback: {}", e);
            extract_content_lopdf(path)
        }
    }
}

/// Fallback extraction using lopdf directly
fn extract_content_lopdf(path: &Path) -> Result<BookContent> {
    let doc = lopdf::Document::load(path)
        .with_context(|| format!("Failed to load PDF: {}", path.display()))?;

    let page_count = doc.get_pages().len();
    let mut chapters = Vec::new();
    let mut toc = Vec::new();

    // Create chapters based on page groups (e.g., every 10 pages)
    let pages_per_chapter = if page_count <= 20 { page_count } else { 10 };
    let num_chapters = (page_count + pages_per_chapter - 1) / pages_per_chapter;

    for i in 0..num_chapters {
        let start_page = i * pages_per_chapter + 1;
        let end_page = ((i + 1) * pages_per_chapter).min(page_count);

        let chapter_id = format!("pages-{}-{}", start_page, end_page);
        let chapter_title = if num_chapters == 1 {
            "Document".to_string()
        } else {
            format!("Pages {}-{}", start_page, end_page)
        };

        let mut chapter = Chapter::new(chapter_id.clone(), i);
        chapter.title = Some(chapter_title.clone());

        // Add a placeholder block for each page group
        chapter.blocks.push(ContentBlock::Paragraph {
            text: format!(
                "[PDF content from pages {} to {}. Text extraction may be limited for this document.]",
                start_page, end_page
            ),
            styles: Vec::new(),
        });

        toc.push(TocEntry::new(chapter_title, chapter_id, 0));

        chapters.push(chapter);
    }

    // If we couldn't create any chapters, create a default one
    if chapters.is_empty() {
        let mut chapter = Chapter::new("main".to_string(), 0);
        chapter.title = Some("Document".to_string());
        chapter.blocks.push(ContentBlock::Paragraph {
            text: "[PDF text extraction failed. The document may be scanned or have complex formatting.]".to_string(),
            styles: Vec::new(),
        });
        chapters.push(chapter);
    }

    Ok(BookContent { chapters, toc })
}

/// Parse extracted text into chapters with smart detection
fn parse_text_to_chapters(text: &str) -> (Vec<Chapter>, Vec<TocEntry>) {
    let mut chapters = Vec::new();
    let mut toc = Vec::new();

    // Detect chapter breaks by looking for patterns like:
    // - "Chapter X" or "CHAPTER X"
    // - "Part X" or "PART X"
    // - Roman numerals at the start of a line
    // - Page break indicators
    let chapter_regex = regex::Regex::new(
        r"(?mi)^(?:chapter|part|section)\s+(?:\d+|[ivxlcdm]+)|^[ivxlcdm]+\.\s+\w|^(?:\d+\.)\s*[A-Z]"
    ).unwrap();

    // Split text into potential chapters
    let mut chapter_starts: Vec<usize> = vec![0];
    for mat in chapter_regex.find_iter(text) {
        // Look for the start of the line containing this match
        let line_start = text[..mat.start()].rfind('\n').map(|i| i + 1).unwrap_or(0);

        // Only add if there's significant content before this
        if line_start > 50 && !chapter_starts.contains(&line_start) {
            chapter_starts.push(line_start);
        }
    }

    // If no chapter markers found, split by page-like breaks or make single chapter
    if chapter_starts.len() == 1 {
        // Look for form feed characters or multiple blank lines
        let page_break_regex = regex::Regex::new(r"\f|\n{4,}").unwrap();
        let mut break_positions: Vec<usize> = vec![0];
        for mat in page_break_regex.find_iter(text) {
            break_positions.push(mat.end());
        }

        // Only use page breaks if we found a reasonable number
        if break_positions.len() > 1 && break_positions.len() < 100 {
            chapter_starts = break_positions;
        }
    }

    chapter_starts.push(text.len());

    for (i, window) in chapter_starts.windows(2).enumerate() {
        let start = window[0];
        let end = window[1];
        let chunk = &text[start..end];

        if chunk.trim().is_empty() {
            continue;
        }

        let blocks = parse_text_content(chunk);
        if blocks.is_empty() {
            continue;
        }

        // Try to extract chapter title from first heading or first line
        let title = blocks
            .iter()
            .find_map(|b| match b {
                ContentBlock::Heading { text, .. } => Some(text.clone()),
                _ => None,
            })
            .or_else(|| {
                // Use first non-empty paragraph's first sentence
                blocks.iter().find_map(|b| match b {
                    ContentBlock::Paragraph { text, .. } if !text.is_empty() => {
                        let first_line = text.lines().next().unwrap_or("");
                        if first_line.len() < 100 {
                            Some(first_line.to_string())
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
            });

        let chapter_id = format!("chapter-{}", i);
        let display_title = title
            .clone()
            .unwrap_or_else(|| format!("Section {}", i + 1));

        let mut chapter = Chapter::new(chapter_id.clone(), i);
        chapter.title = Some(display_title.clone());
        chapter.blocks = blocks;

        toc.push(TocEntry::new(display_title, chapter_id, 0));
        chapters.push(chapter);
    }

    // Fallback to single chapter if nothing was created
    if chapters.is_empty() {
        let blocks = parse_text_content(text);
        let mut chapter = Chapter::new("main".to_string(), 0);
        chapter.title = Some("Document".to_string());
        chapter.blocks = if blocks.is_empty() {
            vec![ContentBlock::Paragraph {
                text: text.trim().to_string(),
                styles: Vec::new(),
            }]
        } else {
            blocks
        };
        chapters.push(chapter);
    }

    (chapters, toc)
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

        // Skip common PDF artifacts
        if is_pdf_artifact(trimmed) {
            continue;
        }

        // Detect potential headings
        let is_likely_heading = is_heading_like(trimmed);

        if is_likely_heading {
            blocks.push(ContentBlock::Heading {
                level: detect_heading_level(trimmed),
                text: trimmed.to_string(),
            });
        } else {
            // Normalize whitespace within paragraphs
            let normalized: String = trimmed
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            if !normalized.is_empty() {
                blocks.push(ContentBlock::Paragraph {
                    text: normalized,
                    styles: Vec::new(),
                });
            }
        }
    }

    blocks
}

/// Check if text looks like a heading
fn is_heading_like(text: &str) -> bool {
    let trimmed = text.trim();

    // Too long for a heading
    if trimmed.len() > 120 {
        return false;
    }

    // Single line
    if trimmed.lines().count() > 2 {
        return false;
    }

    // Ends with punctuation typical of prose
    if trimmed.ends_with('.') || trimmed.ends_with(',') {
        return false;
    }

    // Check for chapter/section markers
    let lower = trimmed.to_lowercase();
    if lower.starts_with("chapter")
        || lower.starts_with("part")
        || lower.starts_with("section")
        || lower.starts_with("introduction")
        || lower.starts_with("conclusion")
        || lower.starts_with("appendix")
        || lower.starts_with("preface")
        || lower.starts_with("prologue")
        || lower.starts_with("epilogue")
    {
        return true;
    }

    // High ratio of uppercase letters
    let alpha_chars: Vec<char> = trimmed.chars().filter(|c| c.is_alphabetic()).collect();
    if !alpha_chars.is_empty() {
        let uppercase_ratio = alpha_chars.iter().filter(|c| c.is_uppercase()).count() as f32
            / alpha_chars.len() as f32;
        if uppercase_ratio > 0.6 && trimmed.len() < 80 {
            return true;
        }
    }

    // Short line without ending punctuation
    if trimmed.len() < 60 && !trimmed.ends_with('.') && !trimmed.ends_with(',') {
        // Check if it looks like a title (capitalized words)
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        if words.len() <= 10 {
            let capitalized = words
                .iter()
                .filter(|w| w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
                .count();
            if capitalized as f32 / words.len() as f32 > 0.5 {
                return true;
            }
        }
    }

    false
}

/// Detect heading level based on text characteristics
fn detect_heading_level(text: &str) -> u8 {
    let lower = text.to_lowercase();

    // Major divisions
    if lower.starts_with("part") || lower.starts_with("book") {
        return 1;
    }

    // Chapters
    if lower.starts_with("chapter") {
        return 2;
    }

    // Sections
    if lower.starts_with("section") {
        return 3;
    }

    // All caps = likely major heading
    if text
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase())
    {
        return 2;
    }

    // Default
    3
}

/// Check if text is a common PDF artifact to skip
fn is_pdf_artifact(text: &str) -> bool {
    let trimmed = text.trim();

    // Page numbers only
    if trimmed.parse::<u32>().is_ok() {
        return true;
    }

    // Common headers/footers
    if trimmed.len() < 50 {
        let lower = trimmed.to_lowercase();
        if lower.contains("page ") && lower.chars().filter(|c| c.is_numeric()).count() > 0 {
            return true;
        }
        if lower == "confidential" || lower == "draft" || lower.starts_with("©") {
            return true;
        }
    }

    // Repeated characters (often PDF rendering artifacts)
    if trimmed.len() > 3 {
        let chars: Vec<char> = trimmed.chars().collect();
        if chars.windows(3).all(|w| w[0] == w[1] && w[1] == w[2]) {
            return true;
        }
    }

    false
}
