//! TUI application runner

use super::event::{convert_event, poll, InputEvent};
use super::input::handle_input;
use super::render::render;
use super::state::AppState;
use crate::config::Config;
use crate::formats::Book;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use std::time::Duration;
use tracing::info;

/// Run the TUI application
pub fn run(book: Book, config: &Config) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    if config.tui.mouse_support {
        execute!(stdout, EnableMouseCapture)?;
    }

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create application state
    let mut state = AppState::new(book);

    // Apply config
    state.show_sidebar = config.tui.show_sidebar;
    state.show_status_bar = config.tui.status_bar;
    state.show_line_numbers = config.tui.line_numbers;
    state.theme = config.theme.active.clone();

    info!("Starting TUI reader");

    // Main loop
    let result = run_loop(&mut terminal, &mut state, config);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    state: &mut AppState,
    config: &Config,
) -> Result<()> {
    let tick_rate = Duration::from_millis(100);

    loop {
        // Render
        terminal.draw(|f| render(f, state, config))?;

        // Handle events
        if let Some(event) = poll(tick_rate)? {
            let input_event = convert_event(event);
            handle_input(state, input_event, config);
        } else {
            // Tick event for animations, timeouts, etc.
            handle_input(state, InputEvent::Tick, config);
        }

        // Check if we should quit
        if state.should_quit {
            break;
        }
    }

    Ok(())
}
