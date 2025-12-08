//! Overlay rendering (help, command line, search, TOC, bookmarks)

use crate::tui::state::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn render_command_line(frame: &mut Frame, state: &AppState, area: Rect) {
    let y = area.height.saturating_sub(1);
    let command_area = Rect {
        x: 0,
        y,
        width: area.width,
        height: 1,
    };

    let text = format!(":{}", state.command_buffer);
    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White).bg(Color::Black));

    frame.render_widget(Clear, command_area);
    frame.render_widget(paragraph, command_area);
}

pub fn render_search_line(frame: &mut Frame, state: &AppState, area: Rect) {
    let y = area.height.saturating_sub(1);
    let search_area = Rect {
        x: 0,
        y,
        width: area.width,
        height: 1,
    };

    let results_info = if state.search.results.is_empty() {
        String::new()
    } else {
        format!(
            " [{}/{}]",
            state.search.current_result + 1,
            state.search.results.len()
        )
    };

    let text = format!("/{}{}", state.search.query, results_info);
    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White).bg(Color::Black));

    frame.render_widget(Clear, search_area);
    frame.render_widget(paragraph, search_area);
}

pub fn render_goto_line(frame: &mut Frame, state: &AppState, area: Rect) {
    let y = area.height.saturating_sub(1);
    let goto_area = Rect {
        x: 0,
        y,
        width: area.width,
        height: 1,
    };

    let text = format!("Go to line: {}", state.command_buffer);
    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White).bg(Color::Black));

    frame.render_widget(Clear, goto_area);
    frame.render_widget(paragraph, goto_area);
}

pub fn render_help_overlay(frame: &mut Frame, _state: &AppState, area: Rect) {
    let width = 60.min(area.width.saturating_sub(4));
    let height = 20.min(area.height.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let help_area = Rect {
        x,
        y,
        width,
        height,
    };

    let help_text = vec![
        "",
        "  Navigation",
        "  ──────────",
        "  j/k, ↑/↓     Scroll up/down",
        "  h/l, ←/→     Previous/next chapter",
        "  Space, PgDn  Page down",
        "  PgUp         Page up",
        "  g/G          Go to top/bottom",
        "  Ctrl+d/u     Half page down/up",
        "",
        "  Commands",
        "  ────────",
        "  /            Search",
        "  n/N          Next/prev search result",
        "  :            Command mode",
        "  m            Add bookmark",
        "  '            List bookmarks",
        "  t            Table of contents",
        "  s/S          Toggle sidebar/status bar",
        "  f            Toggle fullscreen",
        "  T            Cycle themes",
        "  q            Quit",
        "",
        "  Press ? or Esc to close",
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(help_text.join("\n"))
        .block(block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(Clear, help_area);
    frame.render_widget(paragraph, help_area);
}

pub fn render_toc_overlay(frame: &mut Frame, state: &AppState, area: Rect) {
    let width = 50.min(area.width.saturating_sub(4));
    let height = (state.book.content.chapters.len() as u16 + 4).min(area.height.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let toc_area = Rect {
        x,
        y,
        width,
        height,
    };

    let items: Vec<ListItem> = state
        .book
        .content
        .chapters
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let style = if i == state.position.chapter {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!(" {}. {}", i + 1, ch.display_title())).style(style)
        })
        .collect();

    let block = Block::default()
        .title(" Table of Contents ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let list = List::new(items).block(block);

    frame.render_widget(Clear, toc_area);
    frame.render_widget(list, toc_area);
}

pub fn render_bookmark_overlay(frame: &mut Frame, state: &AppState, area: Rect) {
    let width = 50.min(area.width.saturating_sub(4));
    let height = (state.bookmarks.len() as u16 + 4)
        .max(6)
        .min(area.height.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let bookmark_area = Rect {
        x,
        y,
        width,
        height,
    };

    let items: Vec<ListItem> = if state.bookmarks.is_empty() {
        vec![ListItem::new(" No bookmarks yet. Press 'm' to add one.")]
    } else {
        state
            .bookmarks
            .iter()
            .enumerate()
            .map(|(i, bm)| {
                ListItem::new(format!(" {}. {}", i + 1, bm.name))
                    .style(Style::default().fg(Color::White))
            })
            .collect()
    };

    let block = Block::default()
        .title(" Bookmarks ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let list = List::new(items).block(block);

    frame.render_widget(Clear, bookmark_area);
    frame.render_widget(list, bookmark_area);
}
