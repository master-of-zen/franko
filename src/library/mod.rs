//! Library management for Franko
//!
//! Handles book collection, progress tracking, bookmarks, and annotations

mod database;

pub use database::{Annotation, BookStats, Bookmark, Library, LibraryEntry, LibraryStats};

use crate::cli::LibraryCommand;
use crate::config::Config;
use anyhow::Result;

/// Supported book extensions
const BOOK_EXTENSIONS: &[&str] = &["epub", "pdf", "md", "markdown", "txt", "text"];

/// Handle library commands
pub async fn handle_command(cmd: LibraryCommand, config: &Config) -> Result<()> {
    let mut library = Library::new(config)?;

    match cmd {
        LibraryCommand::Add {
            path,
            tags,
            recursive,
        } => {
            if path.is_dir() {
                // Add all books from directory
                let mut added = 0;
                let mut failed = 0;

                let walker = if recursive {
                    walkdir::WalkDir::new(&path)
                } else {
                    walkdir::WalkDir::new(&path).max_depth(1)
                };

                for entry in walker.into_iter().filter_map(|e| e.ok()) {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                            if BOOK_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                                match library.add_book(file_path, Some(tags.clone())) {
                                    Ok(entry) => {
                                        println!("  ✓ {}", entry.metadata.title);
                                        added += 1;
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "  ✗ {:?}: {}",
                                            file_path.file_name().unwrap_or_default(),
                                            e
                                        );
                                        failed += 1;
                                    }
                                }
                            }
                        }
                    }
                }

                println!("\nAdded {} books ({} failed)", added, failed);
                library.save()?;
            } else {
                // Single file
                let entry = library.add_book(&path, Some(tags))?;
                println!(
                    "Added: {} by {}",
                    entry.metadata.title,
                    entry.metadata.authors_string()
                );
                library.save()?;
            }
        }
        LibraryCommand::Remove { id } => {
            library.remove_book(&id)?;
            println!("Removed book: {}", id);
            library.save()?;
        }
        LibraryCommand::List {
            format,
            tag,
            status,
            sort: _,
            output,
        } => {
            let books = library.list_books(format.as_deref(), tag.as_deref(), status)?;

            match output {
                crate::cli::OutputFormat::Table => {
                    print_books_table(&books);
                }
                crate::cli::OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&books)?);
                }
                crate::cli::OutputFormat::Csv => {
                    println!("id,title,author,format,progress");
                    for book in books {
                        println!(
                            "{},{},{},{},{:.1}",
                            book.id,
                            book.metadata.title,
                            book.metadata.authors_string(),
                            book.format,
                            book.progress * 100.0
                        );
                    }
                }
                crate::cli::OutputFormat::Plain => {
                    for book in books {
                        println!(
                            "{} - {} by {}",
                            book.id,
                            book.metadata.title,
                            book.metadata.authors_string()
                        );
                    }
                }
            }
        }
        LibraryCommand::Search { query } => {
            let results = library.search(&query);
            if results.is_empty() {
                println!("No results found for: {}", query);
            } else {
                print_books_table(&results);
            }
        }
        LibraryCommand::Info { id } => match library.get_book(&id) {
            Some(entry) => {
                println!("Title:       {}", entry.metadata.title);
                println!("Author(s):   {}", entry.metadata.authors_string());
                if let Some(publisher) = &entry.metadata.publisher {
                    println!("Publisher:   {}", publisher);
                }
                if let Some(published) = &entry.metadata.published {
                    println!("Published:   {}", published);
                }
                if let Some(lang) = &entry.metadata.language {
                    println!("Language:    {}", lang);
                }
                println!("Format:      {}", entry.format);
                println!("Progress:    {:.1}%", entry.progress * 100.0);
                println!("Path:        {}", entry.path.display());
                if !entry.tags.is_empty() {
                    println!("Tags:        {}", entry.tags.join(", "));
                }
                if let Some(desc) = &entry.metadata.description {
                    println!("\nDescription:");
                    println!("{}", textwrap::wrap(desc, 80).join("\n"));
                }
            }
            None => println!("Book not found: {}", id),
        },
        LibraryCommand::Import { path, recursive } => {
            let count = library.import_directory(&path, recursive)?;
            println!("Imported {} books", count);
            library.save()?;
        }
        LibraryCommand::Export { output, format } => {
            library.export(&output, &format)?;
            println!("Exported library to: {}", output.display());
        }
        LibraryCommand::Bookmark(bm_cmd) => {
            handle_bookmark_command(bm_cmd, &mut library)?;
        }
        LibraryCommand::Annotation(ann_cmd) => {
            handle_annotation_command(ann_cmd, &mut library)?;
        }
    }

    Ok(())
}

fn handle_bookmark_command(cmd: crate::cli::BookmarkCommand, library: &mut Library) -> Result<()> {
    use crate::cli::BookmarkCommand;

    match cmd {
        BookmarkCommand::List { book_id } => {
            let bookmarks = library.get_bookmarks(&book_id)?;
            if bookmarks.is_empty() {
                println!("No bookmarks found");
            } else {
                for (i, bm) in bookmarks.iter().enumerate() {
                    println!(
                        "{}. {} (Ch.{}, Block {})",
                        i + 1,
                        bm.name,
                        bm.chapter + 1,
                        bm.block + 1
                    );
                }
            }
        }
        BookmarkCommand::Add {
            book_id,
            position,
            name,
        } => {
            let (chapter, block) = parse_position(&position)?;
            let bookmark = library.add_bookmark(&book_id, name, chapter, block)?;
            println!("Added bookmark: {}", bookmark.name);
            library.save()?;
        }
        BookmarkCommand::Remove { id } => {
            // Parse book_id:bookmark_id format
            let parts: Vec<&str> = id.split(':').collect();
            if parts.len() == 2 {
                library.remove_bookmark(parts[0], parts[1])?;
                println!("Removed bookmark");
                library.save()?;
            } else {
                anyhow::bail!("Invalid bookmark ID format. Use book_id:bookmark_id");
            }
        }
        BookmarkCommand::Goto { id } => {
            println!("Opening bookmark: {}", id);
            // TODO: Integrate with reader
        }
    }

    Ok(())
}

fn handle_annotation_command(
    cmd: crate::cli::AnnotationCommand,
    library: &mut Library,
) -> Result<()> {
    use crate::cli::AnnotationCommand;

    match cmd {
        AnnotationCommand::List { book_id } => {
            let annotations = library.get_annotations(&book_id)?;
            if annotations.is_empty() {
                println!("No annotations found");
            } else {
                for (i, ann) in annotations.iter().enumerate() {
                    println!("{}. \"{}\"", i + 1, truncate(&ann.text, 50));
                    if let Some(note) = &ann.note {
                        println!("   Note: {}", note);
                    }
                    println!("   (Ch.{}, Block {})", ann.chapter + 1, ann.block + 1);
                }
            }
        }
        AnnotationCommand::Add {
            book_id,
            position,
            text,
            color,
        } => {
            let (chapter, block) = parse_position(&position)?;
            let _annotation =
                library.add_annotation(&book_id, text, None, chapter, block, color)?;
            println!("Added annotation");
            library.save()?;
        }
        AnnotationCommand::Remove { id } => {
            let parts: Vec<&str> = id.split(':').collect();
            if parts.len() == 2 {
                library.remove_annotation(parts[0], parts[1])?;
                println!("Removed annotation");
                library.save()?;
            } else {
                anyhow::bail!("Invalid annotation ID format. Use book_id:annotation_id");
            }
        }
        AnnotationCommand::Export { book_id, output } => {
            let annotations = library.get_annotations(&book_id)?;
            let content = serde_json::to_string_pretty(&annotations)?;
            std::fs::write(&output, content)?;
            println!("Exported annotations to: {}", output.display());
        }
    }

    Ok(())
}

fn parse_position(pos: &str) -> Result<(usize, usize)> {
    let parts: Vec<&str> = pos.split(':').collect();
    if parts.len() == 2 {
        let chapter = parts[0].parse::<usize>()?.saturating_sub(1);
        let block = parts[1].parse::<usize>()?.saturating_sub(1);
        Ok((chapter, block))
    } else if parts.len() == 1 {
        let chapter = parts[0].parse::<usize>()?.saturating_sub(1);
        Ok((chapter, 0))
    } else {
        anyhow::bail!("Invalid position format. Use chapter:block or just chapter")
    }
}

fn print_books_table(books: &[LibraryEntry]) {
    if books.is_empty() {
        println!("No books found");
        return;
    }

    // Calculate column widths
    let id_width = 8;
    let title_width = 40;
    let author_width = 25;
    let format_width = 6;
    let progress_width = 8;

    // Header
    println!(
        "{:<id_width$}  {:<title_width$}  {:<author_width$}  {:<format_width$}  {:>progress_width$}",
        "ID", "Title", "Author", "Format", "Progress",
        id_width = id_width,
        title_width = title_width,
        author_width = author_width,
        format_width = format_width,
        progress_width = progress_width,
    );
    println!(
        "{}",
        "-".repeat(id_width + title_width + author_width + format_width + progress_width + 8)
    );

    // Rows
    for book in books {
        println!(
            "{:<id_width$}  {:<title_width$}  {:<author_width$}  {:<format_width$}  {:>progress_width$}",
            truncate(&book.id, id_width),
            truncate(&book.metadata.title, title_width),
            truncate(&book.metadata.authors_string(), author_width),
            book.format.to_uppercase(),
            format!("{:.1}%", book.progress * 100.0),
            id_width = id_width,
            title_width = title_width,
            author_width = author_width,
            format_width = format_width,
            progress_width = progress_width,
        );
    }

    println!("\nTotal: {} books", books.len());
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}
