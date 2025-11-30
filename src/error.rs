//! Error types for Franko

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrankoError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Book not found: {0}")]
    BookNotFound(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Library error: {0}")]
    Library(String),
    
    #[error("Search error: {0}")]
    Search(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("{0}")]
    Other(#[from] anyhow::Error),

    #[cfg(feature = "epub")]
    #[error("EPUB error: {0}")]
    Epub(String),

    #[cfg(feature = "pdf")]
    #[error("PDF error: {0}")]
    Pdf(String),

    #[cfg(feature = "tui")]
    #[error("TUI error: {0}")]
    Tui(String),

    #[cfg(feature = "web")]
    #[error("Web server error: {0}")]
    Web(String),
    
    #[cfg(feature = "search")]
    #[error("Tantivy error: {0}")]
    Tantivy(#[from] tantivy::TantivyError),
}

pub type Result<T> = std::result::Result<T, FrankoError>;
