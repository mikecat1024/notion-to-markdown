use serde::Deserialize;

use super::MarkdownBlockWithoutChildren;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Divider {}

impl MarkdownBlockWithoutChildren for Divider {
    fn to_markdown(&self) -> String {
        "-----".to_string()
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
            serde_json::from_str(include_str!("../tests/block/divider_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                -----
            "#}
        )
    }
}
