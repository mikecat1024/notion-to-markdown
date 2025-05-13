use comrak::{
    nodes::{AstNode, NodeHeading, NodeValue},
    Arena,
};
use serde::Deserialize;

use super::{Block, BlockAst, BlockContent};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Heading2 {
    heading_2: BlockContent,
}

impl BlockAst for Heading2 {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, _: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(
            arena,
            NodeValue::Heading(NodeHeading {
                level: 2,
                setext: true,
            }),
        );

        let rich_text_asts: Vec<&'a AstNode<'a>> = self
            .heading_2
            .rich_text
            .iter()
            .map(|rich_text| rich_text.to_ast(&arena))
            .flatten()
            .collect();

        rich_text_asts.iter().for_each(|ast| wrapper.append(ast));

        wrapper
    }
}
