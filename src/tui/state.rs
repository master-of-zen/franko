//! TUI application state

use crate::formats::{Book, Chapter};

/// Application mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Normal reading mode
    Normal,
    /// Command input mode
    Command,
    /// Search mode
    Search,
    /// Bookmark selection mode
    Bookmark,
    /// Help overlay
    Help,
    /// Table of contents
    TableOfContents,
    /// Go to line/page
    GoTo,
}

/// Reading position
#[derive(Debug, Clone, Default)]
pub struct Position {
    /// Current chapter index
    pub chapter: usize,
    /// Current block/paragraph index within chapter
    pub block: usize,
    /// Scroll offset within the current view
    pub scroll_offset: usize,
    /// Line offset within current block (for long paragraphs)
    pub line_offset: usize,
}

impl Position {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Search state
#[derive(Debug, Clone, Default)]
pub struct SearchState {
    /// Current search query
    pub query: String,
    /// Search results (chapter_idx, block_idx, start, end)
    pub results: Vec<(usize, usize, usize, usize)>,
    /// Current result index
    pub current_result: usize,
    /// Is search active
    pub active: bool,
    /// Cursor position in search input
    pub cursor: usize,
}

impl SearchState {
    pub fn clear(&mut self) {
        self.query.clear();
        self.results.clear();
        self.current_result = 0;
        self.active = false;
        self.cursor = 0;
    }

    pub fn next_result(&mut self) {
        if !self.results.is_empty() {
            self.current_result = (self.current_result + 1) % self.results.len();
        }
    }

    pub fn prev_result(&mut self) {
        if !self.results.is_empty() {
            self.current_result = if self.current_result == 0 {
                self.results.len() - 1
            } else {
                self.current_result - 1
            };
        }
    }
}

/// Bookmark data
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: String,
    pub name: String,
    pub position: Position,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Annotation data
#[derive(Debug, Clone)]
pub struct Annotation {
    pub id: String,
    pub text: String,
    pub position: Position,
    pub note: Option<String>,
    pub color: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Main application state
pub struct AppState {
    /// The book being read
    pub book: Book,

    /// Current reading position
    pub position: Position,

    /// Current mode
    pub mode: Mode,

    /// Previous mode (for returning from overlays)
    pub previous_mode: Mode,

    /// Search state
    pub search: SearchState,

    /// Command input buffer
    pub command_buffer: String,

    /// Command cursor position
    pub command_cursor: usize,

    /// Bookmarks
    pub bookmarks: Vec<Bookmark>,

    /// Annotations
    pub annotations: Vec<Annotation>,

    /// Show sidebar
    pub show_sidebar: bool,

    /// Show status bar
    pub show_status_bar: bool,

    /// Show line numbers
    pub show_line_numbers: bool,

    /// Fullscreen mode (hide all UI)
    pub fullscreen: bool,

    /// Current theme name
    pub theme: String,

    /// Rendered lines cache
    pub lines_cache: Vec<RenderedLine>,

    /// Total lines in current chapter
    pub total_lines: usize,

    /// Terminal size
    pub terminal_size: (u16, u16),

    /// Messages to display
    pub message: Option<(String, MessageType)>,

    /// Message timeout counter
    pub message_timeout: u8,

    /// Whether the app should quit
    pub should_quit: bool,

    /// Unsaved changes
    pub dirty: bool,
}

/// Type of status message
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
}

/// A rendered line of text
#[derive(Debug, Clone)]
pub struct RenderedLine {
    /// The text content
    pub text: String,
    /// Block index this line belongs to
    pub block_index: usize,
    /// Whether this is a heading
    pub is_heading: bool,
    /// Heading level (if heading)
    pub heading_level: u8,
    /// Whether this is a quote
    pub is_quote: bool,
    /// Whether this is code
    pub is_code: bool,
    /// Search highlights (start, end)
    pub highlights: Vec<(usize, usize)>,
}

impl RenderedLine {
    /// Create an empty line
    pub fn empty(block_index: usize) -> Self {
        Self {
            text: String::new(),
            block_index,
            is_heading: false,
            heading_level: 0,
            is_quote: false,
            is_code: false,
            highlights: Vec::new(),
        }
    }
}

impl AppState {
    pub fn new(book: Book) -> Self {
        Self {
            book,
            position: Position::new(),
            mode: Mode::Normal,
            previous_mode: Mode::Normal,
            search: SearchState::default(),
            command_buffer: String::new(),
            command_cursor: 0,
            bookmarks: Vec::new(),
            annotations: Vec::new(),
            show_sidebar: false,
            show_status_bar: true,
            show_line_numbers: false,
            fullscreen: false,
            theme: "dark".to_string(),
            lines_cache: Vec::new(),
            total_lines: 0,
            terminal_size: (80, 24),
            message: None,
            message_timeout: 0,
            should_quit: false,
            dirty: false,
        }
    }

    /// Get current chapter
    pub fn current_chapter(&self) -> Option<&Chapter> {
        self.book.content.chapters.get(self.position.chapter)
    }

    /// Get current chapter mut
    pub fn current_chapter_mut(&mut self) -> Option<&mut Chapter> {
        self.book.content.chapters.get_mut(self.position.chapter)
    }

    /// Move to next chapter
    pub fn next_chapter(&mut self) {
        if self.position.chapter < self.book.content.chapters.len() - 1 {
            self.position.chapter += 1;
            self.position.block = 0;
            self.position.scroll_offset = 0;
            self.invalidate_cache();
        }
    }

    /// Move to previous chapter
    pub fn prev_chapter(&mut self) {
        if self.position.chapter > 0 {
            self.position.chapter -= 1;
            self.position.block = 0;
            self.position.scroll_offset = 0;
            self.invalidate_cache();
        }
    }

    /// Scroll down by n lines
    pub fn scroll_down(&mut self, n: usize) {
        self.position.scroll_offset = self.position.scroll_offset.saturating_add(n);
        if self.position.scroll_offset >= self.total_lines.saturating_sub(1) {
            self.position.scroll_offset = self.total_lines.saturating_sub(1);
        }
    }

    /// Scroll up by n lines
    pub fn scroll_up(&mut self, n: usize) {
        self.position.scroll_offset = self.position.scroll_offset.saturating_sub(n);
    }

    /// Go to top
    pub fn go_to_top(&mut self) {
        self.position.scroll_offset = 0;
    }

    /// Go to bottom
    pub fn go_to_bottom(&mut self) {
        if self.total_lines > 0 {
            let visible_height = self.visible_height();
            self.position.scroll_offset = self.total_lines.saturating_sub(visible_height);
        }
    }

    /// Page down
    pub fn page_down(&mut self) {
        let height = self.visible_height();
        self.scroll_down(height.saturating_sub(2)); // Keep 2 lines of context
    }

    /// Page up
    pub fn page_up(&mut self) {
        let height = self.visible_height();
        self.scroll_up(height.saturating_sub(2));
    }

    /// Half page down
    pub fn half_page_down(&mut self) {
        let height = self.visible_height();
        self.scroll_down(height / 2);
    }

    /// Half page up
    pub fn half_page_up(&mut self) {
        let height = self.visible_height();
        self.scroll_up(height / 2);
    }

    /// Get visible height (accounting for UI elements)
    pub fn visible_height(&self) -> usize {
        let mut height = self.terminal_size.1 as usize;
        if self.show_status_bar {
            height = height.saturating_sub(2); // Status bar + progress bar
        }
        height.saturating_sub(2) // Margins
    }

    /// Get visible width
    pub fn visible_width(&self) -> usize {
        let mut width = self.terminal_size.0 as usize;
        if self.show_sidebar {
            width = width.saturating_sub(30); // Sidebar width
        }
        if self.show_line_numbers {
            width = width.saturating_sub(6); // Line number column
        }
        width.saturating_sub(8) // Left and right margins
    }

    /// Invalidate the lines cache
    pub fn invalidate_cache(&mut self) {
        self.lines_cache.clear();
    }

    /// Set mode
    pub fn set_mode(&mut self, mode: Mode) {
        self.previous_mode = self.mode;
        self.mode = mode;
    }

    /// Return to previous mode
    pub fn return_to_previous_mode(&mut self) {
        self.mode = self.previous_mode;
    }

    /// Toggle sidebar
    pub fn toggle_sidebar(&mut self) {
        self.show_sidebar = !self.show_sidebar;
        self.invalidate_cache();
    }

    /// Toggle status bar
    pub fn toggle_status_bar(&mut self) {
        self.show_status_bar = !self.show_status_bar;
        self.invalidate_cache();
    }

    /// Toggle line numbers
    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
        self.invalidate_cache();
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
        self.invalidate_cache();
    }

    /// Show a message
    pub fn show_message(&mut self, msg: String, msg_type: MessageType) {
        self.message = Some((msg, msg_type));
        self.message_timeout = 30; // About 3 seconds at 10 fps
    }

    /// Clear message if timed out
    pub fn tick_message(&mut self) {
        if self.message_timeout > 0 {
            self.message_timeout -= 1;
            if self.message_timeout == 0 {
                self.message = None;
            }
        }
    }

    /// Add a bookmark at current position
    pub fn add_bookmark(&mut self, name: Option<String>) {
        let id = uuid::Uuid::new_v4().to_string();
        let name = name.unwrap_or_else(|| {
            format!(
                "Bookmark at Ch.{} P.{}",
                self.position.chapter + 1,
                self.position.block + 1
            )
        });

        self.bookmarks.push(Bookmark {
            id,
            name,
            position: self.position.clone(),
            created_at: chrono::Utc::now(),
        });

        self.dirty = true;
        self.show_message("Bookmark added".to_string(), MessageType::Success);
    }

    /// Go to a bookmark
    pub fn go_to_bookmark(&mut self, index: usize) {
        if let Some(bookmark) = self.bookmarks.get(index) {
            let position = bookmark.position.clone();
            let name = bookmark.name.clone();
            self.position = position;
            self.invalidate_cache();
            self.show_message(format!("Jumped to: {}", name), MessageType::Info);
        }
    }

    /// Calculate reading progress (0.0 - 1.0)
    pub fn progress(&self) -> f64 {
        let total_chapters = self.book.content.chapters.len();
        if total_chapters == 0 {
            return 0.0;
        }

        let chapter_progress = self.position.chapter as f64 / total_chapters as f64;
        let block_progress = if let Some(chapter) = self.current_chapter() {
            if chapter.blocks.is_empty() {
                0.0
            } else {
                self.position.block as f64 / chapter.blocks.len() as f64 / total_chapters as f64
            }
        } else {
            0.0
        };

        (chapter_progress + block_progress).min(1.0)
    }

    /// Get progress percentage string
    pub fn progress_string(&self) -> String {
        format!("{:.1}%", self.progress() * 100.0)
    }

    /// Perform search
    pub fn perform_search(&mut self) {
        self.search.results.clear();
        self.search.current_result = 0;

        if self.search.query.is_empty() {
            return;
        }

        let query_lower = self.search.query.to_lowercase();

        for (chapter_idx, chapter) in self.book.content.chapters.iter().enumerate() {
            for (block_idx, block) in chapter.blocks.iter().enumerate() {
                let text = block.text();
                let text_lower = text.to_lowercase();

                let mut start = 0;
                while let Some(pos) = text_lower[start..].find(&query_lower) {
                    let absolute_pos = start + pos;
                    self.search.results.push((
                        chapter_idx,
                        block_idx,
                        absolute_pos,
                        absolute_pos + self.search.query.len(),
                    ));
                    start = absolute_pos + 1;
                }
            }
        }

        self.search.active = !self.search.results.is_empty();

        let count = self.search.results.len();
        if count > 0 {
            self.show_message(format!("Found {} matches", count), MessageType::Info);
            self.go_to_search_result();
        } else {
            self.show_message("No matches found".to_string(), MessageType::Warning);
        }
    }

    /// Go to current search result
    pub fn go_to_search_result(&mut self) {
        if let Some(&(chapter_idx, block_idx, _, _)) =
            self.search.results.get(self.search.current_result)
        {
            self.position.chapter = chapter_idx;
            self.position.block = block_idx;
            self.invalidate_cache();
        }
    }

    /// Search next
    pub fn search_next(&mut self) {
        self.search.next_result();
        self.go_to_search_result();
    }

    /// Search previous
    pub fn search_prev(&mut self) {
        self.search.prev_result();
        self.go_to_search_result();
    }
}
