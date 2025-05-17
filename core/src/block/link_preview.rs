use serde::Deserialize;

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LinkPreview {
    link_preview: LinkPreviewContent,
}
#[derive(Deserialize, Clone, Debug)]

struct LinkPreviewContent {
    url: String,
}

impl MarkdownBlock for LinkPreview {
    fn to_markdown(&self) -> String {
        format!(
            "[Preview: {}]({})",
            self.link_preview.url, self.link_preview.url
        )
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
            serde_json::from_str(include_str!("../tests/block/link_preview_response.json"))
                .unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                [Preview: https://example.com](https://example.com)
            "#}
        )
    }
}
