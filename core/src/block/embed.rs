use serde::Deserialize;

use super::MarkdownBlockWithoutChildren;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Embed {
    embed: EmbedContent,
}
#[derive(Deserialize, Clone, Debug)]

struct EmbedContent {
    url: String,
}

impl MarkdownBlockWithoutChildren for Embed {
    fn to_markdown(&self) -> String {
        format!("[Embed: {}]({})", self.embed.url, self.embed.url)
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
            serde_json::from_str(include_str!("../tests/block/embed_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                [Embed: https://example.com](https://example.com)
            "#}
        )
    }
}
