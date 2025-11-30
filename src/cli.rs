//! Command-line interface definitions

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "franko")]
#[command(author = "Master of Zen")]
#[command(version)]
#[command(about = "The ultimate book reader for power users", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Path to custom configuration file
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Read a book file
    Read {
        /// Path to the book file
        file: PathBuf,

        /// Interface to use (overrides config)
        #[arg(short, long, value_enum)]
        interface: Option<Interface>,
    },

    /// Library management commands
    #[command(subcommand)]
    Library(LibraryCommand),

    /// Configuration commands
    #[command(subcommand)]
    Config(ConfigCommand),

    /// Start the web server
    #[cfg(feature = "web")]
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Address to bind to
        #[arg(short, long, default_value = "127.0.0.1")]
        bind: String,
    },

    /// Initialize configuration file with defaults
    Init,
}

#[derive(Subcommand)]
pub enum LibraryCommand {
    /// Add a book or folder of books to the library
    Add {
        /// Path to book file or folder containing books
        path: PathBuf,

        /// Tags to add to all books
        #[arg(short, long)]
        tags: Vec<String>,

        /// Recursively scan subfolders
        #[arg(short, long, default_value = "true")]
        recursive: bool,
    },

    /// Remove a book from the library
    Remove {
        /// Book ID or path
        id: String,
    },

    /// List all books in the library
    List {
        /// Filter by format
        #[arg(short, long)]
        format: Option<String>,

        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Filter by reading status
        #[arg(short, long, value_enum)]
        status: Option<ReadingStatus>,

        /// Sort by field
        #[arg(long, default_value = "title")]
        sort: String,

        /// Output format
        #[arg(long, default_value = "table")]
        output: OutputFormat,
    },

    /// Search the library
    Search {
        /// Search query
        query: String,
    },

    /// Show book details
    Info {
        /// Book ID
        id: String,
    },

    /// Import books from a directory
    Import {
        /// Directory to import from
        path: PathBuf,

        /// Recursively scan subdirectories
        #[arg(short, long)]
        recursive: bool,
    },

    /// Export library data
    Export {
        /// Output file path
        output: PathBuf,

        /// Export format (json, csv)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Manage bookmarks
    #[command(subcommand)]
    Bookmark(BookmarkCommand),

    /// Manage annotations
    #[command(subcommand)]
    Annotation(AnnotationCommand),
}

#[derive(Subcommand)]
pub enum BookmarkCommand {
    /// List bookmarks for a book
    List {
        /// Book ID
        book_id: String,
    },

    /// Add a bookmark
    Add {
        /// Book ID
        book_id: String,

        /// Position (chapter:paragraph or page number)
        position: String,

        /// Bookmark name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Remove a bookmark
    Remove {
        /// Bookmark ID
        id: String,
    },

    /// Go to a bookmark
    Goto {
        /// Bookmark ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum AnnotationCommand {
    /// List annotations for a book
    List {
        /// Book ID
        book_id: String,
    },

    /// Add an annotation
    Add {
        /// Book ID
        book_id: String,

        /// Position (chapter:paragraph or page number)
        position: String,

        /// Annotation text
        text: String,

        /// Highlight color
        #[arg(short, long)]
        color: Option<String>,
    },

    /// Remove an annotation
    Remove {
        /// Annotation ID
        id: String,
    },

    /// Export annotations
    Export {
        /// Book ID
        book_id: String,

        /// Output file
        output: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Show current configuration
    Show,

    /// Get a specific configuration value
    Get {
        /// Configuration key
        key: String,
    },

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },

    /// Reset configuration to defaults
    Reset {
        /// Reset only a specific section
        #[arg(short, long)]
        section: Option<String>,
    },

    /// Edit configuration in $EDITOR
    Edit,

    /// List available themes
    Themes,

    /// List available keybinding presets
    Keybindings,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Interface {
    Tui,
    Web,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ReadingStatus {
    Unread,
    Reading,
    Finished,
    Abandoned,
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Csv,
    Plain,
}

impl std::fmt::Display for ReadingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadingStatus::Unread => write!(f, "unread"),
            ReadingStatus::Reading => write!(f, "reading"),
            ReadingStatus::Finished => write!(f, "finished"),
            ReadingStatus::Abandoned => write!(f, "abandoned"),
        }
    }
}
