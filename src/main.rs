//! Franko - The Ultimate Book Reader
//!
//! A powerful, configurable book reader with TUI and Web interfaces,
//! designed for power users who demand flexibility and control.

mod cli;
mod config;
mod error;
mod formats;
mod library;
mod reader;

#[cfg(feature = "search")]
mod search;

#[cfg(feature = "tui")]
mod tui;

#[cfg(feature = "web")]
mod web;

use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use cli::{Cli, Commands};
use config::Config;

fn setup_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose);

    let config = Config::load_or_default(cli.config.as_deref())?;
    info!("Franko v{} starting...", env!("CARGO_PKG_VERSION"));

    match cli.command {
        Commands::Read { file, interface } => {
            reader::read_book(&file, &config, interface).await?;
        }
        Commands::Library(lib_cmd) => {
            library::handle_command(lib_cmd, &config).await?;
        }
        Commands::Config(cfg_cmd) => {
            config::handle_command(cfg_cmd, &config)?;
        }
        #[cfg(feature = "web")]
        Commands::Serve { port, bind } => {
            web::serve(&config, bind, port).await?;
        }
        Commands::Init => {
            config::init_config()?;
            println!(
                "Configuration initialized at {}",
                config::config_path()?.display()
            );
        }
    }

    Ok(())
}
