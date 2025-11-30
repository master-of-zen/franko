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
        "style.css" => Some(include_str!("../../assets/style.css")),
        "reader.js" => Some(include_str!("../../assets/reader.js")),
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
