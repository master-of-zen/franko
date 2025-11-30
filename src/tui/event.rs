//! Event handling for TUI

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use std::time::Duration;

/// Poll for events with timeout
pub fn poll(timeout: Duration) -> std::io::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}

/// Input event abstraction
#[derive(Debug, Clone)]
pub enum InputEvent {
    Key(KeyInput),
    Mouse(MouseInput),
    Resize(u16, u16),
    Tick,
}

#[derive(Debug, Clone)]
pub struct KeyInput {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyInput {
    pub fn from_event(event: KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }

    pub fn is_ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }

    pub fn is_alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }

    pub fn is_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }

    pub fn char(&self) -> Option<char> {
        match self.code {
            KeyCode::Char(c) => Some(c),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MouseInput {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}

impl MouseInput {
    pub fn from_event(event: MouseEvent) -> Self {
        Self {
            kind: event.kind,
            column: event.column,
            row: event.row,
            modifiers: event.modifiers,
        }
    }
}

/// Convert crossterm event to our InputEvent
pub fn convert_event(event: Event) -> InputEvent {
    match event {
        Event::Key(key) => InputEvent::Key(KeyInput::from_event(key)),
        Event::Mouse(mouse) => InputEvent::Mouse(MouseInput::from_event(mouse)),
        Event::Resize(w, h) => InputEvent::Resize(w, h),
        _ => InputEvent::Tick,
    }
}
