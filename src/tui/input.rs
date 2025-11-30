//! Input handling for TUI

use super::event::{InputEvent, KeyInput};
use super::state::{AppState, Mode, MessageType};
use crate::config::Config;
use crossterm::event::{KeyCode, MouseEventKind};

/// Handle input events
pub fn handle_input(state: &mut AppState, event: InputEvent, config: &Config) {
    match event {
        InputEvent::Key(key) => handle_key(state, key, config),
        InputEvent::Mouse(mouse) => handle_mouse(state, mouse, config),
        InputEvent::Resize(w, h) => {
            state.terminal_size = (w, h);
            state.invalidate_cache();
        }
        InputEvent::Tick => {
            state.tick_message();
        }
    }
}

fn handle_key(state: &mut AppState, key: KeyInput, config: &Config) {
    match state.mode {
        Mode::Normal => handle_normal_mode(state, key, config),
        Mode::Command => handle_command_mode(state, key),
        Mode::Search => handle_search_mode(state, key),
        Mode::Help => handle_help_mode(state, key),
        Mode::TableOfContents => handle_toc_mode(state, key),
        Mode::Bookmark => handle_bookmark_mode(state, key),
        Mode::GoTo => handle_goto_mode(state, key),
    }
}

fn handle_normal_mode(state: &mut AppState, key: KeyInput, config: &Config) {
    // Check for quit first
    if key.code == KeyCode::Char('q') && !key.is_ctrl() {
        state.should_quit = true;
        return;
    }

    if key.code == KeyCode::Char('c') && key.is_ctrl() {
        state.should_quit = true;
        return;
    }

    // Navigation
    match key.code {
        // Vim-style navigation
        KeyCode::Char('j') | KeyCode::Down => state.scroll_down(1),
        KeyCode::Char('k') | KeyCode::Up => state.scroll_up(1),
        KeyCode::Char('h') | KeyCode::Left => state.prev_chapter(),
        KeyCode::Char('l') | KeyCode::Right => state.next_chapter(),

        // Page navigation
        KeyCode::Char(' ') | KeyCode::PageDown => state.page_down(),
        KeyCode::PageUp => state.page_up(),

        // Ctrl+navigation
        KeyCode::Char('f') if key.is_ctrl() => state.page_down(),
        KeyCode::Char('b') if key.is_ctrl() => state.page_up(),
        KeyCode::Char('d') if key.is_ctrl() => state.half_page_down(),
        KeyCode::Char('u') if key.is_ctrl() => state.half_page_up(),

        // Go to top/bottom
        KeyCode::Char('g') => state.go_to_top(),
        KeyCode::Char('G') => state.go_to_bottom(),
        KeyCode::Home => state.go_to_top(),
        KeyCode::End => state.go_to_bottom(),

        // Chapter navigation
        KeyCode::Char('n') => state.next_chapter(),
        KeyCode::Char('N') | KeyCode::Char('p') => state.prev_chapter(),
        KeyCode::Char(']') => state.next_chapter(),
        KeyCode::Char('[') => state.prev_chapter(),

        // Search
        KeyCode::Char('/') => {
            state.set_mode(Mode::Search);
            state.search.query.clear();
            state.search.cursor = 0;
        }
        KeyCode::Char('n') if state.search.active => state.search_next(),

        // Bookmarks
        KeyCode::Char('m') => {
            state.add_bookmark(None);
        }
        KeyCode::Char('\'') => {
            state.set_mode(Mode::Bookmark);
        }

        // UI toggles
        KeyCode::Char('s') => state.toggle_sidebar(),
        KeyCode::Char('S') => state.toggle_status_bar(),
        KeyCode::Char('L') => state.toggle_line_numbers(),
        KeyCode::Char('f') if !key.is_ctrl() => state.toggle_fullscreen(),

        // Help
        KeyCode::Char('?') | KeyCode::F(1) => {
            state.set_mode(Mode::Help);
        }

        // Table of contents
        KeyCode::Char('t') => {
            state.set_mode(Mode::TableOfContents);
        }

        // Command mode
        KeyCode::Char(':') => {
            state.set_mode(Mode::Command);
            state.command_buffer.clear();
            state.command_cursor = 0;
        }

        // Go to line
        KeyCode::Char('g') if key.is_ctrl() => {
            state.set_mode(Mode::GoTo);
            state.command_buffer.clear();
        }

        // Theme toggle
        KeyCode::Char('T') => {
            // Cycle through themes
            state.theme = match state.theme.as_str() {
                "dark" => "light".to_string(),
                "light" => "sepia".to_string(),
                _ => "dark".to_string(),
            };
            state.show_message(format!("Theme: {}", state.theme), MessageType::Info);
        }

        // Refresh
        KeyCode::Char('r') if key.is_ctrl() => {
            state.invalidate_cache();
            state.show_message("Refreshed".to_string(), MessageType::Info);
        }

        _ => {}
    }
}

fn handle_command_mode(state: &mut AppState, key: KeyInput) {
    match key.code {
        KeyCode::Esc => {
            state.return_to_previous_mode();
            state.command_buffer.clear();
        }
        KeyCode::Enter => {
            let command = state.command_buffer.clone();
            state.return_to_previous_mode();
            execute_command(state, &command);
            state.command_buffer.clear();
        }
        KeyCode::Backspace => {
            if state.command_cursor > 0 {
                state.command_cursor -= 1;
                state.command_buffer.remove(state.command_cursor);
            }
        }
        KeyCode::Delete => {
            if state.command_cursor < state.command_buffer.len() {
                state.command_buffer.remove(state.command_cursor);
            }
        }
        KeyCode::Left => {
            state.command_cursor = state.command_cursor.saturating_sub(1);
        }
        KeyCode::Right => {
            state.command_cursor = (state.command_cursor + 1).min(state.command_buffer.len());
        }
        KeyCode::Home => {
            state.command_cursor = 0;
        }
        KeyCode::End => {
            state.command_cursor = state.command_buffer.len();
        }
        KeyCode::Char(c) => {
            state.command_buffer.insert(state.command_cursor, c);
            state.command_cursor += 1;
        }
        _ => {}
    }
}

fn handle_search_mode(state: &mut AppState, key: KeyInput) {
    match key.code {
        KeyCode::Esc => {
            state.return_to_previous_mode();
            state.search.clear();
        }
        KeyCode::Enter => {
            state.return_to_previous_mode();
            state.perform_search();
        }
        KeyCode::Backspace => {
            if state.search.cursor > 0 {
                state.search.cursor -= 1;
                state.search.query.remove(state.search.cursor);
            }
        }
        KeyCode::Left => {
            state.search.cursor = state.search.cursor.saturating_sub(1);
        }
        KeyCode::Right => {
            state.search.cursor = (state.search.cursor + 1).min(state.search.query.len());
        }
        KeyCode::Char(c) => {
            state.search.query.insert(state.search.cursor, c);
            state.search.cursor += 1;
        }
        _ => {}
    }
}

fn handle_help_mode(state: &mut AppState, key: KeyInput) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::F(1) => {
            state.return_to_previous_mode();
        }
        KeyCode::Char('j') | KeyCode::Down => state.scroll_down(1),
        KeyCode::Char('k') | KeyCode::Up => state.scroll_up(1),
        _ => {}
    }
}

fn handle_toc_mode(state: &mut AppState, key: KeyInput) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('t') => {
            state.return_to_previous_mode();
        }
        KeyCode::Char('j') | KeyCode::Down => {
            // Navigate TOC entries
        }
        KeyCode::Char('k') | KeyCode::Up => {
            // Navigate TOC entries
        }
        KeyCode::Enter => {
            // Go to selected TOC entry
            state.return_to_previous_mode();
        }
        _ => {}
    }
}

fn handle_bookmark_mode(state: &mut AppState, key: KeyInput) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('\'') => {
            state.return_to_previous_mode();
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            let index = c.to_digit(10).unwrap() as usize;
            if index > 0 && index <= state.bookmarks.len() {
                state.go_to_bookmark(index - 1);
                state.return_to_previous_mode();
            }
        }
        KeyCode::Enter => {
            if !state.bookmarks.is_empty() {
                state.go_to_bookmark(0);
            }
            state.return_to_previous_mode();
        }
        _ => {}
    }
}

fn handle_goto_mode(state: &mut AppState, key: KeyInput) {
    match key.code {
        KeyCode::Esc => {
            state.return_to_previous_mode();
            state.command_buffer.clear();
        }
        KeyCode::Enter => {
            if let Ok(line) = state.command_buffer.parse::<usize>() {
                state.position.scroll_offset = line.saturating_sub(1);
            }
            state.return_to_previous_mode();
            state.command_buffer.clear();
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            state.command_buffer.push(c);
        }
        KeyCode::Backspace => {
            state.command_buffer.pop();
        }
        _ => {}
    }
}

fn handle_mouse(state: &mut AppState, mouse: super::event::MouseInput, _config: &Config) {
    match mouse.kind {
        MouseEventKind::ScrollUp => state.scroll_up(3),
        MouseEventKind::ScrollDown => state.scroll_down(3),
        MouseEventKind::Down(_) => {
            // Handle click - could be for selecting text, clicking links, etc.
        }
        _ => {}
    }
}

fn execute_command(state: &mut AppState, command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    match parts.first().map(|s| *s) {
        Some("q") | Some("quit") | Some("exit") => {
            state.should_quit = true;
        }
        Some("w") | Some("write") | Some("save") => {
            // Save progress
            state.dirty = false;
            state.show_message("Progress saved".to_string(), MessageType::Success);
        }
        Some("wq") => {
            state.dirty = false;
            state.should_quit = true;
        }
        Some("chapter") | Some("ch") => {
            if let Some(num) = parts.get(1) {
                if let Ok(n) = num.parse::<usize>() {
                    if n > 0 && n <= state.book.content.chapters.len() {
                        state.position.chapter = n - 1;
                        state.position.scroll_offset = 0;
                        state.invalidate_cache();
                        state.show_message(format!("Chapter {}", n), MessageType::Info);
                    } else {
                        state.show_message("Invalid chapter number".to_string(), MessageType::Error);
                    }
                }
            }
        }
        Some("theme") => {
            if let Some(theme_name) = parts.get(1) {
                state.theme = theme_name.to_string();
                state.show_message(format!("Theme: {}", theme_name), MessageType::Info);
            }
        }
        Some("set") => {
            // Handle set commands
            if parts.len() >= 2 {
                match parts.get(1).map(|s| *s) {
                    Some("number") | Some("nu") => {
                        state.show_line_numbers = true;
                        state.invalidate_cache();
                    }
                    Some("nonumber") | Some("nonu") => {
                        state.show_line_numbers = false;
                        state.invalidate_cache();
                    }
                    Some("sidebar") => {
                        state.show_sidebar = true;
                        state.invalidate_cache();
                    }
                    Some("nosidebar") => {
                        state.show_sidebar = false;
                        state.invalidate_cache();
                    }
                    _ => {
                        state.show_message(format!("Unknown option: {:?}", parts.get(1)), MessageType::Error);
                    }
                }
            }
        }
        Some("help") | Some("h") => {
            state.set_mode(Mode::Help);
        }
        Some("toc") => {
            state.set_mode(Mode::TableOfContents);
        }
        Some("bookmark") | Some("bm") => {
            let name = if parts.len() > 1 {
                Some(parts[1..].join(" "))
            } else {
                None
            };
            state.add_bookmark(name);
        }
        Some(cmd) if cmd.chars().all(|c| c.is_ascii_digit()) => {
            // Go to line number
            if let Ok(line) = cmd.parse::<usize>() {
                state.position.scroll_offset = line.saturating_sub(1);
                state.show_message(format!("Line {}", line), MessageType::Info);
            }
        }
        None => {}
        Some(cmd) => {
            state.show_message(format!("Unknown command: {}", cmd), MessageType::Error);
        }
    }
}
