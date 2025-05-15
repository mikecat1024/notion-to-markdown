use comrak::{
    nodes::{AstNode, ListDelimType, ListType, NodeList, NodeValue},
    Arena,
};
use serde::Deserialize;

use crate::rich_text::RichText;

use super::{Block, BlockAstWithChildren};

#[derive(Deserialize, Clone, Debug)]
pub struct ToDo {
    to_do: ToDoContent,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ToDoContent {
    pub rich_text: Vec<RichText>,
    checked: bool,
}

impl BlockAstWithChildren for ToDo {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, children: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(
            arena,
            NodeValue::List(NodeList {
                list_type: ListType::Bullet,
                is_task_list: true,
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
            .to_do
            .rich_text
            .iter()
            .enumerate()
            .map(|(i, rich_text)| {
                if i == 0 {
                    let checked_x = if self.to_do.checked { "x" } else { " " };

                    match rich_text {
                        RichText::Text {
                            plain_text,
                            href,
                            annotations,
                        } => RichText::Text {
                            plain_text: format!("[{}] {}", checked_x, plain_text),
                            href: href.clone(),
                            annotations: annotations.clone(),
                        }
                        .to_ast(&arena),
                    }
                } else {
                    rich_text.to_ast(&arena)
                }
            })
            .flatten()
            .collect();

        text_asts.iter().for_each(|ast| paragraph.append(ast));

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
                    list_type: ListType::Bullet,
                    is_task_list: true,
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

    use comrak::{format_commonmark, Arena, Options};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::Block;

    #[test]
    fn test_to_markdown_when_unchecked() {
        let paragraph: Block =
            serde_json::from_str(include_str!("../tests/block/unchecked_to_do_response.json"))
                .unwrap();

        let arena = Arena::new();
        let ast = paragraph.to_ast(&arena);

        let mut options = Options::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;

        let mut output = vec![];
        format_commonmark(ast, &options, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            indoc! {r#"
                - [ ] this is to do item
            "#}
        )
    }

    #[test]
    fn test_to_markdown_when_checked() {
        let paragraph: Block =
            serde_json::from_str(include_str!("../tests/block/checked_to_do_response.json"))
                .unwrap();

        let arena = Arena::new();
        let ast = paragraph.to_ast(&arena);

        let mut options = Options::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;

        let mut output = vec![];
        format_commonmark(ast, &options, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            indoc! {r#"
                - [x] this is to do item
            "#}
        )
    }

    #[test]
    fn test_to_markdown_when_nest() {
        let mut item: Block =
            serde_json::from_str(include_str!("../tests/block/checked_to_do_response.json"))
                .unwrap();

        let child1: Block =
            serde_json::from_str(include_str!("../tests/block/unchecked_to_do_response.json"))
                .unwrap();

        let child2: Block =
            serde_json::from_str(include_str!("../tests/block/checked_to_do_response.json"))
                .unwrap();

        item.append(child1);
        item.append(child2);

        let arena = Arena::new();
        let ast = item.to_ast(&arena);

        let mut options = Options::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;

        let mut output = vec![];
        format_commonmark(ast, &options, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            indoc! {r#"
                - [x] this is to do item
                  - [ ] this is to do item
                  - [x] this is to do item
            "#}
        )
    }
}
