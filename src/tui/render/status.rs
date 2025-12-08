//! Status bar and sidebar rendering

use crate::config::ThemeConfig;
use crate::tui::state::{AppState, Mode};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};

pub fn render_sidebar(frame: &mut Frame, state: &AppState, area: Rect, _theme: &ThemeConfig) {
    let block = Block::default()
        .title(" Table of Contents ")
        .borders(Borders::RIGHT);

    let items: Vec<ListItem> = state
        .book
        .content
        .chapters
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let title = ch.display_title();
            let style = if i == state.position.chapter {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(title).style(style)
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

pub fn render_progress_bar(frame: &mut Frame, state: &AppState, area: Rect, _theme: &ThemeConfig) {
    let progress = state.progress();
    let gauge = Gauge::default()
        .ratio(progress)
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
    frame.render_widget(gauge, area);
}

pub fn render_status_bar(frame: &mut Frame, state: &AppState, area: Rect, _theme: &ThemeConfig) {
    let chapter_info = if let Some(ch) = state.current_chapter() {
        ch.display_title()
    } else {
        "No chapter".to_string()
    };

    let position_info = format!(
        "Ch {}/{} | {} | Line {}/{}",
        state.position.chapter + 1,
        state.book.content.chapters.len(),
        state.progress_string(),
        state.position.scroll_offset + 1,
        state.total_lines,
    );

    let mode_str = match state.mode {
        Mode::Normal => "NORMAL",
        Mode::Command => "COMMAND",
        Mode::Search => "SEARCH",
        Mode::Help => "HELP",
        Mode::TableOfContents => "TOC",
        Mode::Bookmark => "BOOKMARKS",
        Mode::GoTo => "GOTO",
    };

    let left = format!(" {} ", chapter_info);
    let right = format!(" {} | {} ", mode_str, position_info);

    let status_width = area.width as usize;
    let padding = status_width.saturating_sub(left.len() + right.len());
    let status_text = format!("{}{}{}", left, " ".repeat(padding), right);

    let paragraph =
        Paragraph::new(status_text).style(Style::default().fg(Color::White).bg(Color::DarkGray));
    frame.render_widget(paragraph, area);
}
