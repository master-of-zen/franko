//! HTML templates for the web interface

mod base;
mod helpers;
mod index;
mod library;
mod reader;
mod settings;

pub use index::index;
pub use library::library;
pub use reader::{book_info, error, pdf_reader, reader};
pub use settings::settings;
