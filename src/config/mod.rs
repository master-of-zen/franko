//! Configuration system for Franko
//!
//! Provides a comprehensive, layered configuration system with:
//! - TOML-based configuration files
//! - Environment variable overrides
//! - Sensible defaults
//! - Customizable keybindings
//! - Multiple themes

mod keybindings;
mod theme;

pub use keybindings::{Action, Keybindings};
pub use theme::ThemeConfig;

use crate::cli::ConfigCommand;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;

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
    pub keybindings: Keybindings,

    /// Theme configuration
    pub theme: ThemeConfig,
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

// Default implementations

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            tui: TuiConfig::default(),
            web: WebConfig::default(),
            library: LibraryConfig::default(),
            reader: ReaderConfig::default(),
            formats: FormatsConfig::default(),
            keybindings: Keybindings::default(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_interface: "tui".to_string(),
            data_dir: None,
            auto_save: true,
            auto_save_interval: 30,
            logging: true,
            log_level: "info".to_string(),
            language: "en".to_string(),
        }
    }
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            mouse_support: true,
            unicode: true,
            line_numbers: false,
            status_bar: true,
            status_bar_position: "bottom".to_string(),
            progress_bar: true,
            animations: true,
            animation_speed: "normal".to_string(),
            scrolloff: 5,
            smooth_scroll: true,
            dim_unfocused: true,
            show_sidebar: false,
            sidebar_width: 25,
            tab_size: 4,
            wrap_mode: "word".to_string(),
            max_width: 80,
            margin_left: 4,
            margin_right: 4,
        }
    }
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            bind: "127.0.0.1".to_string(),
            https: false,
            tls_cert: None,
            tls_key: None,
            auth_enabled: false,
            auth_method: "basic".to_string(),
            cors: true,
            cors_origins: vec!["*".to_string()],
            compression: true,
            static_dir: None,
            custom_css: None,
            custom_js: None,
            page_size: 1000,
            dark_mode: true,
            font_family: "Georgia, serif".to_string(),
            font_size: 18,
            line_height: 1.8,
            open_browser: true,
        }
    }
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            database_path: None,
            books_dir: None,
            watch_enabled: false,
            extract_covers: true,
            covers_dir: None,
            cover_max_width: 300,
            search_enabled: true,
            search_index_dir: None,
            auto_index: true,
            backup_enabled: true,
            backup_count: 5,
        }
    }
}

impl Default for ReaderConfig {
    fn default() -> Self {
        Self {
            remember_position: true,
            show_reading_time: true,
            words_per_minute: 250,
            dictionary_enabled: false,
            dictionary_source: "wiktionary".to_string(),
            tts_enabled: false,
            tts_voice: "default".to_string(),
            tts_speed: 1.0,
            highlight_search: true,
            search_case_sensitive: false,
            search_regex: false,
            justify: true,
            hyphenation: true,
            hyphenation_lang: "en-us".to_string(),
            prefer_interface: "tui".to_string(),
        }
    }
}

impl Default for FormatsConfig {
    fn default() -> Self {
        Self {
            epub: EpubConfig::default(),
            pdf: PdfConfig::default(),
            markdown: MarkdownConfig::default(),
            txt: TxtConfig::default(),
        }
    }
}

impl Default for EpubConfig {
    fn default() -> Self {
        Self {
            show_images: true,
            image_mode: "inline".to_string(),
            parse_css: true,
            honor_font_size: false,
            inline_footnotes: true,
        }
    }
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            render_mode: "text".to_string(),
            dpi: 150,
            extract_text: true,
            ocr_enabled: false,
            ocr_lang: "eng".to_string(),
        }
    }
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            tables: true,
            task_lists: true,
            strikethrough: true,
            footnotes: true,
            smart_punctuation: true,
            syntax_highlighting: true,
            syntax_theme: "base16-ocean.dark".to_string(),
        }
    }
}

impl Default for TxtConfig {
    fn default() -> Self {
        Self {
            auto_encoding: true,
            default_encoding: "utf-8".to_string(),
            paragraph_mode: "blank_line".to_string(),
            normalize_line_endings: true,
        }
    }
}

impl Config {
    /// Load configuration from file, falling back to defaults
    pub fn load_or_default(path: Option<&Path>) -> Result<Self> {
        let config_path = match path {
            Some(p) => p.to_path_buf(),
            None => config_path()?,
        };

        if config_path.exists() {
            Self::load(&config_path)
        } else {
            info!("No config file found, using defaults");
            Ok(Self::default())
        }
    }

    /// Load configuration from a specific file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        info!("Loaded configuration from {}", path.display());
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        info!("Saved configuration to {}", path.display());
        Ok(())
    }

    /// Get the data directory, creating it if necessary
    pub fn data_dir(&self) -> Result<PathBuf> {
        let path = match &self.general.data_dir {
            Some(p) => p.clone(),
            None => dirs::data_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?
                .join("franko"),
        };

        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        Ok(path)
    }

    /// Get the library database path
    pub fn database_path(&self) -> Result<PathBuf> {
        match &self.library.database_path {
            Some(p) => Ok(p.clone()),
            None => Ok(self.data_dir()?.join("library.db")),
        }
    }
}

/// Get the default configuration file path
pub fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

    Ok(config_dir.join("franko").join("config.toml"))
}

/// Initialize a new configuration file with defaults
pub fn init_config() -> Result<()> {
    let path = config_path()?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let config = Config::default();
    config.save(&path)?;

    Ok(())
}

/// Handle configuration commands
pub fn handle_command(cmd: ConfigCommand, config: &Config) -> Result<()> {
    match cmd {
        ConfigCommand::Show => {
            let content = toml::to_string_pretty(config)?;
            println!("{}", content);
        }
        ConfigCommand::Get { key } => {
            // Simple dot-notation key lookup
            let value = get_config_value(config, &key)?;
            println!("{}", value);
        }
        ConfigCommand::Set { key, value } => {
            let path = config_path()?;
            let mut new_config = config.clone();
            set_config_value(&mut new_config, &key, &value)?;
            new_config.save(&path)?;
            println!("Set {} = {}", key, value);
        }
        ConfigCommand::Reset { section } => {
            let path = config_path()?;
            let new_config = if let Some(section) = section {
                reset_section(config, &section)?
            } else {
                Config::default()
            };
            new_config.save(&path)?;
            println!("Configuration reset");
        }
        ConfigCommand::Edit => {
            let path = config_path()?;
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
            std::process::Command::new(&editor)
                .arg(&path)
                .status()?;
        }
        ConfigCommand::Themes => {
            println!("Available themes:");
            for theme in theme::BUILTIN_THEMES {
                println!("  - {}", theme);
            }
        }
        ConfigCommand::Keybindings => {
            println!("Available keybinding presets:");
            for preset in keybindings::PRESETS {
                println!("  - {}", preset);
            }
        }
    }

    Ok(())
}

fn get_config_value(config: &Config, key: &str) -> Result<String> {
    // Convert config to TOML value for dynamic access
    let value = toml::Value::try_from(config)?;
    
    let parts: Vec<&str> = key.split('.').collect();
    let mut current = &value;

    for part in parts {
        current = current
            .get(part)
            .ok_or_else(|| anyhow::anyhow!("Configuration key not found: {}", key))?;
    }

    Ok(format!("{}", current))
}

fn set_config_value(config: &mut Config, key: &str, value: &str) -> Result<()> {
    // This is a simplified implementation - in production you'd want proper type handling
    let mut toml_value = toml::Value::try_from(config.clone())?;
    
    let parts: Vec<&str> = key.split('.').collect();
    let mut current = &mut toml_value;

    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            // Last part - set the value
            if let Some(table) = current.as_table_mut() {
                // Try to parse as different types
                let new_value = if value == "true" {
                    toml::Value::Boolean(true)
                } else if value == "false" {
                    toml::Value::Boolean(false)
                } else if let Ok(n) = value.parse::<i64>() {
                    toml::Value::Integer(n)
                } else if let Ok(f) = value.parse::<f64>() {
                    toml::Value::Float(f)
                } else {
                    toml::Value::String(value.to_string())
                };
                
                table.insert(part.to_string(), new_value);
            }
        } else {
            current = current
                .get_mut(*part)
                .ok_or_else(|| anyhow::anyhow!("Configuration key not found: {}", key))?;
        }
    }

    *config = toml_value.try_into()?;
    Ok(())
}

fn reset_section(config: &Config, section: &str) -> Result<Config> {
    let mut new_config = config.clone();
    let default = Config::default();

    match section {
        "general" => new_config.general = default.general,
        "tui" => new_config.tui = default.tui,
        "web" => new_config.web = default.web,
        "library" => new_config.library = default.library,
        "reader" => new_config.reader = default.reader,
        "formats" => new_config.formats = default.formats,
        "keybindings" => new_config.keybindings = default.keybindings,
        "theme" => new_config.theme = default.theme,
        _ => anyhow::bail!("Unknown section: {}", section),
    }

    Ok(new_config)
}
