//! TUI (Terminal User Interface) for Franko
//!
//! A beautiful, configurable terminal interface for reading books

mod app;
mod components;
mod event;
mod input;
mod render;
mod state;

pub use app::run;
