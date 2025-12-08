//! Default implementations for configuration structures

use super::structs::*;
use super::{Keybindings, ThemeConfig};

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            tui: TuiConfig::default(),
            web: WebConfig::default(),
            library: LibraryConfig::default(),
            reader: ReaderConfig::default(),
            formats: FormatsConfig::default(),
            keybindings: Keybindings::default(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_interface: "tui".to_string(),
            data_dir: None,
            auto_save: true,
            auto_save_interval: 30,
            logging: true,
            log_level: "info".to_string(),
            language: "en".to_string(),
        }
    }
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            mouse_support: true,
            unicode: true,
            line_numbers: false,
            status_bar: true,
            status_bar_position: "bottom".to_string(),
            progress_bar: true,
            animations: true,
            animation_speed: "normal".to_string(),
            scrolloff: 5,
            smooth_scroll: true,
            dim_unfocused: true,
            show_sidebar: false,
            sidebar_width: 25,
            tab_size: 4,
            wrap_mode: "word".to_string(),
            max_width: 80,
            margin_left: 4,
            margin_right: 4,
        }
    }
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            bind: "127.0.0.1".to_string(),
            https: false,
            tls_cert: None,
            tls_key: None,
            auth_enabled: false,
            auth_method: "basic".to_string(),
            cors: true,
            cors_origins: vec!["*".to_string()],
            compression: true,
            static_dir: None,
            custom_css: None,
            custom_js: None,
            page_size: 1000,
            dark_mode: true,
            font_family: "Georgia, serif".to_string(),
            font_size: 18,
            line_height: 1.8,
            open_browser: true,
        }
    }
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            database_path: None,
            books_dir: None,
            watch_enabled: false,
            extract_covers: true,
            covers_dir: None,
            cover_max_width: 300,
            search_enabled: true,
            search_index_dir: None,
            auto_index: true,
            backup_enabled: true,
            backup_count: 5,
        }
    }
}

impl Default for ReaderConfig {
    fn default() -> Self {
        Self {
            remember_position: true,
            show_reading_time: true,
            words_per_minute: 250,
            dictionary_enabled: false,
            dictionary_source: "wiktionary".to_string(),
            tts_enabled: false,
            tts_voice: "default".to_string(),
            tts_speed: 1.0,
            highlight_search: true,
            search_case_sensitive: false,
            search_regex: false,
            justify: true,
            hyphenation: true,
            hyphenation_lang: "en-us".to_string(),
            prefer_interface: "tui".to_string(),
            layout_mode: "scroll".to_string(),
            pages_per_view: 1,
            page_animation: "slide".to_string(),
            show_page_numbers: true,
            auto_scroll_speed: 0,
            page_gap: 40,
        }
    }
}

impl Default for FormatsConfig {
    fn default() -> Self {
        Self {
            epub: EpubConfig::default(),
            pdf: PdfConfig::default(),
            markdown: MarkdownConfig::default(),
            txt: TxtConfig::default(),
        }
    }
}

impl Default for EpubConfig {
    fn default() -> Self {
        Self {
            show_images: true,
            image_mode: "inline".to_string(),
            parse_css: true,
            honor_font_size: false,
            inline_footnotes: true,
        }
    }
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            render_mode: "text".to_string(),
            dpi: 150,
            extract_text: true,
            ocr_enabled: false,
            ocr_lang: "eng".to_string(),
        }
    }
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            tables: true,
            task_lists: true,
            strikethrough: true,
            footnotes: true,
            smart_punctuation: true,
            syntax_highlighting: true,
            syntax_theme: "base16-ocean.dark".to_string(),
        }
    }
}

impl Default for TxtConfig {
    fn default() -> Self {
        Self {
            auto_encoding: true,
            default_encoding: "utf-8".to_string(),
            paragraph_mode: "blank_line".to_string(),
            normalize_line_endings: true,
        }
    }
}
