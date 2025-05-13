use std::cell::RefCell;

use bulleted_list_item::BulletedListItem;
use comrak::nodes::{Ast, AstNode, NodeValue};
use comrak::Arena;
use heading1::Heading1;
use heading2::Heading2;
use heading3::Heading3;
use numbered_list_item::NumberedListItem;
use paragraph::Paragraph;
use serde;
use serde::Deserialize;
pub mod bulleted_list_item;
pub mod heading1;
pub mod heading2;
pub mod heading3;
pub mod numbered_list_item;
pub mod paragraph;

use crate::rich_text::RichText;

const UNSUPPORTED_NODE_TEXT: &str = "<!-- unsupported_block -->";

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]

pub enum Block {
    Paragraph {
        #[serde(flatten)]
        paragraph: Paragraph,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        #[serde(flatten)]
        heading_1: Heading1,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        #[serde(flatten)]
        heading_2: Heading2,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        #[serde(flatten)]
        heading_3: Heading3,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    BulletedListItem {
        #[serde(flatten)]
        bulleted_list_item: BulletedListItem,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    NumberedListItem {
        #[serde(flatten)]
        numbered_list_item: NumberedListItem,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    #[serde(other)]
    Unsupported,
}

impl Block {
    pub(crate) fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
        match self {
            Block::Paragraph {
                paragraph,
                children,
            } => paragraph.to_ast(arena, children),
            Block::Heading1 {
                heading_1,
                children,
            } => heading_1.to_ast(arena, children),
            Block::Heading2 {
                heading_2,
                children,
            } => heading_2.to_ast(arena, children),
            Block::Heading3 {
                heading_3,
                children,
            } => heading_3.to_ast(arena, children),
            Block::BulletedListItem {
                bulleted_list_item,
                children,
            } => bulleted_list_item.to_ast(arena, children),
            Block::NumberedListItem {
                numbered_list_item,
                children,
            } => numbered_list_item.to_ast(arena, children),
            Block::Unsupported => arena.alloc(AstNode::new(RefCell::new(Ast::new(
                NodeValue::Raw(UNSUPPORTED_NODE_TEXT.into()),
                Default::default(),
            )))),
        }
    }

    pub(crate) fn append(&mut self, child: Block) {
        match self {
            Block::Paragraph { children, .. }
            | Block::Heading1 { children, .. }
            | Block::Heading2 { children, .. }
            | Block::Heading3 { children, .. }
            | Block::BulletedListItem { children, .. }
            | Block::NumberedListItem { children, .. } => children.push(child),
            Block::Unsupported => {}
        }
    }
}

pub trait BlockAst {
    fn create_node<'a>(arena: &'a Arena<AstNode<'a>>, node_value: NodeValue) -> &'a AstNode<'a> {
        arena.alloc(AstNode::new(RefCell::new(Ast::new(
            node_value,
            Default::default(),
        ))))
    }

    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, children: &Vec<Block>) -> &'a AstNode<'a>;
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BlockContent {
    pub rich_text: Vec<RichText>,
}
