//! Plain text and HTML format parser

use super::{Book, BookContent, BookMetadata, Chapter, ContentBlock};
use anyhow::{Context, Result};
use std::path::Path;

/// Parse a plain text or HTML file
pub fn parse(path: &Path) -> Result<Book> {
    let content_str = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    let is_html = matches!(
        extension.as_deref(),
        Some("html") | Some("htm") | Some("xhtml")
    );

    let content = if is_html {
        parse_html_content(&content_str)
    } else {
        parse_text_content(&content_str)
    };

    let title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let metadata = BookMetadata {
        title,
        ..Default::default()
    };

    Ok(Book {
        metadata,
        content,
        source_path: path.to_path_buf(),
        format: if is_html { "html" } else { "txt" }.to_string(),
    })
}

fn parse_text_content(text: &str) -> BookContent {
    let mut blocks = Vec::new();
    let mut current_paragraph = String::new();

    for line in text.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            // End of paragraph
            if !current_paragraph.is_empty() {
                blocks.push(ContentBlock::Paragraph {
                    text: std::mem::take(&mut current_paragraph),
                    styles: Vec::new(),
                });
            }
        } else {
            // Check for potential chapter/section headers
            let is_header = is_likely_header(trimmed);

            if is_header {
                // Flush current paragraph
                if !current_paragraph.is_empty() {
                    blocks.push(ContentBlock::Paragraph {
                        text: std::mem::take(&mut current_paragraph),
                        styles: Vec::new(),
                    });
                }
                blocks.push(ContentBlock::Heading {
                    level: 2,
                    text: trimmed.to_string(),
                });
            } else {
                // Add to current paragraph
                if !current_paragraph.is_empty() {
                    current_paragraph.push(' ');
                }
                current_paragraph.push_str(trimmed);
            }
        }
    }

    // Flush remaining paragraph
    if !current_paragraph.is_empty() {
        blocks.push(ContentBlock::Paragraph {
            text: current_paragraph,
            styles: Vec::new(),
        });
    }

    // Create single chapter
    let mut chapter = Chapter::new("main".to_string(), 0);
    chapter.title = Some("Document".to_string());
    chapter.blocks = blocks;

    BookContent {
        chapters: vec![chapter],
        toc: Vec::new(),
    }
}

fn parse_html_content(html: &str) -> BookContent {
    // Use html2text for conversion
    let text = html2text::from_read(html.as_bytes(), 80);
    parse_text_content(&text)
}

/// Heuristic to detect potential headers in plain text
fn is_likely_header(line: &str) -> bool {
    // Skip very long lines
    if line.len() > 80 {
        return false;
    }

    // Skip lines ending with common punctuation
    if line.ends_with('.') || line.ends_with(',') || line.ends_with(':') {
        return false;
    }

    // Check for chapter patterns
    let lower = line.to_lowercase();
    if lower.starts_with("chapter ") || lower.starts_with("part ") || lower.starts_with("section ")
    {
        return true;
    }

    // Check for roman numerals at start
    let words: Vec<&str> = line.split_whitespace().collect();
    if let Some(first) = words.first() {
        if is_roman_numeral(first) {
            return true;
        }
    }

    // Check for numbered chapters: "1.", "1:", "I.", etc.
    if let Some(first) = words.first() {
        let first = first.trim_end_matches(&['.', ':', '-'][..]);
        if first.chars().all(|c| c.is_numeric()) || is_roman_numeral(first) {
            return words.len() > 1 && words.len() <= 10;
        }
    }

    // Check for all caps (but not single words)
    if line.len() > 3
        && line
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
        && line.contains(' ')
    {
        return true;
    }

    // Check for centered text patterns (lots of leading spaces)
    // This is a common pattern in plain text books

    false
}

fn is_roman_numeral(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars().all(|c| {
        matches!(
            c,
            'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M' | 'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm'
        )
    })
}
