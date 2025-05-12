use std::cell::RefCell;

use comrak::{
    nodes::{Ast, AstNode, NodeValue},
    Arena,
};
use serde::Deserialize;

use crate::block::{Block, BlockCommon, Parent};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Page {
    blocks: Vec<Block>,
}

impl Page {
    fn struct_blocks(blocks: Vec<Block>) -> Vec<Block> {
        blocks
            .iter()
            .map(|parent| {
                let children: Vec<Block> = blocks
                    .iter()
                    .filter(|block| match &block.common.parent {
                        Parent::BlockId { block_id } => block_id == &parent.common.id,
                        _ => false,
                    })
                    .cloned()
                    .collect();

                let node = parent.clone();

                Block {
                    common: BlockCommon {
                        children,
                        ..node.common
                    },
                    variant: node.variant,
                }
            })
            .filter(|block| match &block.common.parent {
                Parent::BlockId { .. } => false,
                _ => true,
            })
            .collect()
    }

    pub fn from_blocks(blocks: Vec<Block>) -> Page {
        Page {
            blocks: Self::struct_blocks(blocks),
        }
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
    use rstest::rstest;

    #[rstest]
    fn test_page_to_ast() {
        let headline1: Block =
            serde_json::from_str(include_str!("tests/block/headline1_response.json")).unwrap();
        let headline2: Block =
            serde_json::from_str(include_str!("tests/block/headline2_response.json")).unwrap();
        let headline3: Block =
            serde_json::from_str(include_str!("tests/block/headline3_response.json")).unwrap();
        let paragraph: Block =
            serde_json::from_str(include_str!("tests/block/paragraph_response.json")).unwrap();
        let parent_item: Block =
            serde_json::from_str(include_str!("tests/block/bulleted_list_item_response.json"))
                .unwrap();
        let child_item: Block = serde_json::from_str(include_str!(
            "tests/block/bulleted_list_item_child_response.json"
        ))
        .unwrap();

        let arena = Arena::new();

        let page = Page::from_blocks(vec![
            headline1,
            headline2,
            headline3,
            paragraph,
            parent_item,
            child_item,
        ]);

        println!("{:#?}", page.blocks);
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

            - this is bulleted list item
              - this is second bulleted list item
            "#}
        )
    }
}
