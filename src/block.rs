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
