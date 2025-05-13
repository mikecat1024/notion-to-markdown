use comrak::{
    nodes::{AstNode, NodeHeading, NodeValue},
    Arena,
};
use serde::Deserialize;

use super::{Block, BlockAst, BlockContent};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Heading1 {
    heading_1: BlockContent,
}

impl BlockAst for Heading1 {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, _: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(
            arena,
            NodeValue::Heading(NodeHeading {
                level: 1,
                setext: true,
            }),
        );

        let rich_text_asts: Vec<&'a AstNode<'a>> = self
            .heading_1
            .rich_text
            .iter()
            .map(|rich_text| rich_text.to_ast(&arena))
            .flatten()
            .collect();

        rich_text_asts.iter().for_each(|ast| wrapper.append(ast));

        wrapper
    }
}

// options.extension.strikethrough = true;
// options.extension.table = true;
// options.extension.tasklist = true;
// options.extension.autolink = true;
