use serde::Deserialize;

use crate::{ChildLinkTarget, MarkdownRenderOptions, block::NOTION_ORIGIN, escape_page_title};

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChildDatabase {
    id: String,
    child_database: ChildDatabaseContent,
}
#[derive(Deserialize, Clone, Debug)]

struct ChildDatabaseContent {
    title: String,
}

impl MarkdownBlock for ChildDatabase {
    fn to_markdown(&self) -> String {
        let options = MarkdownRenderOptions::default();

        let link = match options.child_database_link_target {
            ChildLinkTarget::MarkdownFile => {
                let title = escape_page_title(&self.child_database.title);

                format!("{}.md", title)
            }
            ChildLinkTarget::Notion => format!("{}/{}", NOTION_ORIGIN, self.id),
        };

        format!("[Child Database: {}]({})", self.child_database.title, link)
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
            serde_json::from_str(include_str!("../tests/block/child_database_response.json"))
                .unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                [Child Database: this is child database](https://www.notion.so/XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX)
            "#}
        )
    }
}
