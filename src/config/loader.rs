//! Configuration loading, saving, and command handling

use super::structs::Config;
use super::{keybindings, theme};
use crate::cli::ConfigCommand;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::info;

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
            std::process::Command::new(&editor).arg(&path).status()?;
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
