# 📚 Franko

**The Ultimate Book Reader for Power Users**

Franko is a powerful, highly configurable book reader with both TUI (Terminal User Interface) and Web interfaces. Built in Rust for blazing-fast performance, it's designed for readers who demand flexibility, customization, and vim-style efficiency.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-2021-orange)

## ✨ Features

### 📖 Multiple Reading Interfaces
- **TUI Mode**: Full-featured terminal interface with vim-like navigation
- **Web Mode**: Browser-based reading with modern responsive design
- Seamless switching between interfaces
- Synchronized reading progress across sessions

### 📄 Format Support
- **EPUB** - Full e-book support with metadata extraction
- **PDF** - Text extraction from PDF documents
- **Markdown** - Native markdown rendering with syntax highlighting
- **Plain Text** - TXT and HTML support
- Extensible format system for adding new formats

### ⚙️ Power User Configuration
- **TOML-based configuration** with sensible defaults
- **Keybinding presets**: Vim, Emacs, or fully custom
- **Customizable themes**: Tokyo Night, Solarized, Dracula, Nord, Gruvbox, and more
- **Feature flags**: Enable/disable features at compile time
- **Environment variable overrides** for quick tweaks
- **Per-book settings** for fine-grained control

### 🔍 Advanced Features
- **Full-text search** powered by Tantivy
- **Bookmarks & annotations** with color-coded highlights
- **Reading statistics** and progress tracking
- **Library management** with tagging and filtering
- **Session persistence** - continue where you left off
- **Export capabilities** - annotations, notes, progress data

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/master-of-zen/franko.git
cd franko

# Build with all features (default)
cargo build --release

# Build with specific features only
cargo build --release --no-default-features --features "tui epub markdown"

# Install to system
cargo install --path .
```

### Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `tui` | Terminal UI (ratatui/crossterm) | ✅ |
| `web` | Web interface (axum) | ✅ |
| `epub` | EPUB format support | ✅ |
| `pdf` | PDF format support | ✅ |
| `markdown` | Markdown format support | ✅ |
| `txt` | Plain text support | ✅ |
| `search` | Full-text search (tantivy) | ❌ |
| `syntax-highlighting` | Code syntax highlighting | ❌ |
| `image-support` | Inline image rendering | ❌ |

## 📘 Usage

### Quick Start

```bash
# Open a book in TUI mode (default)
franko read book.epub

# Open in web browser
franko read book.pdf --web

# Start the library web server
franko serve --port 8080

# Show book information
franko info book.epub
```

### Library Management

```bash
# Add a book to library
franko library add book.epub --tags "fiction,sci-fi"

# List all books
franko library list

# List with filters
franko library list --format epub --status reading

# Search library
franko library search "author:asimov"

# Import books from directory
franko library import ~/Books --recursive
```

### Bookmarks & Annotations

```bash
# List bookmarks for a book
franko library bookmark list book-id

# Add a bookmark
franko library bookmark add book-id 5:42 --name "Important quote"

# Export annotations
franko library annotation export book-id notes.md
```

### Configuration

```bash
# Show current configuration
franko config show

# Get a specific setting
franko config get reader.font_size

# Set a setting
franko config set reader.line_spacing 1.5

# Reset to defaults
franko config reset

# Open config in editor
franko config edit

# List available themes
franko config themes

# Initialize default config
franko init
```

## ⌨️ Keybindings

### TUI Mode (Vim Preset)

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `h` / `←` | Previous chapter |
| `l` / `→` | Next chapter |
| `gg` | Go to beginning |
| `G` | Go to end |
| `Ctrl+d` | Page down |
| `Ctrl+u` | Page up |
| `/` | Search |
| `n` | Next search result |
| `N` | Previous search result |
| `m` | Add bookmark |
| `'` | Go to bookmark |
| `t` | Toggle table of contents |
| `?` | Show help |
| `q` | Quit |
| `:` | Command mode |

### Web Mode

| Key | Action |
|-----|--------|
| `←` / `→` | Chapter navigation |
| `Space` | Toggle sidebar |
| `s` | Search |
| `b` | Bookmarks |
| `Esc` | Close dialogs |

## 🎨 Themes

### Built-in Themes

- **Tokyo Night** - Dark theme with vibrant accents
- **Solarized Dark/Light** - Classic color scheme
- **Dracula** - Popular dark theme
- **Nord** - Arctic, bluish color palette
- **Gruvbox Dark/Light** - Retro groove colors

### Custom Themes

Create your own themes in your config file:

```toml
[theme.custom.my_theme]
background = "#1a1b26"
foreground = "#a9b1d6"
accent = "#7aa2f7"
```

## 📁 Configuration

Configuration file location:
- Linux: `~/.config/franko/config.toml`
- macOS: `~/Library/Application Support/franko/config.toml`
- Windows: `%APPDATA%\franko\config.toml`

### Example Configuration

```toml
[reader]
font_size = 16
line_spacing = 1.4
text_width = 80
show_progress = true
auto_save_position = true
scroll_lines = 3

[library]
path = "~/.local/share/franko/library.db"
auto_add_opened = true
track_reading_time = true

[tui]
show_sidebar = false
show_status_bar = true
mouse_support = true
theme = "tokyo-night"

[web]
port = 8080
theme = "auto"
open_browser = true

[keybindings]
preset = "vim"  # or "emacs", "custom"

[keybindings.custom]
scroll_down = "j"
scroll_up = "k"
next_chapter = "l"
prev_chapter = "h"
quit = "q"
search = "/"
```

## 🔧 Environment Variables

Override configuration with environment variables:

```bash
export FRANKO_READER_FONT_SIZE=18
export FRANKO_TUI_THEME=dracula
export FRANKO_WEB_PORT=3000
export FRANKO_LIBRARY_PATH=/custom/path/library.db
```

## 📊 Library Database

Franko uses SQLite for storing your library data:
- Book metadata and file locations
- Reading progress per book
- Bookmarks with names and notes
- Annotations with highlights
- Reading statistics and history

## 🔍 Search

Full-text search is powered by Tantivy (requires `search` feature):

```bash
# Search within a book
franko search "quantum mechanics" book.epub

# Search library
franko library search "title:dune author:herbert"

# Regex search
franko search --regex "foo.*bar" book.txt
```

## 🛠️ Development

### Project Structure

```
franko/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # Command-line interface
│   ├── error.rs         # Error types
│   ├── reader.rs        # Reading session management
│   ├── config/          # Configuration system
│   ├── formats/         # Book format parsers
│   ├── library/         # Library management
│   ├── search/          # Full-text search
│   ├── tui/             # Terminal interface
│   └── web/             # Web interface
├── assets/              # Static web assets
├── tests/               # Integration tests
└── Cargo.toml
```

### Building for Development

```bash
# Debug build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- read book.epub

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🙏 Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [axum](https://github.com/tokio-rs/axum) - Web framework
- [tantivy](https://github.com/quickwit-oss/tantivy) - Search engine
- [epub](https://github.com/nicemicro/epub-rs) - EPUB parsing

---

**Happy Reading! 📖**