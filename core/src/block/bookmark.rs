use serde::Deserialize;

use super::MarkdownBlockWithoutChildren;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Bookmark {
    bookmark: BookmarkContent,
}
#[derive(Deserialize, Clone, Debug)]

struct BookmarkContent {
    url: String,
}

impl MarkdownBlockWithoutChildren for Bookmark {
    fn to_markdown(&self) -> String {
        format!("[Bookmark: {}]({})", self.bookmark.url, self.bookmark.url)
    }
}

#[cfg(test)]
mod test {

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::Block;

    #[test]
    fn test_to_markdown() {
        let item: Block =
            serde_json::from_str(include_str!("../tests/block/bookmark_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                [Bookmark: https://example.com](https://example.com)
            "#}
        )
    }
}
