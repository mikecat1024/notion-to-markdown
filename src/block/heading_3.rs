use comrak::{
    nodes::{AstNode, NodeHeading, NodeValue},
    Arena,
};
use serde::Deserialize;

use super::{Block, BlockAst, BlockContent};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Heading3 {
    heading_3: BlockContent,
}

impl BlockAst for Heading3 {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, _: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(
            arena,
            NodeValue::Heading(NodeHeading {
                level: 3,
                setext: true,
            }),
        );

        let rich_text_asts: Vec<&'a AstNode<'a>> = self
            .heading_3
            .rich_text
            .iter()
            .map(|rich_text| rich_text.to_ast(&arena))
            .flatten()
            .collect();

        rich_text_asts.iter().for_each(|ast| wrapper.append(ast));

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
            serde_json::from_str(include_str!("../tests/block/headline3_response.json")).unwrap();

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
                ### this is headline3
            "#}
        )
    }
}
