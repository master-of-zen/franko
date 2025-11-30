//! Rendering for TUI

use super::state::{AppState, MessageType, Mode, RenderedLine};
use crate::config::{Config, ThemeConfig};
use crate::formats::ContentBlock;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};
use textwrap::wrap;

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
        render_sidebar(frame, state, chunks[0], theme);
        chunks[1]
    } else {
        main_chunks[0]
    };

    // Render main content
    render_content(frame, state, content_area, config);

    // Render status bar
    if state.show_status_bar && !state.fullscreen && main_chunks.len() > 2 {
        render_progress_bar(frame, state, main_chunks[1], theme);
        render_status_bar(frame, state, main_chunks[2], theme);
    }

    // Render overlays
    match state.mode {
        Mode::Help => render_help_overlay(frame, state, size),
        Mode::Command => render_command_line(frame, state, size),
        Mode::Search => render_search_line(frame, state, size),
        Mode::TableOfContents => render_toc_overlay(frame, state, size),
        Mode::Bookmark => render_bookmark_overlay(frame, state, size),
        Mode::GoTo => render_goto_line(frame, state, size),
        _ => {}
    }

    // Render message if any
    if let Some((ref msg, msg_type)) = state.message {
        render_message(frame, msg, msg_type, size);
    }
}

fn render_content(frame: &mut Frame, state: &mut AppState, area: Rect, config: &Config) {
    // Calculate margins
    let margin_left = config.tui.margin_left.min(area.width as usize / 4) as u16;
    let margin_right = config.tui.margin_right.min(area.width as usize / 4) as u16;
    
    let content_area = Rect {
        x: area.x + margin_left,
        y: area.y + 1,
        width: area.width.saturating_sub(margin_left + margin_right),
        height: area.height.saturating_sub(2),
    };

    // Build lines if cache is empty
    if state.lines_cache.is_empty() {
        build_lines_cache(state, content_area.width as usize, config);
    }

    // Get visible lines
    let visible_height = content_area.height as usize;
    let start = state.position.scroll_offset;
    let end = (start + visible_height).min(state.lines_cache.len());

    let mut lines: Vec<Line> = Vec::new();

    for (i, rendered) in state.lines_cache[start..end].iter().enumerate() {
        let line_num = start + i + 1;
        let mut spans = Vec::new();

        // Line numbers
        if state.show_line_numbers {
            spans.push(Span::styled(
                format!("{:4} ", line_num),
                Style::default().fg(Color::DarkGray),
            ));
        }

        // Determine style based on content type
        let style = if rendered.is_heading {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if rendered.is_quote {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC)
        } else if rendered.is_code {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        // Apply search highlights
        if !rendered.highlights.is_empty() {
            let text = &rendered.text;
            let mut last_end = 0;

            for &(start, end) in &rendered.highlights {
                // Text before highlight
                if start > last_end {
                    spans.push(Span::styled(
                        text[last_end..start].to_string(),
                        style,
                    ));
                }
                // Highlighted text
                spans.push(Span::styled(
                    text[start..end].to_string(),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
                last_end = end;
            }
            // Text after last highlight
            if last_end < text.len() {
                spans.push(Span::styled(text[last_end..].to_string(), style));
            }
        } else {
            spans.push(Span::styled(rendered.text.clone(), style));
        }

        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, content_area);
}

fn build_lines_cache(state: &mut AppState, width: usize, config: &Config) {
    state.lines_cache.clear();

    let wrap_width = if config.tui.max_width > 0 && config.tui.max_width < width {
        config.tui.max_width
    } else {
        width.saturating_sub(if state.show_line_numbers { 6 } else { 0 })
    };

    // Extract search state before the loop to avoid borrow issues
    let search_active = state.search.active;
    let search_query = state.search.query.clone();

    if let Some(chapter) = state.current_chapter().cloned() {
        for (block_idx, block) in chapter.blocks.iter().enumerate() {
            match block {
                ContentBlock::Paragraph { text, .. } => {
                    let wrapped = wrap(text, wrap_width);
                    for line in wrapped {
                        state.lines_cache.push(RenderedLine {
                            text: line.to_string(),
                            block_index: block_idx,
                            is_heading: false,
                            heading_level: 0,
                            is_quote: false,
                            is_code: false,
                            highlights: find_highlights_with_query(&line, search_active, &search_query),
                        });
                    }
                    // Empty line after paragraph
                    state.lines_cache.push(RenderedLine {
                        text: String::new(),
                        block_index: block_idx,
                        is_heading: false,
                        heading_level: 0,
                        is_quote: false,
                        is_code: false,
                        highlights: Vec::new(),
                    });
                }
                ContentBlock::Heading { text, level } => {
                    // Add blank line before heading (if not first)
                    if block_idx > 0 {
                        state.lines_cache.push(RenderedLine {
                            text: String::new(),
                            block_index: block_idx,
                            is_heading: false,
                            heading_level: 0,
                            is_quote: false,
                            is_code: false,
                            highlights: Vec::new(),
                        });
                    }

                    let wrapped = wrap(text, wrap_width);
                    for line in wrapped {
                        state.lines_cache.push(RenderedLine {
                            text: line.to_string(),
                            block_index: block_idx,
                            is_heading: true,
                            heading_level: *level,
                            is_quote: false,
                            is_code: false,
                            highlights: find_highlights_with_query(&line, search_active, &search_query),
                        });
                    }

                    // Blank line after heading
                    state.lines_cache.push(RenderedLine {
                        text: String::new(),
                        block_index: block_idx,
                        is_heading: false,
                        heading_level: 0,
                        is_quote: false,
                        is_code: false,
                        highlights: Vec::new(),
                    });
                }
                ContentBlock::Quote { text, .. } => {
                    let wrapped = wrap(text, wrap_width.saturating_sub(2));
                    for line in wrapped {
                        state.lines_cache.push(RenderedLine {
                            text: format!("│ {}", line),
                            block_index: block_idx,
                            is_heading: false,
                            heading_level: 0,
                            is_quote: true,
                            is_code: false,
                            highlights: Vec::new(),
                        });
                    }
                    state.lines_cache.push(RenderedLine {
                        text: String::new(),
                        block_index: block_idx,
                        is_heading: false,
                        heading_level: 0,
                        is_quote: false,
                        is_code: false,
                        highlights: Vec::new(),
                    });
                }
                ContentBlock::Code { code, .. } => {
                    for line in code.lines() {
                        state.lines_cache.push(RenderedLine {
                            text: format!("  {}", line),
                            block_index: block_idx,
                            is_heading: false,
                            heading_level: 0,
                            is_quote: false,
                            is_code: true,
                            highlights: Vec::new(),
                        });
                    }
                    state.lines_cache.push(RenderedLine {
                        text: String::new(),
                        block_index: block_idx,
                        is_heading: false,
                        heading_level: 0,
                        is_quote: false,
                        is_code: false,
                        highlights: Vec::new(),
                    });
                }
                ContentBlock::Separator => {
                    state.lines_cache.push(RenderedLine {
                        text: "─".repeat(wrap_width.min(40)),
                        block_index: block_idx,
                        is_heading: false,
                        heading_level: 0,
                        is_quote: false,
                        is_code: false,
                        highlights: Vec::new(),
                    });
                    state.lines_cache.push(RenderedLine {
                        text: String::new(),
                        block_index: block_idx,
                        is_heading: false,
                        heading_level: 0,
                        is_quote: false,
                        is_code: false,
                        highlights: Vec::new(),
                    });
                }
                ContentBlock::List { ordered, items } => {
                    for (i, item) in items.iter().enumerate() {
                        let prefix = if *ordered {
                            format!("{}. ", i + 1)
                        } else {
                            "• ".to_string()
                        };
                        let wrapped = wrap(item, wrap_width.saturating_sub(prefix.len()));
                        for (j, line) in wrapped.iter().enumerate() {
                            let text = if j == 0 {
                                format!("{}{}", prefix, line)
                            } else {
                                format!("{}{}", " ".repeat(prefix.len()), line)
                            };
                            state.lines_cache.push(RenderedLine {
                                text,
                                block_index: block_idx,
                                is_heading: false,
                                heading_level: 0,
                                is_quote: false,
                                is_code: false,
                                highlights: Vec::new(),
                            });
                        }
                    }
                    state.lines_cache.push(RenderedLine {
                        text: String::new(),
                        block_index: block_idx,
                        is_heading: false,
                        heading_level: 0,
                        is_quote: false,
                        is_code: false,
                        highlights: Vec::new(),
                    });
                }
                _ => {
                    // For other block types, just get text
                    let text = block.text();
                    if !text.is_empty() {
                        let wrapped = wrap(&text, wrap_width);
                        for line in wrapped {
                            state.lines_cache.push(RenderedLine {
                                text: line.to_string(),
                                block_index: block_idx,
                                is_heading: false,
                                heading_level: 0,
                                is_quote: false,
                                is_code: false,
                                highlights: Vec::new(),
                            });
                        }
                        state.lines_cache.push(RenderedLine {
                            text: String::new(),
                            block_index: block_idx,
                            is_heading: false,
                            heading_level: 0,
                            is_quote: false,
                            is_code: false,
                            highlights: Vec::new(),
                        });
                    }
                }
            }
        }
    }

    state.total_lines = state.lines_cache.len();
}

fn find_highlights_with_query(text: &str, search_active: bool, search_query: &str) -> Vec<(usize, usize)> {
    if !search_active || search_query.is_empty() {
        return Vec::new();
    }

    let query_lower = search_query.to_lowercase();
    let text_lower = text.to_lowercase();
    let mut highlights = Vec::new();
    let mut start = 0;

    while let Some(pos) = text_lower[start..].find(&query_lower) {
        let absolute_pos = start + pos;
        highlights.push((absolute_pos, absolute_pos + search_query.len()));
        start = absolute_pos + 1;
    }

    highlights
}

#[allow(dead_code)]
fn find_highlights(text: &str, state: &AppState) -> Vec<(usize, usize)> {
    find_highlights_with_query(text, state.search.active, &state.search.query)
}

fn render_sidebar(frame: &mut Frame, state: &AppState, area: Rect, _theme: &ThemeConfig) {
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

fn render_progress_bar(frame: &mut Frame, state: &AppState, area: Rect, _theme: &ThemeConfig) {
    let progress = state.progress();
    let gauge = Gauge::default()
        .ratio(progress)
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray));
    frame.render_widget(gauge, area);
}

fn render_status_bar(frame: &mut Frame, state: &AppState, area: Rect, _theme: &ThemeConfig) {
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

    let paragraph = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    frame.render_widget(paragraph, area);
}

fn render_command_line(frame: &mut Frame, state: &AppState, area: Rect) {
    let y = area.height.saturating_sub(1);
    let command_area = Rect {
        x: 0,
        y,
        width: area.width,
        height: 1,
    };

    let text = format!(":{}", state.command_buffer);
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    
    frame.render_widget(Clear, command_area);
    frame.render_widget(paragraph, command_area);
}

fn render_search_line(frame: &mut Frame, state: &AppState, area: Rect) {
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
        format!(" [{}/{}]", state.search.current_result + 1, state.search.results.len())
    };

    let text = format!("/{}{}", state.search.query, results_info);
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    
    frame.render_widget(Clear, search_area);
    frame.render_widget(paragraph, search_area);
}

fn render_goto_line(frame: &mut Frame, state: &AppState, area: Rect) {
    let y = area.height.saturating_sub(1);
    let goto_area = Rect {
        x: 0,
        y,
        width: area.width,
        height: 1,
    };

    let text = format!("Go to line: {}", state.command_buffer);
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    
    frame.render_widget(Clear, goto_area);
    frame.render_widget(paragraph, goto_area);
}

fn render_help_overlay(frame: &mut Frame, _state: &AppState, area: Rect) {
    let width = 60.min(area.width.saturating_sub(4));
    let height = 20.min(area.height.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let help_area = Rect { x, y, width, height };

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

fn render_toc_overlay(frame: &mut Frame, state: &AppState, area: Rect) {
    let width = 50.min(area.width.saturating_sub(4));
    let height = (state.book.content.chapters.len() as u16 + 4).min(area.height.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let toc_area = Rect { x, y, width, height };

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

fn render_bookmark_overlay(frame: &mut Frame, state: &AppState, area: Rect) {
    let width = 50.min(area.width.saturating_sub(4));
    let height = (state.bookmarks.len() as u16 + 4).max(6).min(area.height.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let bookmark_area = Rect { x, y, width, height };

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

    let paragraph = Paragraph::new(format!(" {} ", msg))
        .style(Style::default().fg(Color::White).bg(color));
    
    frame.render_widget(paragraph, msg_area);
}
