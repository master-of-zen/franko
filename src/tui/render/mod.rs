//! Rendering for TUI

mod content;
mod lines;
mod overlays;
mod status;

use super::state::{AppState, MessageType, Mode};
use crate::config::Config;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

/// Render the application
pub fn render(frame: &mut Frame, state: &mut AppState, config: &Config) {
    let theme = &config.theme;

    // Update terminal size
    let size = frame.area();
    state.terminal_size = (size.width, size.height);

    // Build layout
    let main_chunks = if state.show_status_bar && !state.fullscreen {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1), // Progress bar
                Constraint::Length(1), // Status bar
            ])
            .split(size)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1)])
            .split(size)
    };

    // Content area
    let content_area = if state.show_sidebar && !state.fullscreen {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(config.tui.sidebar_width),
                Constraint::Min(1),
            ])
            .split(main_chunks[0]);

        // Render sidebar
        status::render_sidebar(frame, state, chunks[0], theme);
        chunks[1]
    } else {
        main_chunks[0]
    };

    // Render main content
    content::render_content(frame, state, content_area, config);

    // Render status bar
    if state.show_status_bar && !state.fullscreen && main_chunks.len() > 2 {
        status::render_progress_bar(frame, state, main_chunks[1], theme);
        status::render_status_bar(frame, state, main_chunks[2], theme);
    }

    // Render overlays
    match state.mode {
        Mode::Help => overlays::render_help_overlay(frame, state, size),
        Mode::Command => overlays::render_command_line(frame, state, size),
        Mode::Search => overlays::render_search_line(frame, state, size),
        Mode::TableOfContents => overlays::render_toc_overlay(frame, state, size),
        Mode::Bookmark => overlays::render_bookmark_overlay(frame, state, size),
        Mode::GoTo => overlays::render_goto_line(frame, state, size),
        _ => {}
    }

    // Render message if any
    if let Some((ref msg, msg_type)) = state.message {
        render_message(frame, msg, msg_type, size);
    }
}

fn render_message(frame: &mut Frame, msg: &str, msg_type: MessageType, area: Rect) {
    let y = area.height.saturating_sub(2);
    let msg_area = Rect {
        x: 1,
        y,
        width: area.width.saturating_sub(2),
        height: 1,
    };

    let color = match msg_type {
        MessageType::Info => Color::Blue,
        MessageType::Success => Color::Green,
        MessageType::Warning => Color::Yellow,
        MessageType::Error => Color::Red,
    };

    let paragraph =
        Paragraph::new(format!(" {} ", msg)).style(Style::default().fg(Color::White).bg(color));

    frame.render_widget(paragraph, msg_area);
}

// Line cache building is used internally by the render module
