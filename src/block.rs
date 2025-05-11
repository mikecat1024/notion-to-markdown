use std::cell::RefCell;

use comrak::nodes::{Ast, AstNode, NodeValue};
use comrak::Arena;
use serde;
use serde::Deserialize;

use crate::rich_text::RichText;

const UNSUPPORTED_NODE_TEXT: &str = "<!-- unsupported_block -->";

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Block {
    Paragraph {
        paragraph: Paragraph,
    },
    #[serde(other)]
    Unsupported,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct Paragraph {
    pub rich_text: Vec<RichText>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub children: Option<Vec<Block>>,
}

impl Block {
    pub fn to_ast<'a>(&'a self, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
        match self {
            Block::Paragraph { paragraph } => {
                let paragraph_node = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    NodeValue::Paragraph,
                    Default::default(),
                ))));

                for rich_text in &paragraph.rich_text {
                    println!("rich_text: {:#?}", rich_text);
                    let children = rich_text.to_ast(arena);
                    children.iter().for_each(|node| paragraph_node.append(node));
                }

                paragraph_node
            }
            Block::Unsupported => arena.alloc(AstNode::new(RefCell::new(Ast::new(
                NodeValue::Raw(UNSUPPORTED_NODE_TEXT.into()),
                Default::default(),
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, fs, io::BufReader};

    use comrak::{
        arena_tree::Node,
        format_commonmark,
        nodes::{Ast, NodeValue},
        parse_document, Arena,
    };
    use rstest::rstest;

    use crate::{test_utils::ast_eq, utils::gfm_options};

    use super::Block;

    #[rstest]
    #[case::paragraph("src/tests/block/paragraph_response.json", "this is paragraph")]
    fn test_block_to_ast(#[case] path: String, #[case] expected: String) {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let block: Block = serde_json::from_reader(reader).unwrap();

        let arena = Arena::new();
        let expected_document = parse_document(&arena, &expected, &gfm_options());
        let notion_paragraph = block.to_ast(&arena);

        let notion_document = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::Document,
            Default::default(),
        ))));

        notion_document.append(&notion_paragraph);

        assert!(
            ast_eq(notion_document, expected_document),
            "Expected {:#?}, but found {:#?}",
            expected_document,
            notion_document
        );
    }

    #[rstest]
    #[case::paragraph("src/tests/block/paragraph_response.json", "this is paragraph")]
    fn test_block_to_markdown(#[case] path: String, #[case] expected_markdown: String) {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let block: Block = serde_json::from_reader(reader).unwrap();

        let arena = Arena::new();
        let notion_paragraph = block.to_ast(&arena);

        let notion_document = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::Document,
            Default::default(),
        ))));
        notion_document.append(&notion_paragraph);

        let mut notion_markdown = vec![];
        let _ = format_commonmark(&notion_document, &gfm_options(), &mut notion_markdown);

        assert_eq!(
            {
                if expected_markdown.is_empty() {
                    "".into()
                } else {
                    expected_markdown + "\n"
                }
            },
            String::from_utf8(notion_markdown).unwrap(),
        );
    }
}
