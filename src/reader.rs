//! Reader module - coordinates reading sessions across interfaces
//!
//! This module provides the unified interface for reading books,
//! dispatching to either TUI or Web interface based on configuration.

use std::path::Path;

use crate::config::Config;
use crate::error::Result;
use crate::formats::{Book, parse_book};
use crate::cli::Interface;

/// Open and read a book with the specified interface
pub async fn read_book(path: &Path, config: &Config, interface: Option<Interface>) -> Result<()> {
    // Parse the book
    let book = parse_book(path)?;
    
    // Determine which interface to use
    let interface = interface.unwrap_or_else(|| {
        if config.reader.prefer_interface == "web" {
            Interface::Web
        } else {
            Interface::Tui
        }
    });
    
    match interface {
        Interface::Tui => {
            #[cfg(feature = "tui")]
            {
                crate::tui::run(book, config)?;
            }
            #[cfg(not(feature = "tui"))]
            {
                anyhow::bail!("TUI feature is not enabled. Rebuild with --features tui");
            }
        }
        Interface::Web => {
            #[cfg(feature = "web")]
            {
                // For single-book web mode, we start server with the book preloaded
                crate::web::serve_book(book, config).await?;
            }
            #[cfg(not(feature = "web"))]
            {
                anyhow::bail!("Web feature is not enabled. Rebuild with --features web");
            }
        }
    }
    
    Ok(())
}

/// Reading session state
#[derive(Debug, Clone)]
pub struct ReadingSession {
    pub book: Book,
    pub current_chapter: usize,
    pub current_block: usize,
    pub scroll_offset: usize,
}

impl ReadingSession {
    /// Create a new reading session
    pub fn new(book: Book) -> Self {
        Self {
            book,
            current_chapter: 0,
            current_block: 0,
            scroll_offset: 0,
        }
    }
    
    /// Create a session resuming from a saved position
    pub fn resume(book: Book, chapter: usize, block: usize) -> Self {
        let chapter = chapter.min(book.content.chapters.len().saturating_sub(1));
        let block = book.content.chapters.get(chapter)
            .map(|c| block.min(c.blocks.len().saturating_sub(1)))
            .unwrap_or(0);
        
        Self {
            book,
            current_chapter: chapter,
            current_block: block,
            scroll_offset: 0,
        }
    }
    
    /// Get the current chapter
    pub fn chapter(&self) -> Option<&crate::formats::Chapter> {
        self.book.content.chapters.get(self.current_chapter)
    }
    
    /// Get total number of chapters
    pub fn total_chapters(&self) -> usize {
        self.book.content.chapters.len()
    }
    
    /// Go to next chapter
    pub fn next_chapter(&mut self) -> bool {
        if self.current_chapter + 1 < self.book.content.chapters.len() {
            self.current_chapter += 1;
            self.current_block = 0;
            self.scroll_offset = 0;
            true
        } else {
            false
        }
    }
    
    /// Go to previous chapter
    pub fn prev_chapter(&mut self) -> bool {
        if self.current_chapter > 0 {
            self.current_chapter -= 1;
            self.current_block = 0;
            self.scroll_offset = 0;
            true
        } else {
            false
        }
    }
    
    /// Go to a specific chapter
    pub fn goto_chapter(&mut self, chapter: usize) -> bool {
        if chapter < self.book.content.chapters.len() {
            self.current_chapter = chapter;
            self.current_block = 0;
            self.scroll_offset = 0;
            true
        } else {
            false
        }
    }
    
    /// Calculate reading progress (0.0 - 1.0)
    pub fn progress(&self) -> f64 {
        if self.book.content.chapters.is_empty() {
            return 0.0;
        }
        
        let total_blocks: usize = self.book.content.chapters.iter()
            .map(|c| c.blocks.len())
            .sum();
        
        if total_blocks == 0 {
            return 0.0;
        }
        
        let blocks_before: usize = self.book.content.chapters.iter()
            .take(self.current_chapter)
            .map(|c| c.blocks.len())
            .sum();
        
        let current = blocks_before + self.current_block;
        
        current as f64 / total_blocks as f64
    }
    
    /// Get position string for display
    pub fn position_string(&self) -> String {
        format!(
            "Chapter {}/{} | Block {}/{}",
            self.current_chapter + 1,
            self.total_chapters(),
            self.current_block + 1,
            self.chapter().map(|c| c.blocks.len()).unwrap_or(0)
        )
    }
    
    /// Get progress percentage string
    pub fn progress_string(&self) -> String {
        format!("{:.1}%", self.progress() * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::{BookMetadata, BookContent, Chapter, ContentBlock};
    use std::path::PathBuf;
    
    fn create_test_book() -> Book {
        Book {
            metadata: BookMetadata {
                title: "Test Book".to_string(),
                ..Default::default()
            },
            content: BookContent {
                chapters: vec![
                    Chapter {
                        id: "ch1".to_string(),
                        title: Some("Chapter 1".to_string()),
                        number: Some(1),
                        blocks: vec![
                            ContentBlock::Paragraph {
                                text: "Para 1".to_string(),
                                styles: vec![],
                            },
                            ContentBlock::Paragraph {
                                text: "Para 2".to_string(),
                                styles: vec![],
                            },
                        ],
                        order: 0,
                    },
                    Chapter {
                        id: "ch2".to_string(),
                        title: Some("Chapter 2".to_string()),
                        number: Some(2),
                        blocks: vec![
                            ContentBlock::Paragraph {
                                text: "Para 1".to_string(),
                                styles: vec![],
                            },
                        ],
                        order: 1,
                    },
                ],
                toc: vec![],
            },
            source_path: PathBuf::new(),
            format: "test".to_string(),
        }
    }
    
    #[test]
    fn test_reading_session_navigation() {
        let book = create_test_book();
        let mut session = ReadingSession::new(book);
        
        assert_eq!(session.current_chapter, 0);
        assert!(session.next_chapter());
        assert_eq!(session.current_chapter, 1);
        assert!(!session.next_chapter()); // No more chapters
        
        assert!(session.prev_chapter());
        assert_eq!(session.current_chapter, 0);
        assert!(!session.prev_chapter()); // Already at start
    }
    
    #[test]
    fn test_reading_session_progress() {
        let book = create_test_book();
        let mut session = ReadingSession::new(book);
        
        assert_eq!(session.progress(), 0.0);
        
        session.current_block = 1;
        // 1 out of 3 blocks = ~0.33
        assert!((session.progress() - 0.333).abs() < 0.01);
        
        session.next_chapter();
        // 2 out of 3 blocks = ~0.66
        assert!((session.progress() - 0.666).abs() < 0.01);
    }
}
