use std::cell::RefCell;

use bookmark::Bookmark;
use bulleted_list_item::BulletedListItem;
use callout::Callout;
use child_database::ChildDatabase;
use child_page::ChildPage;
use code::Code;
use comrak::nodes::{Ast, AstNode, NodeValue};
use comrak::Arena;
use divider::Divider;
use equation::Equation;
use file::File;
use heading_1::Heading1;
use heading_2::Heading2;
use heading_3::Heading3;
use image::Image;
use numbered_list_item::NumberedListItem;
use paragraph::Paragraph;
use pdf::Pdf;
use quote::Quote;
use serde;
use serde::Deserialize;
use table::{Table, TableRow};
use to_do::ToDo;
pub mod bookmark;
pub mod bulleted_list_item;
pub mod callout;
pub mod child_database;
pub mod child_page;
pub mod code;
pub mod divider;
pub mod equation;
pub mod file;
pub mod heading_1;
pub mod heading_2;
pub mod heading_3;
pub mod image;
pub mod numbered_list_item;
pub mod paragraph;
pub mod pdf;
pub mod quote;
pub mod table;
pub mod to_do;

use crate::rich_text::RichText;

const UNSUPPORTED_NODE_TEXT: &str = "<!-- unsupported block -->";
const UNEXPECTED_NODE_TEXT: &str = "<!-- unexpected block -->";

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
    Pdf {
        #[serde(flatten)]
        pdf: Pdf,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    Quote {
        #[serde(flatten)]
        quote: Quote,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    Code {
        #[serde(flatten)]
        code: Code,
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
    Image {
        #[serde(flatten)]
        image: Image,
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
    Divider {
        #[serde(flatten)]
        divider: Divider,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    File {
        #[serde(flatten)]
        file: File,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    ToDo {
        #[serde(flatten)]
        to_do: ToDo,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    Bookmark {
        #[serde(flatten)]
        bookmark: Bookmark,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    Callout {
        #[serde(flatten)]
        callout: Callout,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    ChildPage {
        #[serde(flatten)]
        child_page: ChildPage,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    Equation {
        #[serde(flatten)]
        equation: Equation,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    Table {
        #[serde(flatten)]
        table: Table,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    TableRow {
        #[serde(flatten)]
        table_row: TableRow,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    ChildDatabase {
        #[serde(flatten)]
        child_database: ChildDatabase,
        #[serde(skip_serializing)]
        #[serde(default = "Vec::new")]
        children: Vec<Block>,
    },
    Unsupported,
    #[serde(other)]
    Unexpected,
}

impl Block {
    pub(crate) fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
        match self {
            Block::Paragraph {
                paragraph,
                children,
            } => paragraph.to_ast(arena, children),
            Block::Quote { quote, children } => quote.to_ast(arena, children),
            Block::ChildPage {
                child_page,
                children,
            } => child_page.to_ast(arena, children),
            Block::Table { table, children } => table.to_ast(arena, children),
            Block::Equation { equation, children } => equation.to_ast(arena, children),
            Block::Pdf { pdf, children } => pdf.to_ast(arena, children),
            Block::Code { code, children } => code.to_ast(arena, children),
            Block::Bookmark { bookmark, children } => bookmark.to_ast(arena, children),
            Block::File { file, children } => file.to_ast(arena, children),
            Block::Callout { callout, children } => callout.to_ast(arena, children),
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
            Block::Image { image, children } => image.to_ast(arena, children),
            Block::ToDo { to_do, children } => to_do.to_ast(arena, children),
            Block::Divider { divider, children } => divider.to_ast(arena, children),
            Block::ChildDatabase {
                child_database,
                children,
            } => child_database.to_ast(arena, children),
            Block::Unsupported => arena.alloc(AstNode::new(RefCell::new(Ast::new(
                NodeValue::Raw(UNSUPPORTED_NODE_TEXT.into()),
                Default::default(),
            )))),
            Block::Unexpected | Block::TableRow { .. } => {
                arena.alloc(AstNode::new(RefCell::new(Ast::new(
                    NodeValue::Raw(UNEXPECTED_NODE_TEXT.into()),
                    Default::default(),
                ))))
            }
        }
    }

    pub(crate) fn append(&mut self, child: Block) {
        match self {
            Block::Paragraph { children, .. }
            | Block::Heading1 { children, .. }
            | Block::Heading2 { children, .. }
            | Block::Heading3 { children, .. }
            | Block::ChildPage { children, .. }
            | Block::Equation { children, .. }
            | Block::Callout { children, .. }
            | Block::Pdf { children, .. }
            | Block::Image { children, .. }
            | Block::File { children, .. }
            | Block::ChildDatabase { children, .. }
            | Block::Quote { children, .. }
            | Block::Divider { children, .. }
            | Block::BulletedListItem { children, .. }
            | Block::NumberedListItem { children, .. }
            | Block::Code { children, .. }
            | Block::ToDo { children, .. }
            | Block::Table { children, .. }
            | Block::TableRow { children, .. }
            | Block::Bookmark { children, .. } => children.push(child),
            Block::Unsupported | Block::Unexpected => {}
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
