//! Settings page template

use super::base::base;
use crate::config::Config;

/// Generate the settings page
pub fn settings(config: &Config) -> String {
    let _dark_checked = if config.web.dark_mode { "checked" } else { "" };
    let justify_checked = if config.reader.justify { "checked" } else { "" };
    let hyphenate_checked = if config.reader.hyphenation {
        "checked"
    } else {
        ""
    };
    let show_progress_checked = if config.tui.progress_bar {
        "checked"
    } else {
        ""
    };

    let content = format!(
        r#"
        <header class="site-header">
            <h1>📖 Franko</h1>
            <nav>
                <a href="/">Home</a>
                <a href="/library">Library</a>
                <a href="/settings" class="active">Settings</a>
            </nav>
        </header>
        <main class="settings-page">
            <div class="settings-header">
                <h2>Settings</h2>
                <p class="settings-subtitle">Customize your reading experience</p>
            </div>

            <div class="settings-grid">
                {appearance_section}
                {typography_section}
                {reading_section}
                {keyboard_section}
                {library_section}
                {advanced_section}
            </div>

            <div class="settings-footer">
                <button class="btn primary" id="save-settings">💾 Save All Settings</button>
                <p class="settings-note">Settings are automatically saved to your browser. Click save to persist to server.</p>
            </div>
        </main>
    "#,
        appearance_section = appearance_section(config),
        typography_section = typography_section(config),
        reading_section =
            reading_section(justify_checked, hyphenate_checked, show_progress_checked),
        keyboard_section = keyboard_section(),
        library_section = library_section(config),
        advanced_section = advanced_section(config),
    );

    base("Settings", &content, config)
}

fn appearance_section(config: &Config) -> String {
    format!(
        r#"
        <!-- Appearance Section -->
        <section class="settings-card">
            <div class="settings-card-header">
                <span class="settings-icon">🎨</span>
                <h3>Appearance</h3>
            </div>
            <div class="settings-card-body">
                <div class="setting-item">
                    <div class="setting-info">
                        <label for="theme">Theme</label>
                        <p class="setting-description">Choose your preferred color scheme</p>
                    </div>
                    <div class="setting-control">
                        <div class="theme-switcher">
                            <button class="theme-btn {dark_active}" data-theme="dark" title="Dark">🌙</button>
                            <button class="theme-btn {light_active}" data-theme="light" title="Light">☀️</button>
                            <button class="theme-btn" data-theme="auto" title="Auto">🌓</button>
                        </div>
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="accent-color">Accent Color</label>
                        <p class="setting-description">Primary color for highlights and buttons</p>
                    </div>
                    <div class="setting-control">
                        <div class="color-picker">
                            <button class="color-btn active" data-color="indigo" style="--btn-color: #6366f1"></button>
                            <button class="color-btn" data-color="purple" style="--btn-color: #a855f7"></button>
                            <button class="color-btn" data-color="blue" style="--btn-color: #3b82f6"></button>
                            <button class="color-btn" data-color="green" style="--btn-color: #22c55e"></button>
                            <button class="color-btn" data-color="orange" style="--btn-color: #f97316"></button>
                            <button class="color-btn" data-color="pink" style="--btn-color: #ec4899"></button>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    "#,
        dark_active = if config.web.dark_mode { "active" } else { "" },
        light_active = if !config.web.dark_mode { "active" } else { "" },
    )
}

fn typography_section(config: &Config) -> String {
    format!(
        r#"
        <!-- Typography Section -->
        <section class="settings-card">
            <div class="settings-card-header">
                <span class="settings-icon">📝</span>
                <h3>Typography</h3>
            </div>
            <div class="settings-card-body">
                <div class="setting-item">
                    <div class="setting-info">
                        <label for="font-family">Font Family</label>
                        <p class="setting-description">Choose a comfortable reading font</p>
                    </div>
                    <div class="setting-control">
                        <select id="font-family" class="setting-select">
                            <option value="system" {font_system}>System Default</option>
                            <option value="serif" {font_serif}>Georgia (Serif)</option>
                            <option value="sans" {font_sans}>Inter (Sans-serif)</option>
                            <option value="mono" {font_mono}>JetBrains Mono</option>
                            <option value="literata" {font_literata}>Literata</option>
                            <option value="merriweather" {font_merriweather}>Merriweather</option>
                        </select>
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="font-size">Font Size</label>
                        <p class="setting-description">Base text size: <span id="font-size-value">{font_size}px</span></p>
                    </div>
                    <div class="setting-control">
                        <input type="range" id="font-size" class="setting-range"
                               min="12" max="32" value="{font_size}" step="1">
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="line-height">Line Height</label>
                        <p class="setting-description">Spacing between lines: <span id="line-height-value">{line_height}</span></p>
                    </div>
                    <div class="setting-control">
                        <input type="range" id="line-height" class="setting-range"
                               min="1.2" max="2.4" value="{line_height}" step="0.1">
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="text-width">Text Width</label>
                        <p class="setting-description">Maximum width of text column</p>
                    </div>
                    <div class="setting-control">
                        <select id="text-width" class="setting-select">
                            <option value="narrow">Narrow (600px)</option>
                            <option value="medium" selected>Medium (720px)</option>
                            <option value="wide">Wide (900px)</option>
                            <option value="full">Full Width</option>
                        </select>
                    </div>
                </div>
            </div>
        </section>
    "#,
        font_size = config.web.font_size,
        line_height = config.web.line_height,
        font_system = if config.web.font_family.contains("system") {
            "selected"
        } else {
            ""
        },
        font_serif = if config.web.font_family.contains("Georgia") {
            "selected"
        } else {
            ""
        },
        font_sans = if config.web.font_family.contains("Inter") {
            "selected"
        } else {
            ""
        },
        font_mono = if config.web.font_family.contains("JetBrains") {
            "selected"
        } else {
            ""
        },
        font_literata = if config.web.font_family.contains("Literata") {
            "selected"
        } else {
            ""
        },
        font_merriweather = if config.web.font_family.contains("Merriweather") {
            "selected"
        } else {
            ""
        },
    )
}

fn reading_section(
    justify_checked: &str,
    hyphenate_checked: &str,
    show_progress_checked: &str,
) -> String {
    format!(
        r#"
        <!-- Reading Section -->
        <section class="settings-card">
            <div class="settings-card-header">
                <span class="settings-icon">📖</span>
                <h3>Reading</h3>
            </div>
            <div class="settings-card-body">
                <div class="setting-item">
                    <div class="setting-info">
                        <label for="justify-text">Justify Text</label>
                        <p class="setting-description">Align text to both margins</p>
                    </div>
                    <div class="setting-control">
                        <label class="toggle">
                            <input type="checkbox" id="justify-text" {justify_checked}>
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="hyphenation">Hyphenation</label>
                        <p class="setting-description">Break long words at line ends</p>
                    </div>
                    <div class="setting-control">
                        <label class="toggle">
                            <input type="checkbox" id="hyphenation" {hyphenate_checked}>
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="show-progress">Show Progress Bar</label>
                        <p class="setting-description">Display reading progress at bottom</p>
                    </div>
                    <div class="setting-control">
                        <label class="toggle">
                            <input type="checkbox" id="show-progress" {show_progress_checked}>
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="scroll-behavior">Scroll Behavior</label>
                        <p class="setting-description">How pages transition while reading</p>
                    </div>
                    <div class="setting-control">
                        <select id="scroll-behavior" class="setting-select">
                            <option value="smooth">Smooth Scroll</option>
                            <option value="instant">Instant</option>
                            <option value="paginated">Paginated</option>
                        </select>
                    </div>
                </div>
            </div>
        </section>
    "#,
        justify_checked = justify_checked,
        hyphenate_checked = hyphenate_checked,
        show_progress_checked = show_progress_checked,
    )
}

fn keyboard_section() -> &'static str {
    r#"
    <!-- Keyboard Section -->
    <section class="settings-card">
        <div class="settings-card-header">
            <span class="settings-icon">⌨️</span>
            <h3>Keyboard Shortcuts</h3>
        </div>
        <div class="settings-card-body">
            <div class="shortcuts-list">
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>j</kbd> / <kbd>k</kbd></span>
                    <span class="shortcut-desc">Scroll down / up</span>
                </div>
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>h</kbd> / <kbd>l</kbd></span>
                    <span class="shortcut-desc">Previous / Next chapter</span>
                </div>
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>Space</kbd></span>
                    <span class="shortcut-desc">Page down</span>
                </div>
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>g</kbd> / <kbd>G</kbd></span>
                    <span class="shortcut-desc">Go to top / bottom</span>
                </div>
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>t</kbd></span>
                    <span class="shortcut-desc">Toggle table of contents</span>
                </div>
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>d</kbd></span>
                    <span class="shortcut-desc">Toggle dark mode</span>
                </div>
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>+</kbd> / <kbd>-</kbd></span>
                    <span class="shortcut-desc">Increase / Decrease font size</span>
                </div>
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>/</kbd></span>
                    <span class="shortcut-desc">Search</span>
                </div>
                <div class="shortcut-item">
                    <span class="shortcut-keys"><kbd>Esc</kbd></span>
                    <span class="shortcut-desc">Close sidebar / dialogs</span>
                </div>
            </div>

            <div class="setting-item" style="margin-top: 1.5rem;">
                <div class="setting-info">
                    <label for="keybinding-preset">Keybinding Preset</label>
                    <p class="setting-description">Choose your preferred key layout</p>
                </div>
                <div class="setting-control">
                    <select id="keybinding-preset" class="setting-select">
                        <option value="vim" selected>Vim-style</option>
                        <option value="emacs">Emacs-style</option>
                        <option value="reader">Reader-style</option>
                        <option value="custom">Custom</option>
                    </select>
                </div>
            </div>
        </div>
    </section>
    "#
}

fn library_section(config: &Config) -> String {
    format!(
        r#"
        <!-- Library Section -->
        <section class="settings-card">
            <div class="settings-card-header">
                <span class="settings-icon">📚</span>
                <h3>Library</h3>
            </div>
            <div class="settings-card-body">
                <div class="setting-item">
                    <div class="setting-info">
                        <label for="library-path">Library Path</label>
                        <p class="setting-description">Where your books are stored</p>
                    </div>
                    <div class="setting-control">
                        <input type="text" id="library-path" class="setting-input"
                               value="{library_path}" placeholder="~/Books">
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="auto-scan">Auto-scan Library</label>
                        <p class="setting-description">Automatically detect new books</p>
                    </div>
                    <div class="setting-control">
                        <label class="toggle">
                            <input type="checkbox" id="auto-scan" checked>
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="default-view">Default View</label>
                        <p class="setting-description">How to display your library</p>
                    </div>
                    <div class="setting-control">
                        <select id="default-view" class="setting-select">
                            <option value="grid">Grid</option>
                            <option value="list" selected>List</option>
                            <option value="compact">Compact</option>
                        </select>
                    </div>
                </div>
            </div>
        </section>
    "#,
        library_path = config
            .library
            .books_dir
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
    )
}

fn advanced_section(config: &Config) -> String {
    format!(
        r#"
        <!-- Advanced Section -->
        <section class="settings-card">
            <div class="settings-card-header">
                <span class="settings-icon">⚙️</span>
                <h3>Advanced</h3>
            </div>
            <div class="settings-card-body">
                <div class="setting-item">
                    <div class="setting-info">
                        <label for="web-port">Web Server Port</label>
                        <p class="setting-description">Port for the web interface</p>
                    </div>
                    <div class="setting-control">
                        <input type="number" id="web-port" class="setting-input"
                               value="{web_port}" min="1024" max="65535">
                    </div>
                </div>

                <div class="setting-item">
                    <div class="setting-info">
                        <label for="open-browser">Open Browser on Start</label>
                        <p class="setting-description">Automatically open in browser when server starts</p>
                    </div>
                    <div class="setting-control">
                        <label class="toggle">
                            <input type="checkbox" id="open-browser" checked>
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>

                <div class="setting-actions">
                    <button class="btn" id="export-config">📤 Export Config</button>
                    <button class="btn" id="import-config">📥 Import Config</button>
                    <button class="btn danger" id="reset-settings">🔄 Reset to Defaults</button>
                </div>
            </div>
        </section>
    "#,
        web_port = config.web.port,
    )
}
