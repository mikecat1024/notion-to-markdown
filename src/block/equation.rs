use comrak::{
    nodes::{AstNode, NodeMath, NodeValue},
    Arena,
};
use serde::Deserialize;

use super::{Block, BlockAst};

#[derive(Deserialize, Clone, Debug)]
pub struct Equation {
    equation: EquationContent,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EquationContent {
    expression: String,
}

impl BlockAst for Equation {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, _: &Vec<Block>) -> &'a AstNode<'a> {
        Self::create_node(
            arena,
            NodeValue::Math(NodeMath {
                dollar_math: true,
                display_math: true,
                literal: self.equation.expression.clone(),
            }),
        )
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
            serde_json::from_str(include_str!("../tests/block/equation_response.json")).unwrap();

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
                $$x + y = 1 \\ x^2 + y^1 = 1$$
            "#}
        )
    }
}
