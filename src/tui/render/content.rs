//! Content rendering for the main reader area

use crate::config::Config;
use crate::tui::state::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::lines::build_lines_cache;

pub fn render_content(frame: &mut Frame, state: &mut AppState, area: Rect, config: &Config) {
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
                    spans.push(Span::styled(text[last_end..start].to_string(), style));
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

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });

    frame.render_widget(paragraph, content_area);
}
