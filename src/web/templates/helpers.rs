//! Template helper functions

use crate::formats::{Chapter, ContentBlock};

/// Convert a chapter to HTML
pub fn chapter_to_html(chapter: &Chapter) -> String {
    let mut html = String::new();

    if let Some(title) = &chapter.title {
        html.push_str(&format!(
            "<h2 class=\"chapter-title\">{}</h2>\n",
            escape_html(title)
        ));
    }

    for block in &chapter.blocks {
        match block {
            ContentBlock::Paragraph { text, .. } => {
                html.push_str(&format!("<p>{}</p>\n", escape_html(text)));
            }
            ContentBlock::Heading { level, text } => {
                let tag_level = (*level + 1).min(6); // Offset by 1 since chapter title is h2
                html.push_str(&format!(
                    "<h{l}>{t}</h{l}>\n",
                    l = tag_level,
                    t = escape_html(text)
                ));
            }
            ContentBlock::Quote { text, attribution } => {
                html.push_str("<blockquote>\n");
                html.push_str(&format!("<p>{}</p>\n", escape_html(text)));
                if let Some(attr) = attribution {
                    html.push_str(&format!("<cite>— {}</cite>\n", escape_html(attr)));
                }
                html.push_str("</blockquote>\n");
            }
            ContentBlock::Code { language, code } => {
                let lang_class = language
                    .as_ref()
                    .map(|l| format!(" class=\"language-{}\"", l))
                    .unwrap_or_default();
                html.push_str(&format!(
                    "<pre><code{}>{}</code></pre>\n",
                    lang_class,
                    escape_html(code)
                ));
            }
            ContentBlock::List { ordered, items } => {
                let tag = if *ordered { "ol" } else { "ul" };
                html.push_str(&format!("<{}>\n", tag));
                for item in items {
                    html.push_str(&format!("<li>{}</li>\n", escape_html(item)));
                }
                html.push_str(&format!("</{}>\n", tag));
            }
            ContentBlock::Separator => {
                html.push_str("<hr>\n");
            }
            ContentBlock::Image {
                src, alt, caption, ..
            } => {
                html.push_str("<figure>\n");
                let alt_attr = alt
                    .as_ref()
                    .map(|a| format!(" alt=\"{}\"", escape_html(a)))
                    .unwrap_or_default();
                html.push_str(&format!("<img src=\"{}\"{}>\n", escape_html(src), alt_attr));
                if let Some(cap) = caption {
                    html.push_str(&format!("<figcaption>{}</figcaption>\n", escape_html(cap)));
                }
                html.push_str("</figure>\n");
            }
            ContentBlock::Table { headers, rows } => {
                html.push_str("<table>\n<thead>\n<tr>\n");
                for header in headers {
                    html.push_str(&format!("<th>{}</th>\n", escape_html(header)));
                }
                html.push_str("</tr>\n</thead>\n<tbody>\n");
                for row in rows {
                    html.push_str("<tr>\n");
                    for cell in row {
                        html.push_str(&format!("<td>{}</td>\n", escape_html(cell)));
                    }
                    html.push_str("</tr>\n");
                }
                html.push_str("</tbody>\n</table>\n");
            }
            ContentBlock::Footnote { id, content } => {
                html.push_str(&format!(
                    r#"<aside class="footnote" id="fn-{id}"><sup>{id}</sup> {content}</aside>"#,
                    id = escape_html(id),
                    content = escape_html(content),
                ));
            }
            _ => {}
        }
    }

    html
}

/// Escape HTML special characters
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Format word count for display
pub fn format_word_count(count: usize) -> String {
    if count >= 1000 {
        format!("{:.1}k", count as f64 / 1000.0)
    } else {
        count.to_string()
    }
}
