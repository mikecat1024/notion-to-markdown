use serde::Deserialize;

use crate::block::NOTION_ORIGIN;

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LinkToPage {
    link_to_page: LinkToPageContent,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
enum LinkToPageContent {
    DatabaseId { database_id: String },
    PageId { page_id: String },
}

impl MarkdownBlock for LinkToPage {
    fn to_markdown(&self) -> String {
        match &self.link_to_page {
            LinkToPageContent::DatabaseId { database_id } => {
                format!("<{}/{}>", NOTION_ORIGIN, database_id)
            }
            LinkToPageContent::PageId { page_id } => {
                format!("<{}/{}>", NOTION_ORIGIN, page_id)
            }
        }
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
            serde_json::from_str(include_str!("../tests/block/link_to_page_response.json"))
                .unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                <https://www.notion.so/XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX>
            "#}
        )
    }
}
