pub use block::*;

mod block;
mod rich_text;

pub(crate) fn escape_page_title(title: &str) -> String {
    title
        .chars()
        .map(|c| {
            if c.is_whitespace() || c == '\u{200B}' {
                '_'
            } else {
                c
            }
        })
        .collect()
}
