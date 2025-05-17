use serde::Deserialize;

use crate::{block::BlockChildren, rich_text::RichTextVec};

use super::{Block, BlockContent, BlockMeta, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BulletedListItem {
    bulleted_list_item: BlockContent,
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
    #[serde(skip_serializing, default)]
    meta: BlockMeta,
}

impl BulletedListItem {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }

    pub(crate) fn with_meta(self, meta: BlockMeta) -> BulletedListItem {
        BulletedListItem {
            meta,
            children: self.children,
            bulleted_list_item: self.bulleted_list_item,
        }
    }
}

impl MarkdownBlock for BulletedListItem {
    fn to_markdown(&self) -> String {
        let inline = self.bulleted_list_item.rich_text.to_markdown();

        if self.children.is_empty() {
            format!("- {}", inline)
        } else {
            let children_markdown = self.children.to_markdown(self.meta.depth + 1);
            format!("- {}\n{}", inline, children_markdown)
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
            "../tests/block/bulleted_list_item_response.json"
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
              - this is bulleted list item
                - this is bulleted list item
                  - this is bulleted list item
                - this is bulleted list item
            "#},
        )
    }
}
