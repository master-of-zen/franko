//! Theme configuration for Franko
//!
//! Supports both TUI and Web themes with customizable colors

use serde::{Deserialize, Serialize};

/// Built-in theme names
pub const BUILTIN_THEMES: &[&str] = &[
    "default",
    "dark",
    "light",
    "solarized-dark",
    "solarized-light",
    "gruvbox-dark",
    "gruvbox-light",
    "dracula",
    "nord",
    "monokai",
    "tokyo-night",
    "catppuccin-mocha",
    "catppuccin-latte",
    "sepia",
    "paper",
    "high-contrast",
];

/// Color representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    /// Named color (e.g., "red", "blue")
    Named(String),
    /// Hex color (e.g., "#ff0000")
    Hex(String),
    /// RGB color
    Rgb { r: u8, g: u8, b: u8 },
    /// RGBA color with alpha
    Rgba { r: u8, g: u8, b: u8, a: u8 },
}

impl Default for Color {
    fn default() -> Self {
        Color::Named("default".to_string())
    }
}

impl Color {
    pub fn hex(s: &str) -> Self {
        Color::Hex(s.to_string())
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb { r, g, b }
    }

    pub fn named(s: &str) -> Self {
        Color::Named(s.to_string())
    }

    /// Convert to CSS color string
    pub fn to_css(&self) -> String {
        match self {
            Color::Named(name) => name.clone(),
            Color::Hex(hex) => hex.clone(),
            Color::Rgb { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
            Color::Rgba { r, g, b, a } => format!("rgba({}, {}, {}, {})", r, g, b, *a as f32 / 255.0),
        }
    }

    /// Convert to ratatui color
    #[cfg(feature = "tui")]
    pub fn to_ratatui(&self) -> ratatui::style::Color {
        use ratatui::style::Color as RColor;

        match self {
            Color::Named(name) => match name.to_lowercase().as_str() {
                "black" => RColor::Black,
                "red" => RColor::Red,
                "green" => RColor::Green,
                "yellow" => RColor::Yellow,
                "blue" => RColor::Blue,
                "magenta" | "purple" => RColor::Magenta,
                "cyan" => RColor::Cyan,
                "white" => RColor::White,
                "gray" | "grey" => RColor::Gray,
                "darkgray" | "darkgrey" => RColor::DarkGray,
                "lightred" => RColor::LightRed,
                "lightgreen" => RColor::LightGreen,
                "lightyellow" => RColor::LightYellow,
                "lightblue" => RColor::LightBlue,
                "lightmagenta" => RColor::LightMagenta,
                "lightcyan" => RColor::LightCyan,
                _ => RColor::Reset,
            },
            Color::Hex(hex) => {
                let hex = hex.trim_start_matches('#');
                if let Ok(value) = u32::from_str_radix(hex, 16) {
                    let r = ((value >> 16) & 0xFF) as u8;
                    let g = ((value >> 8) & 0xFF) as u8;
                    let b = (value & 0xFF) as u8;
                    RColor::Rgb(r, g, b)
                } else {
                    RColor::Reset
                }
            }
            Color::Rgb { r, g, b } | Color::Rgba { r, g, b, .. } => RColor::Rgb(*r, *g, *b),
        }
    }
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    /// Active theme name
    pub active: String,

    /// UI colors
    pub ui: UiTheme,

    /// Content/text colors
    pub content: ContentTheme,

    /// Syntax highlighting colors
    pub syntax: SyntaxTheme,
}

/// UI element colors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiTheme {
    /// Background color
    pub background: Color,

    /// Foreground/text color
    pub foreground: Color,

    /// Primary accent color
    pub primary: Color,

    /// Secondary accent color
    pub secondary: Color,

    /// Success color
    pub success: Color,

    /// Warning color
    pub warning: Color,

    /// Error color
    pub error: Color,

    /// Border color
    pub border: Color,

    /// Selection background
    pub selection_bg: Color,

    /// Selection foreground
    pub selection_fg: Color,

    /// Status bar background
    pub statusbar_bg: Color,

    /// Status bar foreground
    pub statusbar_fg: Color,

    /// Sidebar background
    pub sidebar_bg: Color,

    /// Sidebar foreground
    pub sidebar_fg: Color,

    /// Scrollbar color
    pub scrollbar: Color,

    /// Scrollbar thumb color
    pub scrollbar_thumb: Color,

    /// Line numbers color
    pub line_numbers: Color,

    /// Current line background
    pub current_line: Color,

    /// Popup/modal background
    pub popup_bg: Color,

    /// Popup/modal border
    pub popup_border: Color,
}

/// Content/reading area colors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContentTheme {
    /// Main text color
    pub text: Color,

    /// Background color
    pub background: Color,

    /// Heading color
    pub heading: Color,

    /// Link color
    pub link: Color,

    /// Visited link color
    pub link_visited: Color,

    /// Emphasis/italic color
    pub emphasis: Color,

    /// Strong/bold color
    pub strong: Color,

    /// Quote/blockquote color
    pub quote: Color,

    /// Quote background
    pub quote_bg: Color,

    /// Quote border
    pub quote_border: Color,

    /// Code inline color
    pub code: Color,

    /// Code background
    pub code_bg: Color,

    /// Code block background
    pub code_block_bg: Color,

    /// Footnote color
    pub footnote: Color,

    /// Search highlight background
    pub search_highlight: Color,

    /// Bookmark indicator color
    pub bookmark: Color,

    /// Annotation highlight color
    pub annotation: Color,

    /// Progress indicator color
    pub progress: Color,
}

/// Syntax highlighting colors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SyntaxTheme {
    pub keyword: Color,
    pub string: Color,
    pub number: Color,
    pub comment: Color,
    pub function: Color,
    pub variable: Color,
    pub type_name: Color,
    pub constant: Color,
    pub operator: Color,
    pub punctuation: Color,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::dark()
    }
}

impl ThemeConfig {
    /// Dark theme (default)
    pub fn dark() -> Self {
        Self {
            active: "dark".to_string(),
            ui: UiTheme {
                background: Color::hex("#1a1b26"),
                foreground: Color::hex("#a9b1d6"),
                primary: Color::hex("#7aa2f7"),
                secondary: Color::hex("#bb9af7"),
                success: Color::hex("#9ece6a"),
                warning: Color::hex("#e0af68"),
                error: Color::hex("#f7768e"),
                border: Color::hex("#414868"),
                selection_bg: Color::hex("#33467c"),
                selection_fg: Color::hex("#c0caf5"),
                statusbar_bg: Color::hex("#1f2335"),
                statusbar_fg: Color::hex("#737aa2"),
                sidebar_bg: Color::hex("#1f2335"),
                sidebar_fg: Color::hex("#a9b1d6"),
                scrollbar: Color::hex("#292e42"),
                scrollbar_thumb: Color::hex("#565f89"),
                line_numbers: Color::hex("#3b4261"),
                current_line: Color::hex("#292e42"),
                popup_bg: Color::hex("#1f2335"),
                popup_border: Color::hex("#7aa2f7"),
            },
            content: ContentTheme {
                text: Color::hex("#c0caf5"),
                background: Color::hex("#1a1b26"),
                heading: Color::hex("#7dcfff"),
                link: Color::hex("#7aa2f7"),
                link_visited: Color::hex("#bb9af7"),
                emphasis: Color::hex("#bb9af7"),
                strong: Color::hex("#c0caf5"),
                quote: Color::hex("#9aa5ce"),
                quote_bg: Color::hex("#1f2335"),
                quote_border: Color::hex("#565f89"),
                code: Color::hex("#73daca"),
                code_bg: Color::hex("#292e42"),
                code_block_bg: Color::hex("#1f2335"),
                footnote: Color::hex("#737aa2"),
                search_highlight: Color::hex("#e0af68"),
                bookmark: Color::hex("#f7768e"),
                annotation: Color::hex("#9ece6a"),
                progress: Color::hex("#7aa2f7"),
            },
            syntax: SyntaxTheme::default(),
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            active: "light".to_string(),
            ui: UiTheme {
                background: Color::hex("#ffffff"),
                foreground: Color::hex("#1a1b26"),
                primary: Color::hex("#2e7de9"),
                secondary: Color::hex("#9854f1"),
                success: Color::hex("#587539"),
                warning: Color::hex("#8c6c3e"),
                error: Color::hex("#f52a65"),
                border: Color::hex("#dcdfe4"),
                selection_bg: Color::hex("#b6d6fd"),
                selection_fg: Color::hex("#1a1b26"),
                statusbar_bg: Color::hex("#e9e9ed"),
                statusbar_fg: Color::hex("#6172b0"),
                sidebar_bg: Color::hex("#e9e9ed"),
                sidebar_fg: Color::hex("#1a1b26"),
                scrollbar: Color::hex("#e9e9ed"),
                scrollbar_thumb: Color::hex("#9699a3"),
                line_numbers: Color::hex("#9699a3"),
                current_line: Color::hex("#e9e9ed"),
                popup_bg: Color::hex("#ffffff"),
                popup_border: Color::hex("#2e7de9"),
            },
            content: ContentTheme {
                text: Color::hex("#1a1b26"),
                background: Color::hex("#ffffff"),
                heading: Color::hex("#0f4b6e"),
                link: Color::hex("#2e7de9"),
                link_visited: Color::hex("#9854f1"),
                emphasis: Color::hex("#9854f1"),
                strong: Color::hex("#1a1b26"),
                quote: Color::hex("#6172b0"),
                quote_bg: Color::hex("#e9e9ed"),
                quote_border: Color::hex("#9699a3"),
                code: Color::hex("#587539"),
                code_bg: Color::hex("#e9e9ed"),
                code_block_bg: Color::hex("#f5f5f5"),
                footnote: Color::hex("#6172b0"),
                search_highlight: Color::hex("#ffe872"),
                bookmark: Color::hex("#f52a65"),
                annotation: Color::hex("#587539"),
                progress: Color::hex("#2e7de9"),
            },
            syntax: SyntaxTheme::light(),
        }
    }

    /// Sepia theme (paper-like, easy on eyes)
    pub fn sepia() -> Self {
        Self {
            active: "sepia".to_string(),
            ui: UiTheme {
                background: Color::hex("#f4ecd8"),
                foreground: Color::hex("#5b4636"),
                primary: Color::hex("#8b4513"),
                secondary: Color::hex("#a0522d"),
                success: Color::hex("#556b2f"),
                warning: Color::hex("#b8860b"),
                error: Color::hex("#a52a2a"),
                border: Color::hex("#d4c4a8"),
                selection_bg: Color::hex("#d4c4a8"),
                selection_fg: Color::hex("#5b4636"),
                statusbar_bg: Color::hex("#e8dccc"),
                statusbar_fg: Color::hex("#7a6550"),
                sidebar_bg: Color::hex("#e8dccc"),
                sidebar_fg: Color::hex("#5b4636"),
                scrollbar: Color::hex("#d4c4a8"),
                scrollbar_thumb: Color::hex("#b8a890"),
                line_numbers: Color::hex("#a89880"),
                current_line: Color::hex("#e8dccc"),
                popup_bg: Color::hex("#f4ecd8"),
                popup_border: Color::hex("#8b4513"),
            },
            content: ContentTheme {
                text: Color::hex("#5b4636"),
                background: Color::hex("#f4ecd8"),
                heading: Color::hex("#4a3728"),
                link: Color::hex("#8b4513"),
                link_visited: Color::hex("#a0522d"),
                emphasis: Color::hex("#654321"),
                strong: Color::hex("#3d2817"),
                quote: Color::hex("#7a6550"),
                quote_bg: Color::hex("#e8dccc"),
                quote_border: Color::hex("#b8a890"),
                code: Color::hex("#556b2f"),
                code_bg: Color::hex("#e8dccc"),
                code_block_bg: Color::hex("#e8dccc"),
                footnote: Color::hex("#7a6550"),
                search_highlight: Color::hex("#ffd700"),
                bookmark: Color::hex("#a52a2a"),
                annotation: Color::hex("#556b2f"),
                progress: Color::hex("#8b4513"),
            },
            syntax: SyntaxTheme::default(),
        }
    }
}

impl Default for UiTheme {
    fn default() -> Self {
        ThemeConfig::dark().ui
    }
}

impl Default for ContentTheme {
    fn default() -> Self {
        ThemeConfig::dark().content
    }
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        Self {
            keyword: Color::hex("#bb9af7"),
            string: Color::hex("#9ece6a"),
            number: Color::hex("#ff9e64"),
            comment: Color::hex("#565f89"),
            function: Color::hex("#7aa2f7"),
            variable: Color::hex("#c0caf5"),
            type_name: Color::hex("#2ac3de"),
            constant: Color::hex("#ff9e64"),
            operator: Color::hex("#89ddff"),
            punctuation: Color::hex("#9aa5ce"),
        }
    }
}

impl SyntaxTheme {
    pub fn light() -> Self {
        Self {
            keyword: Color::hex("#9854f1"),
            string: Color::hex("#587539"),
            number: Color::hex("#b15c00"),
            comment: Color::hex("#9699a3"),
            function: Color::hex("#2e7de9"),
            variable: Color::hex("#1a1b26"),
            type_name: Color::hex("#006c86"),
            constant: Color::hex("#b15c00"),
            operator: Color::hex("#006c86"),
            punctuation: Color::hex("#6172b0"),
        }
    }
}

/// Complete theme struct combining all theme data
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub config: ThemeConfig,
}

impl Theme {
    pub fn from_name(name: &str) -> Self {
        let config = match name {
            "light" => ThemeConfig::light(),
            "sepia" | "paper" => ThemeConfig::sepia(),
            _ => ThemeConfig::dark(),
        };

        Self {
            name: name.to_string(),
            config,
        }
    }
}
