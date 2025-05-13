use comrak::{
    nodes::{AstNode, ListDelimType, ListType, NodeList, NodeValue},
    Arena,
};
use serde::Deserialize;

use super::{Block, BlockAst, BlockContent};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BulletedListItem {
    bulleted_list_item: BlockContent,
}

impl BlockAst for BulletedListItem {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, children: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(
            arena,
            NodeValue::List(NodeList {
                list_type: ListType::Bullet,
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

        let text_asts: Vec<&'a AstNode<'a>> = self
            .bulleted_list_item
            .rich_text
            .iter()
            .map(|rich_text| rich_text.to_ast(&arena))
            .flatten()
            .collect();

        text_asts.iter().for_each(|ast| paragraph.append(ast));

        item.append(paragraph);

        let mut has_list_item = false;

        let children_asts: Vec<&'a AstNode<'a>> = children
            .iter()
            .map(|child| match child {
                Block::BulletedListItem { .. } | Block::NumberedListItem { .. } => {
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
                    list_type: ListType::Bullet,
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
