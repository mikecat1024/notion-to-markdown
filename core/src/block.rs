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
    NumberedListItem(NumberedListItem),
    BulletedListItem(BulletedListItem),
    Paragraph(Paragraph),
    Pdf(Pdf),
    Quote(Quote),
    Code(Code),
    #[serde(rename = "heading_1")]
    Heading1(Heading1),
    #[serde(rename = "heading_2")]
    Heading2(Heading2),
    #[serde(rename = "heading_3")]
    Heading3(Heading3),
    Image(Image),
    Divider(Divider),
    File(File),
    ToDo(ToDo),
    Bookmark(Bookmark),
    Callout(Callout),
    ChildPage(ChildPage),
    Equation(Equation),
    Table(Table),
    TableRow(TableRow),
    Embed(Embed),
    LinkPreview(LinkPreview),
    ChildDatabase(ChildDatabase),
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
            Block::NumberedListItem(item) => Block::NumberedListItem(item.with_meta(meta)),
            Block::BulletedListItem(item) => Block::BulletedListItem(item.with_meta(meta)),
            Block::ToDo(item) => Block::ToDo(item.with_meta(meta)),
            Block::Table(item) => Block::Table(item.with_meta(meta)),
            _ => self,
        }
    }

    pub fn append(&mut self, child: Block) {
        match self {
            Block::Table(item) => item.append(child),
            Block::ToDo(item) => item.append(child),
            Block::BulletedListItem(item) => item.append(child),
            Block::NumberedListItem(item) => item.append(child),
            _ => {}
        }
    }

    pub fn to_markdown(&self) -> String {
        match &self {
            Block::NumberedListItem(item) => item.to_markdown(),
            Block::BulletedListItem(item) => item.to_markdown(),
            Block::ToDo(item) => item.to_markdown(),
            Block::Table(item) => item.to_markdown(),
            Block::Paragraph(item) => item.to_markdown(),
            Block::Pdf(item) => item.to_markdown(),
            Block::Quote(item) => item.to_markdown(),
            Block::Code(item) => item.to_markdown(),
            Block::Heading1(item) => item.to_markdown(),
            Block::Heading2(item) => item.to_markdown(),
            Block::Heading3(item) => item.to_markdown(),
            Block::Image(item) => item.to_markdown(),
            Block::Divider(item) => item.to_markdown(),
            Block::File(item) => item.to_markdown(),
            Block::Bookmark(item) => item.to_markdown(),
            Block::Equation(item) => item.to_markdown(),
            Block::Callout(item) => item.to_markdown(),
            Block::ChildPage(item) => item.to_markdown(),
            Block::Embed(item) => item.to_markdown(),
            Block::LinkPreview(item) => item.to_markdown(),
            Block::ChildDatabase(item) => item.to_markdown(),
            Block::Unsupported => UNSUPPORTED_NODE_TEXT.into(),
            Block::Unexpected => UNEXPECTED_NODE_TEXT.into(),
            Block::TableRow(_) => panic!(
                "The method to_markdown for Block::TableRow is not allowed. Please append rows to table as children."
            ),
        }
    }
}

trait MarkdownBlock {
    fn to_markdown(&self) -> String;
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) struct BlockContent {
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
