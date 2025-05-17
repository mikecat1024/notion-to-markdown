use serde::Deserialize;

use super::{BREADCRUMB_NODE_TEXT, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Breadcrumb {}

impl MarkdownBlock for Breadcrumb {
    fn to_markdown(&self) -> String {
        BREADCRUMB_NODE_TEXT.into()
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
            serde_json::from_str(include_str!("../tests/block/breadcrumb_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                <!-- breadcrumb block -->
            "#}
        )
    }
}
