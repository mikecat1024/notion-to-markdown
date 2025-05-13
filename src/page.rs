use std::cell::RefCell;

use comrak::{
    nodes::{Ast, AstNode, NodeValue},
    Arena,
};
use serde::Deserialize;

use crate::block::Block;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Page {
    pub blocks: Vec<Block>,
}

impl Page {
    pub fn from_blocks(blocks: Vec<Block>) -> Page {
        return Page { blocks };
    }

    pub fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
        let document = arena.alloc(AstNode::new(RefCell::new(Ast::new(
            NodeValue::Document,
            Default::default(),
        ))));

        self.blocks
            .iter()
            .for_each(|block| document.append(block.to_ast(arena)));

        document
    }
}

#[cfg(test)]
mod test {

    use super::{Block, Page};
    use comrak::{format_commonmark, Arena, Options};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_page_with_bulleted_item() {
        let mut parent_item: Block =
            serde_json::from_str(include_str!("tests/block/bulleted_list_item_response.json"))
                .unwrap();
        let child_item1: Block =
            serde_json::from_str(include_str!("tests/block/bulleted_list_item_response.json"))
                .unwrap();
        let child_item2: Block =
            serde_json::from_str(include_str!("tests/block/bulleted_list_item_response.json"))
                .unwrap();

        parent_item.append(child_item1);
        parent_item.append(child_item2);

        let arena = Arena::new();

        let page = Page::from_blocks(vec![parent_item]);

        let ast = page.to_ast(&arena);

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
            - this is bulleted list item
              - this is second bulleted list item
              - this is second bulleted list item
            "#}
        )
    }

    #[rstest]
    fn test_page_with_numbered_item() {
        let mut numbered_parent_item: Block =
            serde_json::from_str(include_str!("tests/block/numbered_list_item_response.json"))
                .unwrap();
        let numbered_child_item1: Block =
            serde_json::from_str(include_str!("tests/block/numbered_list_item_response.json"))
                .unwrap();
        let numbered_child_item2: Block =
            serde_json::from_str(include_str!("tests/block/numbered_list_item_response.json"))
                .unwrap();

        numbered_parent_item.append(numbered_child_item1);
        numbered_parent_item.append(numbered_child_item2);

        let arena = Arena::new();

        let page = Page::from_blocks(vec![numbered_parent_item]);

        let ast = page.to_ast(&arena);

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
            1. this is numbered list item
               1. this is second numbered list item
               2. this is second numbered list item
            "#}
        )
    }

    #[rstest]
    fn test_page_to_markdown() {
        let headline1: Block =
            serde_json::from_str(include_str!("tests/block/headline1_response.json")).unwrap();
        let headline2: Block =
            serde_json::from_str(include_str!("tests/block/headline2_response.json")).unwrap();
        let headline3: Block =
            serde_json::from_str(include_str!("tests/block/headline3_response.json")).unwrap();
        let paragraph: Block =
            serde_json::from_str(include_str!("tests/block/paragraph_response.json")).unwrap();

        let arena = Arena::new();

        let page = Page::from_blocks(vec![headline1, headline2, headline3, paragraph]);

        let ast = page.to_ast(&arena);

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
            # this is headline1

            ## this is headline2

            ### this is headline3

            this is paragraph
            "#}
        )
    }
}
