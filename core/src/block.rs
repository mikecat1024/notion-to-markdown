use core::panic;

use bookmark::Bookmark;
use bulleted_list_item::BulletedListItem;
use callout::Callout;
use child_database::ChildDatabase;
use child_page::ChildPage;
use code::Code;
use divider::Divider;
use embed::Embed;
use equation::Equation;
use file::File;
use heading_1::Heading1;
use heading_2::Heading2;
use heading_3::Heading3;
use image::Image;
use link_preview::LinkPreview;
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
pub mod embed;
pub mod equation;
pub mod file;
pub mod heading_1;
pub mod heading_2;
pub mod heading_3;
pub mod image;
pub mod link_preview;
// pub mod link_to_page;
pub mod numbered_list_item;
pub mod paragraph;
pub mod pdf;
pub mod quote;
pub mod table;
pub mod to_do;

use crate::rich_text::RichText;

const UNSUPPORTED_NODE_TEXT: &str = "<!-- unsupported block -->";
const UNEXPECTED_NODE_TEXT: &str = "<!-- unexpected block -->";
const INDENT: &str = "  ";

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]

pub enum Block {
    NumberedListItem {
        #[serde(flatten)]
        numbered_list_item: NumberedListItem,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
        #[serde(skip_serializing, default)]
        meta: BlockMeta,
    },
    BulletedListItem {
        #[serde(flatten)]
        bulleted_list_item: BulletedListItem,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
        #[serde(skip_serializing, default)]
        meta: BlockMeta,
    },
    Paragraph {
        #[serde(flatten)]
        paragraph: Paragraph,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
    },
    Pdf {
        #[serde(flatten)]
        pdf: Pdf,
    },
    Quote {
        #[serde(flatten)]
        quote: Quote,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
    },
    Code {
        #[serde(flatten)]
        code: Code,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        #[serde(flatten)]
        heading_1: Heading1,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        #[serde(flatten)]
        heading_2: Heading2,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        #[serde(flatten)]
        heading_3: Heading3,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
    },

    Image {
        #[serde(flatten)]
        image: Image,
    },

    Divider {
        #[serde(flatten)]
        divider: Divider,
    },
    File {
        #[serde(flatten)]
        file: File,
    },
    ToDo {
        #[serde(flatten)]
        to_do: ToDo,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
        #[serde(skip_serializing, default)]
        meta: BlockMeta,
    },
    Bookmark {
        #[serde(flatten)]
        bookmark: Bookmark,
    },
    Callout {
        #[serde(flatten)]
        callout: Callout,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
    },
    ChildPage {
        #[serde(flatten)]
        child_page: ChildPage,
    },
    Equation {
        #[serde(flatten)]
        equation: Equation,
    },
    Table {
        #[serde(flatten)]
        table: Table,
        #[serde(skip_serializing, default)]
        children: Vec<Block>,
        #[serde(skip_serializing, default)]
        meta: BlockMeta,
    },
    TableRow {
        #[serde(flatten)]
        table_row: TableRow,
    },
    Embed {
        #[serde(flatten)]
        embed: Embed,
    },
    LinkPreview {
        #[serde(flatten)]
        link_preview: LinkPreview,
    },
    // LinkToPage {
    //     #[serde(flatten)]
    //     link_to_page: LinkToPage,
    // },
    ChildDatabase {
        #[serde(flatten)]
        child_database: ChildDatabase,
    },
    Unsupported,
    #[serde(other)]
    Unexpected,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
pub struct BlockMeta {
    order: usize,
    depth: usize,
}

impl Default for BlockMeta {
    fn default() -> BlockMeta {
        BlockMeta { order: 1, depth: 0 }
    }
}

impl Block {
    fn with_meta(self, meta: BlockMeta) -> Block {
        match self {
            Block::NumberedListItem {
                numbered_list_item,
                children,
                ..
            } => Block::NumberedListItem {
                numbered_list_item,
                children,
                meta: meta,
            },
            Block::BulletedListItem {
                bulleted_list_item,
                children,
                ..
            } => Block::BulletedListItem {
                bulleted_list_item,
                children,
                meta: meta,
            },
            Block::ToDo {
                to_do,
                children,
                meta,
            } => Block::ToDo {
                to_do,
                children,
                meta,
            },
            Block::Paragraph {
                paragraph,
                children,
            } => Block::Paragraph {
                paragraph,
                children,
            },
            _ => todo!("not implemented"),
        }
    }

    pub fn append(&mut self, child: Block) {
        match self {
            Block::Paragraph { children, .. }
            | Block::Heading1 { children, .. }
            | Block::Heading2 { children, .. }
            | Block::Heading3 { children, .. }
            | Block::Callout { children, .. }
            | Block::Quote { children, .. }
            | Block::BulletedListItem { children, .. }
            | Block::NumberedListItem { children, .. }
            | Block::Table { children, .. }
            | Block::ToDo { children, .. } => children.push(child),
            _ => {}
        }
    }

    pub fn to_markdown(&self) -> String {
        match &self {
            Block::NumberedListItem {
                numbered_list_item,
                children,
                meta,
            } => numbered_list_item.to_markdown(children, meta),
            Block::BulletedListItem {
                bulleted_list_item,
                children,
                meta,
            } => bulleted_list_item.to_markdown(children, meta),
            Block::ToDo {
                to_do,
                children,
                meta,
            } => to_do.to_markdown(children, meta),
            Block::Table {
                table,
                children,
                meta,
            } => table.to_markdown(children, meta),
            Block::Paragraph { paragraph, .. } => paragraph.to_markdown(),
            Block::Pdf { pdf, .. } => pdf.to_markdown(),
            Block::Quote { quote, .. } => quote.to_markdown(),
            Block::Code { code } => code.to_markdown(),
            Block::Heading1 { heading_1, .. } => heading_1.to_markdown(),
            Block::Heading2 { heading_2, .. } => heading_2.to_markdown(),
            Block::Heading3 { heading_3, .. } => heading_3.to_markdown(),
            Block::Image { image } => image.to_markdown(),
            Block::Divider { divider } => divider.to_markdown(),
            Block::File { file } => file.to_markdown(),
            Block::Bookmark { bookmark } => bookmark.to_markdown(),
            Block::Equation { equation } => equation.to_markdown(),
            Block::Callout { callout, .. } => callout.to_markdown(),
            Block::ChildPage { child_page, .. } => child_page.to_markdown(),
            Block::Embed { embed } => embed.to_markdown(),
            Block::LinkPreview { link_preview } => link_preview.to_markdown(),
            Block::ChildDatabase { child_database } => child_database.to_markdown(),
            Block::Unsupported => UNSUPPORTED_NODE_TEXT.into(),
            Block::Unexpected => UNEXPECTED_NODE_TEXT.into(),
            Block::TableRow { .. } => panic!("The method to_markdown for Block::TableRow is not allowed. Please append rows to table as children.")
        }
    }
}

pub(crate) trait MarkdownBlockWithChildren {
    fn to_markdown(&self, children: &Vec<Block>, meta: &BlockMeta) -> String;
}

pub(crate) trait MarkdownBlockWithoutChildren {
    fn to_markdown(&self) -> String;
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BlockContent {
    pub rich_text: Vec<RichText>,
}

trait BlockChildren {
    fn to_markdown(&self, depth: usize) -> String;
}

impl BlockChildren for Vec<Block> {
    fn to_markdown(&self, depth: usize) -> String {
        self.iter()
            .enumerate()
            .map(|(order, block)| {
                format!(
                    "{}{}",
                    INDENT.repeat(depth),
                    block
                        .clone()
                        .with_meta(BlockMeta {
                            order: order + 1,
                            depth
                        })
                        .to_markdown()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
