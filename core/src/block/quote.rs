use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{BlockContent, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Quote {
    quote: BlockContent,
}

impl MarkdownBlock for Quote {
    fn to_markdown(&self) -> String {
        format!("> {}", self.quote.rich_text.to_markdown())
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
            serde_json::from_str(include_str!("../tests/block/quote_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                > this is quote
            "#}
        )
    }
}
