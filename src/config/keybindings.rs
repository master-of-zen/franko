//! Keybinding configuration for Franko
//!
//! Supports Vim-style, Emacs-style, and custom keybindings

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Available keybinding presets
pub const PRESETS: &[&str] = &["vim", "emacs", "reader", "custom"];

/// Actions that can be bound to keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    // Navigation
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    PageUp,
    PageDown,
    HalfPageUp,
    HalfPageDown,
    GoToTop,
    GoToBottom,
    GoToLine,
    NextChapter,
    PrevChapter,
    NextParagraph,
    PrevParagraph,

    // Reading
    ToggleFullscreen,
    ToggleSidebar,
    ToggleStatusBar,
    ToggleLineNumbers,
    IncreaseFontSize,
    DecreaseFontSize,
    ResetFontSize,
    ToggleJustify,
    ToggleHyphenation,

    // Search
    Search,
    SearchNext,
    SearchPrev,
    ClearSearch,

    // Bookmarks
    AddBookmark,
    ListBookmarks,
    GotoBookmark,
    RemoveBookmark,

    // Annotations
    AddAnnotation,
    ListAnnotations,
    GotoAnnotation,
    RemoveAnnotation,

    // Dictionary/TTS
    LookupWord,
    ToggleTts,
    TtsPause,
    TtsStop,
    TtsSpeedUp,
    TtsSlowDown,

    // Library
    OpenLibrary,
    RecentBooks,
    BookInfo,

    // UI
    Help,
    Quit,
    ForceQuit,
    Escape,
    Confirm,
    Cancel,
    Refresh,
    ToggleTheme,
    CommandPalette,

    // Selection
    StartSelection,
    EndSelection,
    CopySelection,
    ClearSelection,
}

/// A key binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: String,
    pub modifiers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl KeyBinding {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            modifiers: vec![],
            mode: None,
        }
    }

    pub fn with_ctrl(mut self) -> Self {
        self.modifiers.push("ctrl".to_string());
        self
    }

    pub fn with_alt(mut self) -> Self {
        self.modifiers.push("alt".to_string());
        self
    }

    pub fn with_shift(mut self) -> Self {
        self.modifiers.push("shift".to_string());
        self
    }

    pub fn in_mode(mut self, mode: &str) -> Self {
        self.mode = Some(mode.to_string());
        self
    }
}

/// Complete keybindings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Keybindings {
    /// Active preset
    pub preset: String,

    /// Custom bindings (override preset)
    pub bindings: HashMap<Action, Vec<KeyBinding>>,

    /// Leader key for multi-key commands
    pub leader: String,

    /// Timeout for multi-key sequences (ms)
    pub timeout: u64,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self::vim_preset()
    }
}

impl Keybindings {
    /// Create Vim-style keybindings
    pub fn vim_preset() -> Self {
        let mut bindings = HashMap::new();

        // Navigation
        bindings.insert(
            Action::ScrollUp,
            vec![KeyBinding::new("k"), KeyBinding::new("Up")],
        );
        bindings.insert(
            Action::ScrollDown,
            vec![KeyBinding::new("j"), KeyBinding::new("Down")],
        );
        bindings.insert(
            Action::ScrollLeft,
            vec![KeyBinding::new("h"), KeyBinding::new("Left")],
        );
        bindings.insert(
            Action::ScrollRight,
            vec![KeyBinding::new("l"), KeyBinding::new("Right")],
        );
        bindings.insert(
            Action::PageUp,
            vec![KeyBinding::new("b").with_ctrl(), KeyBinding::new("PageUp")],
        );
        bindings.insert(
            Action::PageDown,
            vec![
                KeyBinding::new("f").with_ctrl(),
                KeyBinding::new("PageDown"),
                KeyBinding::new("Space"),
            ],
        );
        bindings.insert(Action::HalfPageUp, vec![KeyBinding::new("u").with_ctrl()]);
        bindings.insert(Action::HalfPageDown, vec![KeyBinding::new("d").with_ctrl()]);
        bindings.insert(
            Action::GoToTop,
            vec![KeyBinding::new("g"), KeyBinding::new("Home")],
        );
        bindings.insert(
            Action::GoToBottom,
            vec![KeyBinding::new("G"), KeyBinding::new("End")],
        );
        bindings.insert(Action::GoToLine, vec![KeyBinding::new(":")]);
        bindings.insert(
            Action::NextChapter,
            vec![KeyBinding::new("]"), KeyBinding::new("n")],
        );
        bindings.insert(
            Action::PrevChapter,
            vec![KeyBinding::new("["), KeyBinding::new("N")],
        );
        bindings.insert(Action::NextParagraph, vec![KeyBinding::new("}")]);
        bindings.insert(Action::PrevParagraph, vec![KeyBinding::new("{")]);

        // Search
        bindings.insert(Action::Search, vec![KeyBinding::new("/")]);
        bindings.insert(
            Action::SearchNext,
            vec![KeyBinding::new("n").in_mode("search")],
        );
        bindings.insert(
            Action::SearchPrev,
            vec![KeyBinding::new("N").in_mode("search")],
        );
        bindings.insert(Action::ClearSearch, vec![KeyBinding::new("Escape")]);

        // Bookmarks
        bindings.insert(Action::AddBookmark, vec![KeyBinding::new("m")]);
        bindings.insert(Action::ListBookmarks, vec![KeyBinding::new("'")]);
        bindings.insert(Action::GotoBookmark, vec![KeyBinding::new("`")]);

        // UI
        bindings.insert(Action::ToggleFullscreen, vec![KeyBinding::new("f")]);
        bindings.insert(Action::ToggleSidebar, vec![KeyBinding::new("s")]);
        bindings.insert(Action::ToggleStatusBar, vec![KeyBinding::new("S")]);
        bindings.insert(
            Action::IncreaseFontSize,
            vec![KeyBinding::new("="), KeyBinding::new("+")],
        );
        bindings.insert(Action::DecreaseFontSize, vec![KeyBinding::new("-")]);
        bindings.insert(Action::ResetFontSize, vec![KeyBinding::new("0")]);
        bindings.insert(
            Action::Help,
            vec![KeyBinding::new("?"), KeyBinding::new("F1")],
        );
        bindings.insert(Action::Quit, vec![KeyBinding::new("q")]);
        bindings.insert(
            Action::ForceQuit,
            vec![KeyBinding::new("Q"), KeyBinding::new("c").with_ctrl()],
        );
        bindings.insert(Action::Escape, vec![KeyBinding::new("Escape")]);
        bindings.insert(Action::Confirm, vec![KeyBinding::new("Enter")]);
        bindings.insert(
            Action::Refresh,
            vec![
                KeyBinding::new("r").with_ctrl(),
                KeyBinding::new("l").with_ctrl(),
            ],
        );
        bindings.insert(Action::ToggleTheme, vec![KeyBinding::new("t")]);
        bindings.insert(Action::CommandPalette, vec![KeyBinding::new(":")]);

        // Selection
        bindings.insert(Action::StartSelection, vec![KeyBinding::new("v")]);
        bindings.insert(Action::CopySelection, vec![KeyBinding::new("y")]);

        // Library
        bindings.insert(Action::OpenLibrary, vec![KeyBinding::new("L")]);
        bindings.insert(Action::BookInfo, vec![KeyBinding::new("i")]);

        Self {
            preset: "vim".to_string(),
            bindings,
            leader: " ".to_string(), // Space as leader
            timeout: 1000,
        }
    }

    /// Create Emacs-style keybindings
    pub fn emacs_preset() -> Self {
        let mut bindings = HashMap::new();

        // Navigation
        bindings.insert(
            Action::ScrollUp,
            vec![KeyBinding::new("p").with_ctrl(), KeyBinding::new("Up")],
        );
        bindings.insert(
            Action::ScrollDown,
            vec![KeyBinding::new("n").with_ctrl(), KeyBinding::new("Down")],
        );
        bindings.insert(
            Action::ScrollLeft,
            vec![KeyBinding::new("b").with_ctrl(), KeyBinding::new("Left")],
        );
        bindings.insert(
            Action::ScrollRight,
            vec![KeyBinding::new("f").with_ctrl(), KeyBinding::new("Right")],
        );
        bindings.insert(
            Action::PageUp,
            vec![KeyBinding::new("v").with_alt(), KeyBinding::new("PageUp")],
        );
        bindings.insert(
            Action::PageDown,
            vec![
                KeyBinding::new("v").with_ctrl(),
                KeyBinding::new("PageDown"),
            ],
        );
        bindings.insert(
            Action::GoToTop,
            vec![KeyBinding::new("<").with_alt(), KeyBinding::new("Home")],
        );
        bindings.insert(
            Action::GoToBottom,
            vec![KeyBinding::new(">").with_alt(), KeyBinding::new("End")],
        );
        bindings.insert(Action::GoToLine, vec![KeyBinding::new("g").with_alt()]);

        // Search
        bindings.insert(Action::Search, vec![KeyBinding::new("s").with_ctrl()]);
        bindings.insert(Action::SearchNext, vec![KeyBinding::new("s").with_ctrl()]);
        bindings.insert(Action::SearchPrev, vec![KeyBinding::new("r").with_ctrl()]);

        // UI
        bindings.insert(Action::Quit, vec![KeyBinding::new("q").with_ctrl()]);
        bindings.insert(Action::Help, vec![KeyBinding::new("h").with_ctrl()]);
        bindings.insert(
            Action::Escape,
            vec![KeyBinding::new("g").with_ctrl(), KeyBinding::new("Escape")],
        );

        // Selection
        bindings.insert(
            Action::StartSelection,
            vec![KeyBinding::new("Space").with_ctrl()],
        );
        bindings.insert(Action::CopySelection, vec![KeyBinding::new("w").with_alt()]);

        Self {
            preset: "emacs".to_string(),
            bindings,
            leader: "x".to_string(),
            timeout: 1000,
        }
    }

    /// Create a simple reader preset for casual users
    pub fn reader_preset() -> Self {
        let mut bindings = HashMap::new();

        // Simple navigation
        bindings.insert(Action::ScrollUp, vec![KeyBinding::new("Up")]);
        bindings.insert(Action::ScrollDown, vec![KeyBinding::new("Down")]);
        bindings.insert(
            Action::PageUp,
            vec![KeyBinding::new("PageUp"), KeyBinding::new("Left")],
        );
        bindings.insert(
            Action::PageDown,
            vec![
                KeyBinding::new("PageDown"),
                KeyBinding::new("Right"),
                KeyBinding::new("Space"),
            ],
        );
        bindings.insert(Action::GoToTop, vec![KeyBinding::new("Home")]);
        bindings.insert(Action::GoToBottom, vec![KeyBinding::new("End")]);
        bindings.insert(Action::NextChapter, vec![KeyBinding::new("n")]);
        bindings.insert(Action::PrevChapter, vec![KeyBinding::new("p")]);

        // Search
        bindings.insert(Action::Search, vec![KeyBinding::new("f").with_ctrl()]);

        // Bookmarks
        bindings.insert(Action::AddBookmark, vec![KeyBinding::new("b").with_ctrl()]);
        bindings.insert(
            Action::ListBookmarks,
            vec![KeyBinding::new("B").with_ctrl()],
        );

        // UI
        bindings.insert(Action::ToggleFullscreen, vec![KeyBinding::new("F11")]);
        bindings.insert(
            Action::IncreaseFontSize,
            vec![
                KeyBinding::new("+").with_ctrl(),
                KeyBinding::new("=").with_ctrl(),
            ],
        );
        bindings.insert(
            Action::DecreaseFontSize,
            vec![KeyBinding::new("-").with_ctrl()],
        );
        bindings.insert(Action::Help, vec![KeyBinding::new("F1")]);
        bindings.insert(
            Action::Quit,
            vec![KeyBinding::new("q"), KeyBinding::new("Escape")],
        );

        Self {
            preset: "reader".to_string(),
            bindings,
            leader: String::new(),
            timeout: 500,
        }
    }

    /// Get the binding for an action
    pub fn get(&self, action: Action) -> Option<&Vec<KeyBinding>> {
        self.bindings.get(&action)
    }

    /// Add or override a binding
    pub fn bind(&mut self, action: Action, bindings: Vec<KeyBinding>) {
        self.bindings.insert(action, bindings);
    }

    /// Remove a binding
    pub fn unbind(&mut self, action: Action) {
        self.bindings.remove(&action);
    }
}
