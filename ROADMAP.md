# Franko - Development Roadmap

## ✅ Completed Features

### Core Infrastructure

- [x] Project structure with Cargo workspace
- [x] Feature flags for optional components (tui, web, epub, pdf, markdown, txt, search, syntax-highlighting, image-support, cloud-sync)
- [x] Comprehensive configuration system (TOML-based with defaults)
- [x] Environment variable overrides
- [x] XDG directory support for data/config paths
- [x] Error handling with thiserror/anyhow
- [x] Logging with tracing

### CLI Interface

- [x] `franko read <file>` - Open book in TUI or web
- [x] `franko library add <path>` - Add book or folder to library
- [x] `franko library list` - List books with multiple output formats (table, json, csv, plain)
- [x] `franko library remove <id>` - Remove book from library
- [x] `franko library search <query>` - Search library
- [x] `franko library info <id>` - Show book details
- [x] `franko serve` - Start web server
- [x] `franko config show/get/set/reset/edit` - Configuration management
- [x] Recursive folder scanning for book imports

### Format Support

- [x] **EPUB**: Full parsing with metadata extraction, chapter navigation, HTML content extraction
- [x] **EPUB Cover Extraction**: Multiple fallback methods (get_cover API, resource pattern matching, first large image)
- [x] **PDF**: Improved text extraction with chapter detection, heading recognition, and artifact filtering
- [x] **Markdown**: Parsing with pulldown-cmark (tables, task lists, footnotes, syntax highlighting)
- [x] **Plain Text**: Basic support with encoding detection

### Library Management

- [x] SQLite database for book storage
- [x] Book metadata storage (title, author, publisher, language, etc.)
- [x] Reading progress tracking
- [x] Bookmarks system
- [x] Annotations with colors
- [x] Tags support
- [x] Search functionality
- [x] **Reading statistics** - Time tracking per book, library-wide statistics

### TUI Interface

- [x] Ratatui-based terminal UI
- [x] Vim-style keybindings (j/k, h/l, g/G, etc.)
- [x] Chapter navigation
- [x] Reading progress display
- [x] Sidebar with table of contents
- [x] Theme support (dark/light)
- [x] **In-book search** - Search within current book

### Web Interface

- [x] Axum-based web server
- [x] Modern card-based UI design
- [x] Dark/Light theme support
- [x] Responsive design
- [x] Homepage with recent books
- [x] Library page with search and sort
- [x] Book reader with chapter navigation
- [x] Book info page
- [x] **Settings page** with:
  - Theme selection (dark/light/auto)
  - Accent color picker (8 colors)
  - Font family selection (System, Serif, Sans, Mono, Literata, Merriweather)
  - Font size slider (12-32px)
  - Line height options
  - Text width options
  - Reading preferences (justify, hyphenation, progress bar)
  - Keyboard shortcuts display
  - Settings export/import
- [x] CSS variables for theming
- [x] Glass-morphism effects and animations
- [x] JavaScript settings persistence (localStorage)
- [x] Toast notifications
- [x] Query parameter chapter navigation (`?chapter=X`)

### API

- [x] REST API endpoints
- [x] `/api/books` - List books with filtering, sorting, pagination
- [x] `/api/books/:id` - Get book details
- [x] `/api/books/:id/content` - Get book content
- [x] `/api/books/:id/progress` - Get/set reading progress
- [x] `/api/books/:id/cover` - Get book cover image
- [x] `/api/books/:id/search` - Search within book
- [x] `/api/books/:id/reading-time` - Update reading time
- [x] `/api/books/:id/statistics` - Get book statistics
- [x] `/api/statistics` - Get library-wide statistics

---

## 🔄 In Progress / Known Issues

### Bugs to Fix

- [ ] Some PDFs fail to load (corrupt deflate streams, parse errors)
- [ ] Book titles with special characters display incorrectly (Unicode issues)
- [ ] `library list` crashes with "Broken pipe" when piped to other commands
- [ ] Some EPUB chapters show minimal content (cover pages, dedication pages)

### Parser Improvements Needed

- [ ] Better PDF text extraction (handle scanned PDFs, complex layouts)
- [ ] EPUB CSS parsing for better formatting
- [ ] EPUB image extraction and display
- [ ] Handle DRM-protected EPUBs gracefully (show error message)

---

## 📋 TODO - Next Steps

### High Priority

- [x] ~~**Fix PDF parsing reliability** - Handle more PDF variants~~ ✅ Improved with chapter detection, heading recognition, fallbacks
- [ ] **Improve EPUB content extraction** - Parse more HTML elements, handle CSS
- [x] ~~**Add book cover extraction** - Display covers in library and reader~~ ✅ Implemented for EPUB
- [ ] **Persist settings to config file** - Currently only localStorage, need server-side save
- [x] ~~**Add search within book** - Full-text search in current book~~ ✅ Implemented in TUI and Web API

### Medium Priority

- [x] ~~**Reading statistics** - Time spent reading, pages per day~~ ✅ Implemented with API endpoints
- [ ] **Collections/Shelves** - Organize books into collections
- [ ] **Import from Calibre** - Import existing Calibre libraries
- [ ] **OPDS catalog support** - Browse and download from OPDS feeds
- [ ] **Sync progress across devices** - Cloud sync implementation
- [ ] **Offline web app** - PWA with service worker
- [ ] **Keyboard shortcuts customization** - Edit keybindings in settings
- [ ] **Reading goals** - Set and track reading goals

### Low Priority / Future

- [ ] **FB2 format support** - Popular in Russian-speaking countries
- [ ] **MOBI/AZW format support** - Kindle formats
- [ ] **CBZ/CBR support** - Comic book archives
- [ ] **Text-to-speech** - Read books aloud
- [ ] **Dictionary integration** - Look up words while reading
- [ ] **Highlights export** - Export annotations to Markdown/JSON
- [ ] **Multiple themes** - Sepia, solarized, custom themes
- [ ] **Font loading** - Load custom fonts from web
- [ ] **Plugin system** - Extend functionality with plugins
- [ ] **Mobile app** - React Native or Flutter companion app
- [ ] **E-ink optimization** - Optimized UI for e-ink displays

### Performance

- [ ] **Lazy loading** - Load chapters on demand, not entire book
- [ ] **Caching** - Cache parsed books for faster loading
- [ ] **Pagination** - Handle large libraries efficiently
- [ ] **Background indexing** - Index books in background for search

### Documentation

- [ ] Complete README with screenshots
- [ ] Configuration documentation
- [ ] API documentation
- [ ] Contributing guide
- [ ] Release builds for Linux/macOS/Windows

---

## 📊 Statistics

- **Total Lines of Rust Code**: ~5,000+
- **Dependencies**: 30+
- **Supported Formats**: 4 (EPUB, PDF, Markdown, TXT)
- **Configuration Options**: 80+

---

## 🏗️ Architecture

```
franko/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI argument parsing
│   ├── error.rs          # Error types
│   ├── reader.rs         # Reading session management
│   ├── config/           # Configuration system
│   │   ├── mod.rs
│   │   ├── keybindings.rs
│   │   └── theme.rs
│   ├── formats/          # Book format parsers
│   │   ├── mod.rs
│   │   ├── book.rs       # Book data structures
│   │   ├── epub.rs
│   │   ├── pdf.rs
│   │   ├── markdown.rs
│   │   └── txt.rs
│   ├── library/          # Library management
│   │   ├── mod.rs
│   │   └── database.rs
│   ├── search/           # Full-text search
│   │   ├── mod.rs
│   │   ├── index.rs
│   │   └── query.rs
│   ├── tui/              # Terminal UI
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   ├── state.rs
│   │   ├── event.rs
│   │   ├── input.rs
│   │   ├── render.rs
│   │   └── components.rs
│   └── web/              # Web interface
│       ├── mod.rs
│       ├── api.rs
│       ├── handlers.rs
│       ├── templates.rs
│       └── static_files.rs
├── assets/
│   ├── style.css         # Web UI styles
│   └── reader.js         # Web UI JavaScript
├── Cargo.toml
├── README.md
├── LICENSE
├── config.example.toml
└── ROADMAP.md            # This file
```

---

## 🎯 Version Goals

### v0.1.0 (Current)

- Basic reading functionality
- Library management
- Web and TUI interfaces
- EPUB/PDF/Markdown/TXT support

### v0.2.0

- Improved format parsing
- Book covers
- Search within book
- Settings persistence

### v0.3.0

- Cloud sync
- Collections
- Reading statistics
- OPDS support

### v1.0.0

- Stable API
- Full documentation
- Cross-platform releases
- Plugin system
