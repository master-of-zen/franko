//! Markdown format parser

use super::{Book, BookContent, BookMetadata, Chapter, ContentBlock, TocEntry};
use anyhow::{Context, Result};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::path::Path;

/// Parse a Markdown file
pub fn parse(path: &Path) -> Result<Book> {
    let content_str = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let (metadata, content) = parse_markdown(&content_str);

    // Use filename as title if not in frontmatter
    let metadata = if metadata.title.is_empty() {
        BookMetadata {
            title: path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            ..metadata
        }
    } else {
        metadata
    };

    Ok(Book {
        metadata,
        content,
        source_path: path.to_path_buf(),
        format: "markdown".to_string(),
    })
}

fn parse_markdown(source: &str) -> (BookMetadata, BookContent) {
    let mut metadata = BookMetadata::default();
    let mut content_start = 0;

    // Check for YAML frontmatter
    if source.starts_with("---") {
        if let Some(end) = source[3..].find("---") {
            let frontmatter = &source[3..3 + end];
            content_start = 3 + end + 3;
            
            // Simple YAML parsing for common fields
            for line in frontmatter.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim().to_lowercase();
                    let value = value.trim().trim_matches(&['"', '\''][..]).to_string();
                    
                    match key.as_str() {
                        "title" => metadata.title = value,
                        "author" | "authors" => {
                            metadata.authors = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect();
                        }
                        "date" | "published" => metadata.published = Some(value),
                        "description" | "summary" => metadata.description = Some(value),
                        "tags" | "subjects" => {
                            metadata.subjects = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect();
                        }
                        "lang" | "language" => metadata.language = Some(value),
                        _ => {}
                    }
                }
            }
        }
    }

    let md_content = &source[content_start..];
    let content = parse_markdown_content(md_content);

    // If no title from frontmatter, try to get from first heading
    if metadata.title.is_empty() {
        if let Some(chapter) = content.chapters.first() {
            if let Some(block) = chapter.blocks.first() {
                if let ContentBlock::Heading { text, level } = block {
                    if *level == 1 {
                        metadata.title = text.clone();
                    }
                }
            }
        }
    }

    (metadata, content)
}

fn parse_markdown_content(source: &str) -> BookContent {
    let options = Options::all();
    let parser = Parser::new_ext(source, options);

    let mut blocks = Vec::new();
    let mut toc = Vec::new();
    let mut current_text = String::new();
    let mut in_heading = false;
    let mut heading_level = 0u8;
    let mut in_list = false;
    let mut list_ordered = false;
    let mut list_items = Vec::new();
    let mut in_code_block = false;
    let mut code_lang = None;
    let mut code_content = String::new();
    let mut in_quote = false;
    let mut quote_text = String::new();
    let mut in_table = false;
    let mut table_headers = Vec::new();
    let mut table_rows = Vec::new();
    let mut current_row = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                flush_text(&mut current_text, &mut blocks);
                in_heading = true;
                heading_level = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
            }
            Event::End(TagEnd::Heading(_)) => {
                let text = std::mem::take(&mut current_text);
                
                // Add to TOC
                if heading_level <= 3 {
                    toc.push(TocEntry::new(
                        text.clone(),
                        format!("heading-{}", blocks.len()),
                        (heading_level - 1) as usize,
                    ));
                }
                
                blocks.push(ContentBlock::Heading {
                    level: heading_level,
                    text,
                });
                in_heading = false;
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                flush_text(&mut current_text, &mut blocks);
            }
            Event::Start(Tag::List(ordered)) => {
                flush_text(&mut current_text, &mut blocks);
                in_list = true;
                list_ordered = ordered.is_some();
                list_items.clear();
            }
            Event::End(TagEnd::List(_)) => {
                blocks.push(ContentBlock::List {
                    ordered: list_ordered,
                    items: std::mem::take(&mut list_items),
                });
                in_list = false;
            }
            Event::Start(Tag::Item) => {
                current_text.clear();
            }
            Event::End(TagEnd::Item) => {
                list_items.push(std::mem::take(&mut current_text));
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                flush_text(&mut current_text, &mut blocks);
                in_code_block = true;
                code_lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        let lang = lang.to_string();
                        if lang.is_empty() { None } else { Some(lang) }
                    }
                    _ => None,
                };
                code_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                blocks.push(ContentBlock::Code {
                    language: code_lang.take(),
                    code: std::mem::take(&mut code_content),
                });
                in_code_block = false;
            }
            Event::Start(Tag::BlockQuote) => {
                flush_text(&mut current_text, &mut blocks);
                in_quote = true;
                quote_text.clear();
            }
            Event::End(TagEnd::BlockQuote) => {
                blocks.push(ContentBlock::Quote {
                    text: std::mem::take(&mut quote_text).trim().to_string(),
                    attribution: None,
                });
                in_quote = false;
            }
            Event::Start(Tag::Table(_)) => {
                flush_text(&mut current_text, &mut blocks);
                in_table = true;
                table_headers.clear();
                table_rows.clear();
            }
            Event::End(TagEnd::Table) => {
                blocks.push(ContentBlock::Table {
                    headers: std::mem::take(&mut table_headers),
                    rows: std::mem::take(&mut table_rows),
                });
                in_table = false;
            }
            Event::Start(Tag::TableHead) => {
                current_row.clear();
            }
            Event::End(TagEnd::TableHead) => {
                table_headers = std::mem::take(&mut current_row);
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                if !current_row.is_empty() {
                    table_rows.push(std::mem::take(&mut current_row));
                }
            }
            Event::Start(Tag::TableCell) => {
                current_text.clear();
            }
            Event::End(TagEnd::TableCell) => {
                current_row.push(std::mem::take(&mut current_text));
            }
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                flush_text(&mut current_text, &mut blocks);
                blocks.push(ContentBlock::Image {
                    src: dest_url.to_string(),
                    alt: None,
                    caption: if title.is_empty() { None } else { Some(title.to_string()) },
                    data: None,
                });
            }
            Event::Text(text) => {
                if in_code_block {
                    code_content.push_str(&text);
                } else if in_quote {
                    quote_text.push_str(&text);
                } else {
                    current_text.push_str(&text);
                }
            }
            Event::Code(code) => {
                current_text.push('`');
                current_text.push_str(&code);
                current_text.push('`');
            }
            Event::SoftBreak => {
                if in_code_block {
                    code_content.push('\n');
                } else if in_quote {
                    quote_text.push(' ');
                } else {
                    current_text.push(' ');
                }
            }
            Event::HardBreak => {
                if in_code_block {
                    code_content.push('\n');
                } else if in_quote {
                    quote_text.push('\n');
                } else {
                    current_text.push('\n');
                }
            }
            Event::Rule => {
                flush_text(&mut current_text, &mut blocks);
                blocks.push(ContentBlock::Separator);
            }
            _ => {}
        }
    }

    flush_text(&mut current_text, &mut blocks);

    // Create a single chapter for the document
    let mut chapter = Chapter::new("main".to_string(), 0);
    chapter.title = None; // Will be set from first H1 if present
    chapter.blocks = blocks;

    BookContent {
        chapters: vec![chapter],
        toc,
    }
}

fn flush_text(text: &mut String, blocks: &mut Vec<ContentBlock>) {
    let trimmed = text.trim();
    if !trimmed.is_empty() {
        blocks.push(ContentBlock::Paragraph {
            text: trimmed.to_string(),
            styles: Vec::new(),
        });
    }
    text.clear();
}
