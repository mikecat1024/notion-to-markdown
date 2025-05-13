use comrak::{
    nodes::{AstNode, NodeValue},
    Arena,
};
use serde::Deserialize;

use super::{Block, BlockAst, BlockContent};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Paragraph {
    paragraph: BlockContent,
}

impl BlockAst for Paragraph {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, _: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(arena, NodeValue::Paragraph);

        let rich_text_asts: Vec<&'a AstNode<'a>> = self
            .paragraph
            .rich_text
            .iter()
            .map(|rich_text| rich_text.to_ast(&arena))
            .flatten()
            .collect();

        rich_text_asts.iter().for_each(|ast| wrapper.append(ast));

        wrapper
    }
}
