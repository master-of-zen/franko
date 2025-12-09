//! Static file serving

use axum::{
    body::Body,
    extract::Path,
    http::{header, Response, StatusCode},
};

/// Serve embedded static files
pub async fn serve_static(Path(path): Path<String>) -> Response<Body> {
    let content_type = match path.rsplit('.').next() {
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("html") => "text/html",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    };

    let content = match path.as_str() {
        // Legacy monolithic files (kept for backward compatibility)
        "style.css" => Some(include_str!("../../assets/style.css")),
        "reader.js" => Some(include_str!("../../assets/reader.js")),

        // ========== CSS Modular Files ==========
        // Main entry point
        "css/main.css" => Some(include_str!("../../assets/css/main.css")),

        // Base
        "css/base/_index.css" => Some(include_str!("../../assets/css/base/_index.css")),
        "css/base/_variables.css" => Some(include_str!("../../assets/css/base/_variables.css")),
        "css/base/_reset.css" => Some(include_str!("../../assets/css/base/_reset.css")),

        // Components
        "css/components/_index.css" => Some(include_str!("../../assets/css/components/_index.css")),
        "css/components/_buttons.css" => {
            Some(include_str!("../../assets/css/components/_buttons.css"))
        }
        "css/components/_forms.css" => Some(include_str!("../../assets/css/components/_forms.css")),
        "css/components/_modal.css" => Some(include_str!("../../assets/css/components/_modal.css")),
        "css/components/_loading.css" => {
            Some(include_str!("../../assets/css/components/_loading.css"))
        }
        "css/components/_tooltips.css" => {
            Some(include_str!("../../assets/css/components/_tooltips.css"))
        }

        // Layout
        "css/layout/_index.css" => Some(include_str!("../../assets/css/layout/_index.css")),
        "css/layout/_header.css" => Some(include_str!("../../assets/css/layout/_header.css")),
        "css/layout/_responsive.css" => {
            Some(include_str!("../../assets/css/layout/_responsive.css"))
        }

        // Pages
        "css/pages/_index.css" => Some(include_str!("../../assets/css/pages/_index.css")),
        "css/pages/_home.css" => Some(include_str!("../../assets/css/pages/_home.css")),
        "css/pages/_library.css" => Some(include_str!("../../assets/css/pages/_library.css")),
        "css/pages/_reader.css" => Some(include_str!("../../assets/css/pages/_reader.css")),
        "css/pages/_reader-settings.css" => {
            Some(include_str!("../../assets/css/pages/_reader-settings.css"))
        }
        "css/pages/_settings.css" => Some(include_str!("../../assets/css/pages/_settings.css")),
        "css/pages/_book-info.css" => Some(include_str!("../../assets/css/pages/_book-info.css")),

        // Themes
        "css/themes/_index.css" => Some(include_str!("../../assets/css/themes/_index.css")),
        "css/themes/light.css" => Some(include_str!("../../assets/css/themes/light.css")),
        "css/themes/sepia.css" => Some(include_str!("../../assets/css/themes/sepia.css")),
        "css/themes/custom.css" => Some(include_str!("../../assets/css/themes/custom.css")),
        "css/themes/tokyo-night.css" => {
            Some(include_str!("../../assets/css/themes/tokyo-night.css"))
        }
        "css/themes/dracula.css" => Some(include_str!("../../assets/css/themes/dracula.css")),
        "css/themes/nord.css" => Some(include_str!("../../assets/css/themes/nord.css")),
        "css/themes/one-dark.css" => Some(include_str!("../../assets/css/themes/one-dark.css")),
        "css/themes/monokai.css" => Some(include_str!("../../assets/css/themes/monokai.css")),
        "css/themes/solarized-dark.css" => {
            Some(include_str!("../../assets/css/themes/solarized-dark.css"))
        }
        "css/themes/solarized-light.css" => {
            Some(include_str!("../../assets/css/themes/solarized-light.css"))
        }
        "css/themes/gruvbox-dark.css" => {
            Some(include_str!("../../assets/css/themes/gruvbox-dark.css"))
        }
        "css/themes/gruvbox-light.css" => {
            Some(include_str!("../../assets/css/themes/gruvbox-light.css"))
        }
        "css/themes/catppuccin-mocha.css" => {
            Some(include_str!("../../assets/css/themes/catppuccin-mocha.css"))
        }
        "css/themes/catppuccin-macchiato.css" => Some(include_str!(
            "../../assets/css/themes/catppuccin-macchiato.css"
        )),
        "css/themes/catppuccin-frappe.css" => Some(include_str!(
            "../../assets/css/themes/catppuccin-frappe.css"
        )),
        "css/themes/catppuccin-latte.css" => {
            Some(include_str!("../../assets/css/themes/catppuccin-latte.css"))
        }
        "css/themes/github-dark.css" => {
            Some(include_str!("../../assets/css/themes/github-dark.css"))
        }
        "css/themes/github-light.css" => {
            Some(include_str!("../../assets/css/themes/github-light.css"))
        }
        "css/themes/rose-pine.css" => Some(include_str!("../../assets/css/themes/rose-pine.css")),
        "css/themes/rose-pine-moon.css" => {
            Some(include_str!("../../assets/css/themes/rose-pine-moon.css"))
        }
        "css/themes/rose-pine-dawn.css" => {
            Some(include_str!("../../assets/css/themes/rose-pine-dawn.css"))
        }
        "css/themes/everforest-dark.css" => {
            Some(include_str!("../../assets/css/themes/everforest-dark.css"))
        }
        "css/themes/everforest-light.css" => {
            Some(include_str!("../../assets/css/themes/everforest-light.css"))
        }
        "css/themes/kanagawa.css" => Some(include_str!("../../assets/css/themes/kanagawa.css")),
        "css/themes/high-contrast.css" => {
            Some(include_str!("../../assets/css/themes/high-contrast.css"))
        }
        "css/themes/amoled.css" => Some(include_str!("../../assets/css/themes/amoled.css")),
        "css/themes/paper.css" => Some(include_str!("../../assets/css/themes/paper.css")),
        "css/themes/material-dark.css" => {
            Some(include_str!("../../assets/css/themes/material-dark.css"))
        }
        "css/themes/night-owl.css" => Some(include_str!("../../assets/css/themes/night-owl.css")),
        "css/themes/night-owl-light.css" => {
            Some(include_str!("../../assets/css/themes/night-owl-light.css"))
        }
        "css/themes/atom-one-dark.css" => {
            Some(include_str!("../../assets/css/themes/atom-one-dark.css"))
        }
        "css/themes/atom-one-light.css" => {
            Some(include_str!("../../assets/css/themes/atom-one-light.css"))
        }
        "css/themes/palenight.css" => Some(include_str!("../../assets/css/themes/palenight.css")),
        "css/themes/shades-of-purple.css" => {
            Some(include_str!("../../assets/css/themes/shades-of-purple.css"))
        }
        "css/themes/ayu-dark.css" => Some(include_str!("../../assets/css/themes/ayu-dark.css")),
        "css/themes/ayu-mirage.css" => Some(include_str!("../../assets/css/themes/ayu-mirage.css")),
        "css/themes/ayu-light.css" => Some(include_str!("../../assets/css/themes/ayu-light.css")),
        "css/themes/horizon.css" => Some(include_str!("../../assets/css/themes/horizon.css")),
        "css/themes/cobalt2.css" => Some(include_str!("../../assets/css/themes/cobalt2.css")),
        "css/themes/synthwave84.css" => {
            Some(include_str!("../../assets/css/themes/synthwave84.css"))
        }
        "css/themes/iceberg.css" => Some(include_str!("../../assets/css/themes/iceberg.css")),
        "css/themes/zenburn.css" => Some(include_str!("../../assets/css/themes/zenburn.css")),
        "css/themes/poimandres.css" => Some(include_str!("../../assets/css/themes/poimandres.css")),
        "css/themes/vesper.css" => Some(include_str!("../../assets/css/themes/vesper.css")),
        "css/themes/flexoki-dark.css" => {
            Some(include_str!("../../assets/css/themes/flexoki-dark.css"))
        }
        "css/themes/flexoki-light.css" => {
            Some(include_str!("../../assets/css/themes/flexoki-light.css"))
        }
        "css/themes/oxocarbon-dark.css" => {
            Some(include_str!("../../assets/css/themes/oxocarbon-dark.css"))
        }
        "css/themes/kindle.css" => Some(include_str!("../../assets/css/themes/kindle.css")),
        "css/themes/kobo.css" => Some(include_str!("../../assets/css/themes/kobo.css")),
        "css/themes/midnight-blue.css" => {
            Some(include_str!("../../assets/css/themes/midnight-blue.css"))
        }
        "css/themes/warm-night.css" => Some(include_str!("../../assets/css/themes/warm-night.css")),

        // ========== JS Modular Files ==========
        // Main entry point
        "js/main.js" => Some(include_str!("../../assets/js/main.js")),

        // Core modules
        "js/core/index.js" => Some(include_str!("../../assets/js/core/index.js")),
        "js/core/utils.js" => Some(include_str!("../../assets/js/core/utils.js")),
        "js/core/dom.js" => Some(include_str!("../../assets/js/core/dom.js")),
        "js/core/storage.js" => Some(include_str!("../../assets/js/core/storage.js")),
        "js/core/toast.js" => Some(include_str!("../../assets/js/core/toast.js")),

        // Feature modules
        "js/features/index.js" => Some(include_str!("../../assets/js/features/index.js")),
        "js/features/theme.js" => Some(include_str!("../../assets/js/features/theme.js")),
        "js/features/typography.js" => Some(include_str!("../../assets/js/features/typography.js")),
        "js/features/layout.js" => Some(include_str!("../../assets/js/features/layout.js")),
        "js/features/progress.js" => Some(include_str!("../../assets/js/features/progress.js")),
        "js/features/sidebar.js" => Some(include_str!("../../assets/js/features/sidebar.js")),
        "js/features/keyboard.js" => Some(include_str!("../../assets/js/features/keyboard.js")),
        "js/features/search.js" => Some(include_str!("../../assets/js/features/search.js")),
        "js/features/autoscroll.js" => Some(include_str!("../../assets/js/features/autoscroll.js")),
        "js/features/animations.js" => Some(include_str!("../../assets/js/features/animations.js")),
        "js/features/settings-panel.js" => {
            Some(include_str!("../../assets/js/features/settings-panel.js"))
        }
        "js/features/position.js" => Some(include_str!("../../assets/js/features/position.js")),

        _ => None,
    };

    match content {
        Some(data) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::from(data))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))
            .unwrap(),
    }
}
