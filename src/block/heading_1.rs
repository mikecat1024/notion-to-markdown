use comrak::{
    nodes::{AstNode, NodeHeading, NodeValue},
    Arena,
};
use serde::Deserialize;

use crate::rich_text::RichTextVec;

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

        self.heading_1
            .rich_text
            .to_ast(arena)
            .iter()
            .for_each(|ast| wrapper.append(ast));

        wrapper
    }
}

#[cfg(test)]
mod test {

    use comrak::{format_commonmark, Arena, Options};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::Block;

    #[test]
    fn test_to_markdown() {
        let paragraph: Block =
            serde_json::from_str(include_str!("../tests/block/headline1_response.json")).unwrap();

        let arena = Arena::new();
        let ast = paragraph.to_ast(&arena);

        let mut options = Options::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;

        let mut output = vec![];
        format_commonmark(ast, &options, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            indoc! {r#"
                # this is headline1
            "#}
        )
    }
}
