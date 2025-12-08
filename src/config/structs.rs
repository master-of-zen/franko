//! Configuration structures

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// General settings
    pub general: GeneralConfig,

    /// TUI-specific settings
    pub tui: TuiConfig,

    /// Web interface settings
    pub web: WebConfig,

    /// Library settings
    pub library: LibraryConfig,

    /// Reader settings
    pub reader: ReaderConfig,

    /// Format-specific settings
    pub formats: FormatsConfig,

    /// Keybindings
    pub keybindings: super::Keybindings,

    /// Theme configuration
    pub theme: super::ThemeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Default interface to use
    pub default_interface: String,

    /// Data directory for library and cache
    pub data_dir: Option<PathBuf>,

    /// Enable auto-save of reading progress
    pub auto_save: bool,

    /// Auto-save interval in seconds
    pub auto_save_interval: u64,

    /// Enable logging
    pub logging: bool,

    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,

    /// Language for the interface
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TuiConfig {
    /// Enable mouse support
    pub mouse_support: bool,

    /// Enable Unicode rendering
    pub unicode: bool,

    /// Show line numbers
    pub line_numbers: bool,

    /// Show status bar
    pub status_bar: bool,

    /// Status bar position (top, bottom)
    pub status_bar_position: String,

    /// Show progress bar
    pub progress_bar: bool,

    /// Enable animations
    pub animations: bool,

    /// Animation speed (slow, normal, fast)
    pub animation_speed: String,

    /// Scrolloff - lines to keep above/below cursor
    pub scrolloff: usize,

    /// Enable smooth scrolling
    pub smooth_scroll: bool,

    /// Dim unfocused panes
    pub dim_unfocused: bool,

    /// Show chapter navigation sidebar
    pub show_sidebar: bool,

    /// Sidebar width (percentage)
    pub sidebar_width: u16,

    /// Tab size for display
    pub tab_size: usize,

    /// Word wrap mode (none, word, char)
    pub wrap_mode: String,

    /// Maximum line width (0 for terminal width)
    pub max_width: usize,

    /// Left margin
    pub margin_left: usize,

    /// Right margin
    pub margin_right: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WebConfig {
    /// Default port
    pub port: u16,

    /// Bind address
    pub bind: String,

    /// Enable HTTPS
    pub https: bool,

    /// Path to TLS certificate
    pub tls_cert: Option<PathBuf>,

    /// Path to TLS key
    pub tls_key: Option<PathBuf>,

    /// Enable authentication
    pub auth_enabled: bool,

    /// Authentication method (basic, token)
    pub auth_method: String,

    /// Enable CORS
    pub cors: bool,

    /// CORS allowed origins
    pub cors_origins: Vec<String>,

    /// Enable compression
    pub compression: bool,

    /// Static files directory
    pub static_dir: Option<PathBuf>,

    /// Custom CSS file
    pub custom_css: Option<PathBuf>,

    /// Custom JS file
    pub custom_js: Option<PathBuf>,

    /// Page size for pagination
    pub page_size: usize,

    /// Enable dark mode by default
    pub dark_mode: bool,

    /// Font family
    pub font_family: String,

    /// Font size (in pixels)
    pub font_size: u16,

    /// Line height
    pub line_height: f32,

    /// Automatically open browser when starting web server
    pub open_browser: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LibraryConfig {
    /// Library database path
    pub database_path: Option<PathBuf>,

    /// Default book directory
    pub books_dir: Option<PathBuf>,

    /// Watch for new books
    pub watch_enabled: bool,

    /// Scan directories for covers
    pub extract_covers: bool,

    /// Cover cache directory
    pub covers_dir: Option<PathBuf>,

    /// Maximum cover size (width)
    pub cover_max_width: u32,

    /// Enable full-text search
    pub search_enabled: bool,

    /// Search index directory
    pub search_index_dir: Option<PathBuf>,

    /// Automatically index new books
    pub auto_index: bool,

    /// Backup library on exit
    pub backup_enabled: bool,

    /// Number of backups to keep
    pub backup_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReaderConfig {
    /// Remember reading position
    pub remember_position: bool,

    /// Show reading time estimate
    pub show_reading_time: bool,

    /// Words per minute (for estimates)
    pub words_per_minute: u32,

    /// Enable dictionary lookup
    pub dictionary_enabled: bool,

    /// Dictionary source
    pub dictionary_source: String,

    /// Enable text-to-speech
    pub tts_enabled: bool,

    /// TTS voice
    pub tts_voice: String,

    /// TTS speed (0.5 - 2.0)
    pub tts_speed: f32,

    /// Highlight search matches
    pub highlight_search: bool,

    /// Search case sensitive
    pub search_case_sensitive: bool,

    /// Search regex support
    pub search_regex: bool,

    /// Justify text
    pub justify: bool,

    /// Hyphenation
    pub hyphenation: bool,

    /// Hyphenation language
    pub hyphenation_lang: String,

    /// Preferred interface ("tui" or "web")
    pub prefer_interface: String,

    /// Reading layout mode: "scroll", "paged", "dual"
    pub layout_mode: String,

    /// Pages per view (1, 2, or 3) for paged/dual mode
    pub pages_per_view: u8,

    /// Page turn animation: "none", "slide", "fade", "flip"
    pub page_animation: String,

    /// Show page numbers in paged mode
    pub show_page_numbers: bool,

    /// Auto-scroll speed (0 to disable, 1-10 for speed)
    pub auto_scroll_speed: u8,

    /// Page gap in pixels for multi-page view
    pub page_gap: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatsConfig {
    /// EPUB settings
    pub epub: EpubConfig,

    /// PDF settings
    pub pdf: PdfConfig,

    /// Markdown settings
    pub markdown: MarkdownConfig,

    /// Plain text settings
    pub txt: TxtConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EpubConfig {
    /// Show images
    pub show_images: bool,

    /// Image display mode (inline, separate, off)
    pub image_mode: String,

    /// Parse CSS styles
    pub parse_css: bool,

    /// Honor font sizes from CSS
    pub honor_font_size: bool,

    /// Handle footnotes inline
    pub inline_footnotes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PdfConfig {
    /// Render mode (text, image, hybrid)
    pub render_mode: String,

    /// DPI for image rendering
    pub dpi: u32,

    /// Extract text layers
    pub extract_text: bool,

    /// Handle scanned documents with OCR
    pub ocr_enabled: bool,

    /// OCR language
    pub ocr_lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MarkdownConfig {
    /// Render tables
    pub tables: bool,

    /// Render task lists
    pub task_lists: bool,

    /// Strikethrough support
    pub strikethrough: bool,

    /// Footnotes support
    pub footnotes: bool,

    /// Smart punctuation
    pub smart_punctuation: bool,

    /// Syntax highlighting for code blocks
    pub syntax_highlighting: bool,

    /// Syntax theme
    pub syntax_theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TxtConfig {
    /// Encoding detection
    pub auto_encoding: bool,

    /// Default encoding
    pub default_encoding: String,

    /// Paragraph detection (blank_line, indent)
    pub paragraph_mode: String,

    /// Line ending normalization
    pub normalize_line_endings: bool,
}
