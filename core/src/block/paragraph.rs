use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{BlockContent, MarkdownBlockWithoutChildren};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Paragraph {
    paragraph: BlockContent,
}

impl MarkdownBlockWithoutChildren for Paragraph {
    fn to_markdown(&self) -> String {
        self.paragraph.rich_text.to_markdown()
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
            serde_json::from_str(include_str!("../tests/block/paragraph_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                this is paragraph
            "#}
        )
    }
}
