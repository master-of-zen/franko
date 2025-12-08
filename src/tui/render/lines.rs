//! Line cache building and text wrapping

use crate::config::Config;
use crate::formats::ContentBlock;
use crate::tui::state::{AppState, RenderedLine};
use textwrap::wrap;

/// Build the lines cache for rendering
pub fn build_lines_cache(state: &mut AppState, width: usize, config: &Config) {
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
                    build_paragraph_lines(state, text, block_idx, wrap_width, search_active, &search_query);
                }
                ContentBlock::Heading { text, level } => {
                    build_heading_lines(state, text, *level, block_idx, wrap_width, search_active, &search_query);
                }
                ContentBlock::Quote { text, .. } => {
                    build_quote_lines(state, text, block_idx, wrap_width);
                }
                ContentBlock::Code { code, .. } => {
                    build_code_lines(state, code, block_idx);
                }
                ContentBlock::Separator => {
                    build_separator_lines(state, block_idx, wrap_width);
                }
                ContentBlock::List { ordered, items } => {
                    build_list_lines(state, items, *ordered, block_idx, wrap_width);
                }
                _ => {
                    // For other block types, just get text
                    let text = block.text();
                    if !text.is_empty() {
                        build_generic_lines(state, &text, block_idx, wrap_width);
                    }
                }
            }
        }
    }

    state.total_lines = state.lines_cache.len();
}

fn build_paragraph_lines(
    state: &mut AppState,
    text: &str,
    block_idx: usize,
    wrap_width: usize,
    search_active: bool,
    search_query: &str,
) {
    let wrapped = wrap(text, wrap_width);
    for line in wrapped {
        state.lines_cache.push(RenderedLine {
            text: line.to_string(),
            block_index: block_idx,
            is_heading: false,
            heading_level: 0,
            is_quote: false,
            is_code: false,
            highlights: find_highlights(&line, search_active, search_query),
        });
    }
    // Empty line after paragraph
    state.lines_cache.push(RenderedLine::empty(block_idx));
}

fn build_heading_lines(
    state: &mut AppState,
    text: &str,
    level: u8,
    block_idx: usize,
    wrap_width: usize,
    search_active: bool,
    search_query: &str,
) {
    // Add blank line before heading (if not first)
    if block_idx > 0 {
        state.lines_cache.push(RenderedLine::empty(block_idx));
    }

    let wrapped = wrap(text, wrap_width);
    for line in wrapped {
        state.lines_cache.push(RenderedLine {
            text: line.to_string(),
            block_index: block_idx,
            is_heading: true,
            heading_level: level,
            is_quote: false,
            is_code: false,
            highlights: find_highlights(&line, search_active, search_query),
        });
    }

    // Blank line after heading
    state.lines_cache.push(RenderedLine::empty(block_idx));
}

fn build_quote_lines(state: &mut AppState, text: &str, block_idx: usize, wrap_width: usize) {
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
    state.lines_cache.push(RenderedLine::empty(block_idx));
}

fn build_code_lines(state: &mut AppState, code: &str, block_idx: usize) {
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
    state.lines_cache.push(RenderedLine::empty(block_idx));
}

fn build_separator_lines(state: &mut AppState, block_idx: usize, wrap_width: usize) {
    state.lines_cache.push(RenderedLine {
        text: "─".repeat(wrap_width.min(40)),
        block_index: block_idx,
        is_heading: false,
        heading_level: 0,
        is_quote: false,
        is_code: false,
        highlights: Vec::new(),
    });
    state.lines_cache.push(RenderedLine::empty(block_idx));
}

fn build_list_lines(
    state: &mut AppState,
    items: &[String],
    ordered: bool,
    block_idx: usize,
    wrap_width: usize,
) {
    for (i, item) in items.iter().enumerate() {
        let prefix = if ordered {
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
    state.lines_cache.push(RenderedLine::empty(block_idx));
}

fn build_generic_lines(state: &mut AppState, text: &str, block_idx: usize, wrap_width: usize) {
    let wrapped = wrap(text, wrap_width);
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
    state.lines_cache.push(RenderedLine::empty(block_idx));
}

/// Find search highlights in text
fn find_highlights(text: &str, search_active: bool, search_query: &str) -> Vec<(usize, usize)> {
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
