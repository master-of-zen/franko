//! EPUB format parser

use super::{Book, BookContent, BookMetadata, Chapter, ContentBlock, TocEntry};
use anyhow::Result;
use std::path::Path;

/// Parse an EPUB file
pub fn parse(path: &Path) -> Result<Book> {
    let doc = epub::doc::EpubDoc::new(path)
        .map_err(|e| anyhow::anyhow!("Failed to open EPUB: {:?}", e))?;

    let metadata = extract_metadata(&doc)?;
    let content = extract_content(path)?;

    Ok(Book {
        metadata,
        content,
        source_path: path.to_path_buf(),
        format: "epub".to_string(),
    })
}

/// Extract metadata from EPUB
pub fn metadata(path: &Path) -> Result<BookMetadata> {
    let doc = epub::doc::EpubDoc::new(path)
        .map_err(|e| anyhow::anyhow!("Failed to open EPUB: {:?}", e))?;

    extract_metadata(&doc)
}

fn get_metadata_string(doc: &epub::doc::EpubDoc<std::io::BufReader<std::fs::File>>, key: &str) -> Option<String> {
    doc.mdata(key).map(|item| item.value.clone())
}

fn extract_metadata(doc: &epub::doc::EpubDoc<std::io::BufReader<std::fs::File>>) -> Result<BookMetadata> {
    let title = get_metadata_string(doc, "title")
        .unwrap_or_else(|| "Unknown Title".to_string());
    
    let authors = get_metadata_string(doc, "creator")
        .map(|a| vec![a])
        .unwrap_or_default();

    let publisher = get_metadata_string(doc, "publisher");
    let language = get_metadata_string(doc, "language");
    let description = get_metadata_string(doc, "description");
    let published = get_metadata_string(doc, "date");

    // Extract subjects/tags
    let subjects = get_metadata_string(doc, "subject")
        .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
        .unwrap_or_default();

    Ok(BookMetadata {
        title,
        authors,
        publisher,
        language,
        description,
        published,
        subjects,
        isbn: get_metadata_string(doc, "identifier"),
        series: None,
        series_index: None,
        cover: None,
        cover_mime: None,
        word_count: None,
        reading_time: None,
    })
}

fn extract_content(path: &Path) -> Result<BookContent> {
    let mut doc = epub::doc::EpubDoc::new(path)
        .map_err(|e| anyhow::anyhow!("Failed to open EPUB: {:?}", e))?;

    let mut chapters = Vec::new();
    let mut order = 0;

    // Get the spine (reading order)
    let spine = doc.spine.clone();
    
    for spine_item in &spine {
        // spine_item is a SpineItem, we need to get its idref
        let chapter_id = &spine_item.idref;
        
        // Try to get the content
        if let Some((content, _mime)) = doc.get_resource(chapter_id) {
            let html = String::from_utf8_lossy(&content).to_string();
            let blocks = parse_html_content(&html);
            
            // Try to extract title from first heading
            let title = blocks.iter()
                .find_map(|b| match b {
                    ContentBlock::Heading { text, level } if *level <= 2 => Some(text.clone()),
                    _ => None,
                });

            let mut chapter = Chapter::new(chapter_id.clone(), order);
            chapter.title = title;
            chapter.blocks = blocks;
            
            chapters.push(chapter);
            order += 1;
        }
    }

    // Build TOC
    let toc = build_toc(&doc);

    Ok(BookContent { chapters, toc })
}

fn parse_html_content(html: &str) -> Vec<ContentBlock> {
    let mut blocks = Vec::new();
    
    // Try html2text first with a wider width for better text extraction
    let text = html2text::from_read(html.as_bytes(), 10000);
    
    // If html2text gives us nothing useful, try manual extraction
    if text.trim().is_empty() || text.trim().len() < 20 {
        // Fallback: extract text between tags manually
        let mut cleaned = html.to_string();
        
        // Remove script and style tags completely
        let script_re = regex::Regex::new(r"(?is)<script[^>]*>.*?</script>").unwrap();
        cleaned = script_re.replace_all(&cleaned, "").to_string();
        let style_re = regex::Regex::new(r"(?is)<style[^>]*>.*?</style>").unwrap();
        cleaned = style_re.replace_all(&cleaned, "").to_string();
        
        // Extract paragraphs
        let p_re = regex::Regex::new(r"(?is)<p[^>]*>(.*?)</p>").unwrap();
        for cap in p_re.captures_iter(&cleaned) {
            if let Some(content) = cap.get(1) {
                let text = strip_tags(content.as_str());
                let trimmed = text.trim();
                if !trimmed.is_empty() && trimmed.len() > 1 {
                    blocks.push(ContentBlock::Paragraph {
                        text: trimmed.to_string(),
                        styles: Vec::new(),
                    });
                }
            }
        }
        
        // Extract headings
        let h_re = regex::Regex::new(r"(?is)<h([1-6])[^>]*>(.*?)</h[1-6]>").unwrap();
        for cap in h_re.captures_iter(&cleaned) {
            if let (Some(level), Some(content)) = (cap.get(1), cap.get(2)) {
                let text = strip_tags(content.as_str());
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    let level_num: u8 = level.as_str().parse().unwrap_or(1);
                    blocks.push(ContentBlock::Heading {
                        level: level_num,
                        text: trimmed.to_string(),
                    });
                }
            }
        }
        
        // If still no content, try to get any text from body
        if blocks.is_empty() {
            let body_re = regex::Regex::new(r"(?is)<body[^>]*>(.*?)</body>").unwrap();
            if let Some(cap) = body_re.captures(&cleaned) {
                if let Some(body) = cap.get(1) {
                    let text = strip_tags(body.as_str());
                    let trimmed = text.trim();
                    if !trimmed.is_empty() && trimmed.len() > 10 {
                        // Split into paragraphs by double newlines or significant whitespace
                        for para in trimmed.split("\n\n") {
                            let p = para.trim();
                            if !p.is_empty() && p.len() > 1 {
                                blocks.push(ContentBlock::Paragraph {
                                    text: p.to_string(),
                                    styles: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        return blocks;
    }
    
    // Process html2text output
    for para in text.split("\n\n") {
        let trimmed = para.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Detect headings (lines that are short and possibly styled)
        if trimmed.len() < 100 && !trimmed.contains('.') && trimmed.lines().count() == 1 {
            // Check if it looks like a heading (all caps, or short single line)
            if trimmed.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) && trimmed.len() > 2 {
                blocks.push(ContentBlock::Heading {
                    level: 1,
                    text: trimmed.to_string(),
                });
                continue;
            }
        }
        
        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|&c| c == '#').count() as u8;
            blocks.push(ContentBlock::Heading {
                level: level.min(6),
                text: trimmed.trim_start_matches('#').trim().to_string(),
            });
        } else if trimmed.starts_with('>') || (trimmed.starts_with('"') && trimmed.len() > 50) {
            blocks.push(ContentBlock::Quote {
                text: trimmed.trim_start_matches(&['>', '"'][..]).trim().to_string(),
                attribution: None,
            });
        } else if trimmed == "---" || trimmed == "***" || trimmed == "* * *" {
            blocks.push(ContentBlock::Separator);
        } else {
            blocks.push(ContentBlock::Paragraph {
                text: trimmed.to_string(),
                styles: Vec::new(),
            });
        }
    }

    blocks
}

/// Strip HTML tags from text
fn strip_tags(html: &str) -> String {
    let tag_re = regex::Regex::new(r"<[^>]+>").unwrap();
    let result = tag_re.replace_all(html, "");
    
    // Decode common HTML entities
    result
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&#x27;", "'")
        .replace("&#8220;", "\"")
        .replace("&#8221;", "\"")
        .replace("&#8216;", "'")
        .replace("&#8217;", "'")
        .replace("&#8212;", "—")
        .replace("&#8211;", "–")
        .replace("&#160;", " ")
}

fn build_toc(doc: &epub::doc::EpubDoc<std::io::BufReader<std::fs::File>>) -> Vec<TocEntry> {
    // EPUB crate doesn't expose TOC directly, so we'll build from spine
    doc.spine.iter().enumerate().map(|(i, item)| {
        TocEntry::new(
            format!("Section {}", i + 1),
            item.idref.clone(),
            0,
        )
    }).collect()
}
