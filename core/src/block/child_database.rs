use serde::Deserialize;

use crate::escape_page_title;

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChildDatabase {
    child_database: ChildDatabaseContent,
}
#[derive(Deserialize, Clone, Debug)]

struct ChildDatabaseContent {
    title: String,
}

impl MarkdownBlock for ChildDatabase {
    fn to_markdown(&self) -> String {
        let title = escape_page_title(&self.child_database.title);

        format!(
            "[Database: {}]({}/{}.md)",
            self.child_database.title, title, title
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
            serde_json::from_str(include_str!("../tests/block/child_database_response.json"))
                .unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                [Database: this is child database](this_is_child_database/this_is_child_database.md)
            "#}
        )
    }
}
