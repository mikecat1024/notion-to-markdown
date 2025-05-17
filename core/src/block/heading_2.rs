use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{Block, BlockContent, BlockMeta, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Heading2 {
    pub heading_2: BlockContent,
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
    #[serde(skip_serializing, default)]
    meta: BlockMeta,
}

impl Heading2 {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }

    pub(crate) fn with_meta(self, meta: BlockMeta) -> Heading2 {
        Heading2 {
            meta,
            children: self.children,
            heading_2: self.heading_2,
        }
    }
}

impl MarkdownBlock for Heading2 {
    fn to_markdown(&self) -> String {
        format!("## {}", self.heading_2.rich_text.to_markdown())
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
            serde_json::from_str(include_str!("../tests/block/headline2_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                ## this is headline2
            "#}
        )
    }
}
