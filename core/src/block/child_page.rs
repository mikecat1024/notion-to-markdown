use serde::Deserialize;

use crate::escape_page_title;

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChildPage {
    child_page: ChildPageContent,
}
#[derive(Deserialize, Clone, Debug)]

struct ChildPageContent {
    title: String,
}

impl MarkdownBlock for ChildPage {
    fn to_markdown(&self) -> String {
        let title = escape_page_title(&self.child_page.title);

        format!(
            "[Child Page: {}]({})",
            self.child_page.title,
            format!("{}.md", title)
        )
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
            serde_json::from_str(include_str!("../tests/block/child_page_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                [Child Page: this is child page](this_is_child_page.md)
            "#}
        )
    }
}
