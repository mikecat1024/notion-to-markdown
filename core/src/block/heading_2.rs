use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{Block, BlockChildren, BlockContent, BlockMeta, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Heading2 {
    pub(crate) heading_2: BlockContent,
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
        let children = self.children.to_markdown(self.meta.depth + 1);

        if children.is_empty() {
            format!("## {}", self.heading_2.rich_text.to_markdown())
        } else {
            format!(
                "## {}\n{}",
                self.heading_2.rich_text.to_markdown(),
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
            serde_json::from_str(include_str!("../tests/block/headline2_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                ## this is headline2
            "#}
        )
    }

    #[test]
    fn test_to_markdown_with_children() {
        let mut item: Block =
            serde_json::from_str(include_str!("../tests/block/headline2_response.json")).unwrap();

        let child: Block =
            serde_json::from_str(include_str!("../tests/block/paragraph_response.json")).unwrap();

        item.append(child.clone());
        item.append(child);

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                ## this is headline2
                  this is paragraph
                  this is paragraph

            "#}
        )
    }
}
