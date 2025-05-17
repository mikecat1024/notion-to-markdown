use serde::Deserialize;

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
pub struct Equation {
    equation: EquationContent,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EquationContent {
    expression: String,
}

impl MarkdownBlock for Equation {
    fn to_markdown(&self) -> String {
        format!("$$\n{}\n$$", self.equation.expression)
    }
}

#[cfg(test)]
mod test {

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::Block;

    #[test]
    fn test_to_markdown() {
        let item: Block =
            serde_json::from_str(include_str!("../tests/block/equation_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                $$
                x + y = 1 \\ x^2 + y^1 = 1
                $$
            "#}
        )
    }
}
