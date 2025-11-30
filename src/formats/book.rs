//! Core book data structures

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A parsed book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    /// Book metadata
    pub metadata: BookMetadata,

    /// Book content (chapters)
    pub content: BookContent,

    /// Original file path
    pub source_path: PathBuf,

    /// Format of the source file
    pub format: String,
}

/// Book metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BookMetadata {
    /// Book title
    pub title: String,

    /// Author(s)
    pub authors: Vec<String>,

    /// Publisher
    pub publisher: Option<String>,

    /// Publication date
    pub published: Option<String>,

    /// Language (ISO 639-1 code)
    pub language: Option<String>,

    /// ISBN
    pub isbn: Option<String>,

    /// Description/synopsis
    pub description: Option<String>,

    /// Subjects/genres/tags
    pub subjects: Vec<String>,

    /// Series name
    pub series: Option<String>,

    /// Position in series
    pub series_index: Option<f32>,

    /// Cover image (as bytes)
    #[serde(skip)]
    pub cover: Option<Vec<u8>>,

    /// Cover image MIME type
    pub cover_mime: Option<String>,

    /// Total word count
    pub word_count: Option<usize>,

    /// Estimated reading time in minutes
    pub reading_time: Option<usize>,
}

impl BookMetadata {
    /// Get the primary author
    pub fn author(&self) -> Option<&str> {
        self.authors.first().map(|s| s.as_str())
    }

    /// Get formatted author string
    pub fn authors_string(&self) -> String {
        if self.authors.is_empty() {
            "Unknown Author".to_string()
        } else if self.authors.len() == 1 {
            self.authors[0].clone()
        } else if self.authors.len() == 2 {
            format!("{} and {}", self.authors[0], self.authors[1])
        } else {
            format!(
                "{}, and {}",
                self.authors[..self.authors.len() - 1].join(", "),
                self.authors.last().unwrap()
            )
        }
    }

    /// Calculate reading time based on word count
    pub fn calculate_reading_time(&mut self, words_per_minute: u32) {
        if let Some(wc) = self.word_count {
            self.reading_time = Some(wc / words_per_minute as usize);
        }
    }
}

/// Book content container
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BookContent {
    /// List of chapters
    pub chapters: Vec<Chapter>,

    /// Table of contents
    pub toc: Vec<TocEntry>,
}

impl BookContent {
    /// Get total number of paragraphs across all chapters
    pub fn total_paragraphs(&self) -> usize {
        self.chapters.iter().map(|c| c.blocks.len()).sum()
    }

    /// Get total word count
    pub fn word_count(&self) -> usize {
        self.chapters
            .iter()
            .flat_map(|c| &c.blocks)
            .map(|b| b.word_count())
            .sum()
    }

    /// Find chapter by ID
    pub fn get_chapter(&self, id: &str) -> Option<&Chapter> {
        self.chapters.iter().find(|c| c.id == id)
    }

    /// Find chapter by index
    pub fn get_chapter_by_index(&self, index: usize) -> Option<&Chapter> {
        self.chapters.get(index)
    }
}

/// A chapter in the book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// Unique identifier
    pub id: String,

    /// Chapter title
    pub title: Option<String>,

    /// Chapter number (if applicable)
    pub number: Option<usize>,

    /// Content blocks (paragraphs, headings, etc.)
    pub blocks: Vec<ContentBlock>,

    /// Order in the book
    pub order: usize,
}

impl Chapter {
    pub fn new(id: String, order: usize) -> Self {
        Self {
            id,
            title: None,
            number: None,
            blocks: Vec::new(),
            order,
        }
    }

    /// Get display title
    pub fn display_title(&self) -> String {
        if let Some(title) = &self.title {
            if let Some(num) = self.number {
                format!("Chapter {}: {}", num, title)
            } else {
                title.clone()
            }
        } else if let Some(num) = self.number {
            format!("Chapter {}", num)
        } else {
            format!("Section {}", self.order + 1)
        }
    }

    /// Get word count for this chapter
    pub fn word_count(&self) -> usize {
        self.blocks.iter().map(|b| b.word_count()).sum()
    }
}

/// A content block (paragraph, heading, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentBlock {
    /// A paragraph of text
    Paragraph {
        text: String,
        styles: Vec<TextStyle>,
    },

    /// A heading
    Heading {
        level: u8,
        text: String,
    },

    /// A blockquote
    Quote {
        text: String,
        attribution: Option<String>,
    },

    /// A code block
    Code {
        language: Option<String>,
        code: String,
    },

    /// An image
    Image {
        src: String,
        alt: Option<String>,
        caption: Option<String>,
        #[serde(skip)]
        data: Option<Vec<u8>>,
    },

    /// A list
    List {
        ordered: bool,
        items: Vec<String>,
    },

    /// A horizontal rule/separator
    Separator,

    /// A footnote
    Footnote {
        id: String,
        content: String,
    },

    /// Raw HTML (for formats that support it)
    RawHtml {
        html: String,
    },

    /// A table
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },

    /// Empty space/break
    Break,
}

impl ContentBlock {
    /// Get the text content of this block
    pub fn text(&self) -> String {
        match self {
            ContentBlock::Paragraph { text, .. } => text.clone(),
            ContentBlock::Heading { text, .. } => text.clone(),
            ContentBlock::Quote { text, .. } => text.clone(),
            ContentBlock::Code { code, .. } => code.clone(),
            ContentBlock::Image { alt, caption, .. } => {
                caption.clone().or_else(|| alt.clone()).unwrap_or_default()
            }
            ContentBlock::List { items, .. } => items.join("\n"),
            ContentBlock::Footnote { content, .. } => content.clone(),
            ContentBlock::RawHtml { html } => html2text::from_read(html.as_bytes(), 80),
            ContentBlock::Table { headers, rows } => {
                let mut text = headers.join(" | ");
                for row in rows {
                    text.push('\n');
                    text.push_str(&row.join(" | "));
                }
                text
            }
            ContentBlock::Separator | ContentBlock::Break => String::new(),
        }
    }

    /// Get word count for this block
    pub fn word_count(&self) -> usize {
        self.text().split_whitespace().count()
    }

    /// Check if this is a heading
    pub fn is_heading(&self) -> bool {
        matches!(self, ContentBlock::Heading { .. })
    }

    /// Check if this is a paragraph
    pub fn is_paragraph(&self) -> bool {
        matches!(self, ContentBlock::Paragraph { .. })
    }
}

/// Text styling information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    /// Start position in text
    pub start: usize,

    /// End position in text
    pub end: usize,

    /// Style type
    pub style_type: StyleType,
}

/// Types of text styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StyleType {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Code,
    Link,
    Superscript,
    Subscript,
    SmallCaps,
}

/// Table of contents entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocEntry {
    /// Entry title
    pub title: String,

    /// Link to chapter/section
    pub href: String,

    /// Nesting level (0 = top level)
    pub level: usize,

    /// Child entries
    pub children: Vec<TocEntry>,
}

impl TocEntry {
    pub fn new(title: String, href: String, level: usize) -> Self {
        Self {
            title,
            href,
            level,
            children: Vec::new(),
        }
    }
}
