use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{Block, BlockContent, BlockMeta, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Heading1 {
    pub heading_1: BlockContent,
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
    #[serde(skip_serializing, default)]
    meta: BlockMeta,
}

impl Heading1 {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }

    pub(crate) fn with_meta(self, meta: BlockMeta) -> Heading1 {
        Heading1 {
            meta,
            children: self.children,
            heading_1: self.heading_1,
        }
    }
}

impl MarkdownBlock for Heading1 {
    fn to_markdown(&self) -> String {
        format!("# {}", self.heading_1.rich_text.to_markdown())
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
            serde_json::from_str(include_str!("../tests/block/headline1_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                # this is headline1
            "#}
        )
    }
}
