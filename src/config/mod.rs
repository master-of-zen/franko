//! Configuration system for Franko
//!
//! Provides a comprehensive, layered configuration system with:
//! - TOML-based configuration files
//! - Environment variable overrides
//! - Sensible defaults
//! - Customizable keybindings
//! - Multiple themes

mod defaults;
pub mod keybindings;
mod loader;
mod structs;
pub mod theme;

// Re-export main types
pub use keybindings::Keybindings;
pub use loader::{config_path, handle_command, init_config};
pub use structs::Config;
pub use theme::ThemeConfig;
