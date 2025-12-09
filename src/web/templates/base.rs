//! Base HTML template

use super::helpers::escape_html;
use crate::config::Config;

/// Generate the base HTML wrapper
pub fn base(title: &str, content: &str, config: &Config) -> String {
    let theme_class = if config.web.dark_mode {
        "dark"
    } else {
        "light"
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en" class="{theme_class}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Franko</title>
    <link rel="stylesheet" href="/static/css/main.css">
    <style>
        :root {{
            --font-family: {font_family};
            --font-size: {font_size}px;
            --line-height: {line_height};
        }}
    </style>
</head>
<body>
    <div id="app">
        {content}
    </div>
    <script type="module" src="/static/js/main.js"></script>
</body>
</html>"#,
        theme_class = theme_class,
        title = escape_html(title),
        content = content,
        font_family = config.web.font_family,
        font_size = config.web.font_size,
        line_height = config.web.line_height,
    )
}
