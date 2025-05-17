use super::{Block, BlockChildren, BlockContent, BlockMeta, MarkdownBlock};
use crate::rich_text::RichTextVec;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Heading3 {
    pub(crate) heading_3: BlockContent,
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
    #[serde(skip_serializing, default)]
    meta: BlockMeta,
}

impl Heading3 {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }

    pub(crate) fn with_meta(self, meta: BlockMeta) -> Heading3 {
        Heading3 {
            meta,
            children: self.children,
            heading_3: self.heading_3,
        }
    }
}

impl MarkdownBlock for Heading3 {
    fn to_markdown(&self) -> String {
        let children = self.children.to_markdown(self.meta.depth + 1);

        if children.is_empty() {
            format!("### {}", self.heading_3.rich_text.to_markdown())
        } else {
            format!(
                "### {}\n{}",
                self.heading_3.rich_text.to_markdown(),
                children
            )
        }
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

    #[test]
    fn test_to_markdown_with_children() {
        let mut item: Block =
            serde_json::from_str(include_str!("../tests/block/headline3_response.json")).unwrap();

        let child: Block =
            serde_json::from_str(include_str!("../tests/block/paragraph_response.json")).unwrap();

        item.append(child.clone());
        item.append(child);

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                ### this is headline3
                  this is paragraph
                  this is paragraph
            "#}
        )
    }
}
