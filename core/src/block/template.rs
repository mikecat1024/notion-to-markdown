use serde::Deserialize;

use super::{BREADCRUMB_NODE_TEXT, MarkdownBlock, TEMPLATE_NODE_TEXT};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Template {}

impl MarkdownBlock for Template {
    fn to_markdown(&self) -> String {
        TEMPLATE_NODE_TEXT.into()
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
            serde_json::from_str(include_str!("../tests/block/template_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                <!-- template block -->
            "#}
        )
    }
}
