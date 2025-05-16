use comrak::{
    nodes::{AstNode, ListDelimType, ListType, NodeList, NodeValue},
    Arena,
};
use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{Block, BlockAstWithChildren, BlockChildren, BlockContent, BlockMeta};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NumberedListItem {
    numbered_list_item: BlockContent,
}

impl NumberedListItem {
    pub(crate) fn to_markdown(&self, children: &Vec<Block>, meta: &BlockMeta) -> String {
        let inline = self.numbered_list_item.rich_text.to_markdown();

        if children.is_empty() {
            format!("{}. {}", meta.order, inline)
        } else {
            let children_markdown = children.to_markdown(meta.depth + 1);
            format!("{}. {}\n{}", meta.order, inline, children_markdown)
        }
    }
}

impl BlockAstWithChildren for NumberedListItem {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, children: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(
            arena,
            NodeValue::List(NodeList {
                list_type: ListType::Ordered,
                is_task_list: false,
                bullet_char: b'-',
                tight: true,
                delimiter: ListDelimType::default(),
                marker_offset: 0,
                padding: 4,
                start: 1,
            }),
        );

        let item_value = NodeValue::Item(NodeList {
            list_type: ListType::Ordered,
            is_task_list: false,
            bullet_char: b'-',
            tight: true,
            delimiter: ListDelimType::default(),
            marker_offset: 0,
            padding: 4,
            start: 1,
        });

        let item = Self::create_node(arena, item_value);

        let paragraph = Self::create_node(arena, NodeValue::Paragraph);

        self.numbered_list_item
            .rich_text
            .to_ast(arena)
            .iter()
            .for_each(|ast| paragraph.append(ast));

        item.append(paragraph);

        let mut has_list_item = false;

        let children_asts: Vec<&'a AstNode<'a>> = children
            .iter()
            .map(|child| match child {
                Block::BulletedListItem { .. }
                | Block::NumberedListItem { .. }
                | Block::ToDo { .. } => {
                    // This child AST should be wrapped with NodeValue::List
                    // So, AST have only one child, NodeValue::Item.
                    let ast = child.to_ast(arena);
                    let child = ast.first_child().unwrap();
                    has_list_item = true;
                    child
                }
                _ => child.to_ast(arena),
            })
            .collect();

        if has_list_item {
            let list = Self::create_node(
                arena,
                NodeValue::List(NodeList {
                    list_type: ListType::Ordered,
                    is_task_list: false,
                    bullet_char: b'-',
                    tight: true,
                    delimiter: ListDelimType::default(),
                    marker_offset: 0,
                    padding: 4,
                    start: 1,
                }),
            );
            children_asts.iter().for_each(|ast| list.append(ast));
            item.append(list);
        } else {
            children_asts.iter().for_each(|ast| item.append(ast));
        }

        wrapper.append(item);

        wrapper
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
