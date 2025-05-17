use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{Block, BlockChildren, BlockContent, BlockMeta, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Toggle {
    toggle: BlockContent,
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
    #[serde(skip_serializing, default)]
    meta: BlockMeta,
}

impl Toggle {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }

    pub(crate) fn with_meta(self, meta: BlockMeta) -> Toggle {
        Toggle {
            meta,
            toggle: self.toggle,
            children: self.children,
        }
    }
}

impl MarkdownBlock for Toggle {
    fn to_markdown(&self) -> String {
        format!(
            "{}\n{}",
            self.toggle.rich_text.to_markdown(),
            self.children.to_markdown(self.meta.depth + 1)
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
        let mut item: Block =
            serde_json::from_str(include_str!("../tests/block/toggle_response.json")).unwrap();

        let child1: Block =
            serde_json::from_str(include_str!("../tests/block/paragraph_response.json")).unwrap();
        let child2: Block =
            serde_json::from_str(include_str!("../tests/block/paragraph_response.json")).unwrap();

        item.append(child1);
        item.append(child2);

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                this is toggle
                  this is paragraph
                  this is paragraph
            "#}
        )
    }
}
