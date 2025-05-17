use super::{BlockContent, MarkdownBlock};
use crate::rich_text::RichTextVec;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Heading3 {
    heading_3: BlockContent,
}

impl MarkdownBlock for Heading3 {
    fn to_markdown(&self) -> String {
        format!("### {}", self.heading_3.rich_text.to_markdown())
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
            serde_json::from_str(include_str!("../tests/block/headline3_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                ### this is headline3
            "#}
        )
    }
}
