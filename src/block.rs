use std::cell::RefCell;

use comrak::nodes::{Ast, AstNode, ListDelimType, ListType, NodeHeading, NodeList, NodeValue};
use comrak::Arena;
use serde;
use serde::Deserialize;

use crate::rich_text::RichText;

const UNSUPPORTED_NODE_TEXT: &str = "<!-- unsupported_block -->";

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Block {
    #[serde(flatten)]
    pub(crate) common: BlockCommon,
    #[serde(flatten)]
    pub(crate) variant: BlockVariant,
}

impl Block {
    fn child_to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, depth: usize) -> &'a AstNode<'a> {
        let node_value = self.variant.wrapper_node(depth);

        match &self.variant {
            BlockVariant::Paragraph {
                paragraph: block_content,
            }
            | BlockVariant::Heading1 {
                heading_1: block_content,
            }
            | BlockVariant::Heading2 {
                heading_2: block_content,
            }
            | BlockVariant::Heading3 {
                heading_3: block_content,
            } => {
                let wrapper = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    node_value,
                    Default::default(),
                ))));

                let rich_text_asts: Vec<&'a AstNode<'a>> = block_content
                    .rich_text
                    .iter()
                    .map(|rich_text| rich_text.to_ast(&arena))
                    .flatten()
                    .collect();

                rich_text_asts.iter().for_each(|ast| wrapper.append(ast));

                wrapper
            }
            BlockVariant::BulletedListItem { bulleted_list_item } => {
                let wrapper = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    node_value,
                    Default::default(),
                ))));

                let item_value = NodeValue::Item(NodeList {
                    list_type: ListType::Bullet,
                    is_task_list: false,
                    bullet_char: b'-',
                    tight: true,
                    delimiter: ListDelimType::default(),
                    marker_offset: depth,
                    padding: 2,
                    start: 1,
                });

                let item = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    item_value,
                    Default::default(),
                ))));

                let paragraph = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    NodeValue::Paragraph,
                    Default::default(),
                ))));

                let text_asts: Vec<&'a AstNode<'a>> = bulleted_list_item
                    .rich_text
                    .iter()
                    .map(|rich_text| rich_text.to_ast(&arena))
                    .flatten()
                    .collect();

                text_asts.iter().for_each(|ast| paragraph.append(ast));

                item.append(paragraph);

                let children_asts: Vec<&'a AstNode<'a>> = self
                    .common
                    .children
                    .iter()
                    .map(|child| child.child_to_ast(arena, depth + 1))
                    .collect();

                children_asts.iter().for_each(|ast| item.append(ast));

                wrapper.append(item);

                wrapper
            }
            _ => panic!("not implemented error"),
        }
    }

    pub(crate) fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
        let node_value = self.variant.wrapper_node(0);

        match &self.variant {
            BlockVariant::Paragraph {
                paragraph: block_content,
            }
            | BlockVariant::Heading1 {
                heading_1: block_content,
            }
            | BlockVariant::Heading2 {
                heading_2: block_content,
            }
            | BlockVariant::Heading3 {
                heading_3: block_content,
            } => {
                let wrapper = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    node_value,
                    Default::default(),
                ))));

                let rich_text_asts: Vec<&'a AstNode<'a>> = block_content
                    .rich_text
                    .iter()
                    .map(|rich_text| rich_text.to_ast(&arena))
                    .flatten()
                    .collect();

                rich_text_asts.iter().for_each(|ast| wrapper.append(ast));

                wrapper
            }
            BlockVariant::BulletedListItem { bulleted_list_item } => {
                let wrapper = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    node_value,
                    Default::default(),
                ))));

                let item_value = NodeValue::Item(NodeList {
                    list_type: ListType::Bullet,
                    is_task_list: false,
                    bullet_char: b'-',
                    tight: true,
                    delimiter: ListDelimType::default(),
                    marker_offset: 0,
                    padding: 2,
                    start: 1,
                });

                let item = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    item_value,
                    Default::default(),
                ))));

                let paragraph = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    NodeValue::Paragraph,
                    Default::default(),
                ))));

                let text_asts: Vec<&'a AstNode<'a>> = bulleted_list_item
                    .rich_text
                    .iter()
                    .map(|rich_text| rich_text.to_ast(&arena))
                    .flatten()
                    .collect();

                text_asts.iter().for_each(|ast| paragraph.append(ast));

                item.append(paragraph);

                let children_asts: Vec<&'a AstNode<'a>> = self
                    .common
                    .children
                    .iter()
                    .map(|child| child.child_to_ast(arena, 1))
                    .collect();

                children_asts.iter().for_each(|ast| item.append(ast));

                wrapper.append(item);

                wrapper
            }
            _ => panic!("not implemented error"),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BlockVariant {
    Paragraph {
        paragraph: BlockContent,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        heading_1: BlockContent,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        heading_2: BlockContent,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        heading_3: BlockContent,
    },
    BulletedListItem {
        bulleted_list_item: BlockContent,
    },
    #[serde(other)]
    Unsupported,
}

impl BlockVariant {
    fn wrapper_node(&self, depth: usize) -> NodeValue {
        match self {
            BlockVariant::Paragraph { .. } => NodeValue::Paragraph,
            BlockVariant::Heading1 { .. } => NodeValue::Heading(NodeHeading {
                level: 1,
                setext: true,
            }),
            BlockVariant::Heading2 { .. } => NodeValue::Heading(NodeHeading {
                level: 2,
                setext: true,
            }),
            BlockVariant::Heading3 { .. } => NodeValue::Heading(NodeHeading {
                level: 3,
                setext: true,
            }),
            BlockVariant::BulletedListItem { .. } => NodeValue::List(NodeList {
                list_type: ListType::Bullet,
                is_task_list: false,
                bullet_char: b'-',
                tight: true,
                delimiter: ListDelimType::default(),
                marker_offset: depth,
                padding: 2,
                start: 1,
            }),
            BlockVariant::Unsupported => NodeValue::Raw(UNSUPPORTED_NODE_TEXT.into()),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BlockContent {
    pub rich_text: Vec<RichText>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) struct BlockCommon {
    pub(crate) id: String,
    pub(crate) parent: Parent,
    #[serde(skip_serializing)]
    #[serde(default = "Vec::new")]
    pub(crate) children: Vec<Block>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum Parent {
    PageId,
    BlockId { block_id: String },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ParentType {
    PageId,
    BlockId,
}
