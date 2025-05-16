use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{Block, BlockChildren, BlockContent, BlockMeta, MarkdownBlockWithChildren};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NumberedListItem {
    numbered_list_item: BlockContent,
}

impl MarkdownBlockWithChildren for NumberedListItem {
    fn to_markdown(&self, children: &Vec<Block>, meta: &BlockMeta) -> String {
        let inline = self.numbered_list_item.rich_text.to_markdown();

        if children.is_empty() {
            format!("{}. {}", meta.order, inline)
        } else {
            let children_markdown = children.to_markdown(meta.depth + 1);
            format!("{}. {}\n{}", meta.order, inline, children_markdown)
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
