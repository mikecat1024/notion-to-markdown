use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{Block, BlockChildren, BlockContent, BlockMeta, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NumberedListItem {
    numbered_list_item: BlockContent,
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
    #[serde(skip_serializing, default)]
    meta: BlockMeta,
}

impl NumberedListItem {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }

    pub(crate) fn with_meta(self, meta: BlockMeta) -> NumberedListItem {
        NumberedListItem {
            meta,
            children: self.children,
            numbered_list_item: self.numbered_list_item,
        }
    }
}

impl MarkdownBlock for NumberedListItem {
    fn to_markdown(&self) -> String {
        let inline = self.numbered_list_item.rich_text.to_markdown();

        if self.children.is_empty() {
            format!("{}. {}", self.meta.order, inline)
        } else {
            let children_markdown = self.children.to_markdown(self.meta.depth + 1);
            format!("{}. {}\n{}", self.meta.order, inline, children_markdown)
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
        let mut item: Block = serde_json::from_str(include_str!(
            "../tests/block/numbered_list_item_response.json"
        ))
        .unwrap();
        let mut item1 = item.clone();
        let item11 = item.clone();
        let item2 = item.clone();

        item1.append(item11);
        item.append(item1);
        item.append(item2);

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
              1. this is numbered list item
                1. this is numbered list item
                  1. this is numbered list item

                2. this is numbered list item

            "#}
        )
    }
}
