//! Web interface for Franko
//!
//! A beautiful web-based reader using Axum

mod api;
mod handlers;
mod static_files;
mod templates;

use crate::config::Config;
use crate::formats::Book;
use anyhow::Result;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};
use tracing::info;

/// Application state shared across handlers
pub struct AppState {
    pub config: Config,
    pub library: Arc<RwLock<crate::library::Library>>,
    pub current_book: Option<Arc<RwLock<Book>>>,
}

/// Start the web server
pub async fn serve(config: &Config, bind: String, port: u16) -> Result<()> {
    // Initialize library
    let library = crate::library::Library::new(config)?;
    
    let state = Arc::new(AppState {
        config: config.clone(),
        library: Arc::new(RwLock::new(library)),
        current_book: None,
    });

    // Build router
    let app = Router::new()
        // Static files
        .route("/", get(handlers::index))
        .route("/static/*path", get(static_files::serve_static))
        // Reader routes
        .route("/read/:id", get(handlers::reader))
        .route("/read/:id/chapter/:chapter", get(handlers::reader_chapter))
        // Library routes
        .route("/library", get(handlers::library))
        .route("/book/:id", get(handlers::book_info))
        // Settings routes
        .route("/settings", get(handlers::settings))
        // API routes
        .nest("/api", api::router())
        // State
        .with_state(state)
        // Middleware
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr: SocketAddr = format!("{}:{}", bind, port).parse()?;
    info!("Starting web server at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Start the web server with a single book preloaded
pub async fn serve_book(book: Book, config: &Config) -> Result<()> {
    let library = crate::library::Library::new(config)?;
    let port = config.web.port;
    let bind = config.web.bind.clone();
    
    let state = Arc::new(AppState {
        config: config.clone(),
        library: Arc::new(RwLock::new(library)),
        current_book: Some(Arc::new(RwLock::new(book))),
    });

    // Build router for single-book mode
    let app = Router::new()
        .route("/", get(handlers::single_book_reader))
        .route("/static/*path", get(static_files::serve_static))
        .route("/chapter/:chapter", get(handlers::single_book_chapter))
        .nest("/api", api::router())
        .with_state(state)
        .layer(CompressionLayer::new());

    let addr: SocketAddr = format!("{}:{}", bind, port).parse()?;
    info!("Starting book reader at http://{}", addr);

    // Open browser if configured
    if config.web.open_browser {
        let url = format!("http://{}", addr);
        let _ = open::that(&url);
    }

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
