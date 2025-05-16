use serde::Deserialize;

use crate::rich_text::{RichText, RichTextVec};

use super::MarkdownBlockWithoutChildren;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Callout {
    callout: CalloutContent,
}

#[derive(Deserialize, Clone, Debug)]
struct CalloutContent {
    icon: IconContent,
    rich_text: Vec<RichText>,
}

#[derive(Deserialize, Clone, Debug)]

struct IconContent {
    emoji: String,
}

impl MarkdownBlockWithoutChildren for Callout {
    fn to_markdown(&self) -> String {
        if self.callout.icon.emoji.is_empty() {
            format!("> {}", self.callout.rich_text.to_markdown())
        } else {
            format!(
                "> {} {}",
                self.callout.icon.emoji,
                self.callout.rich_text.to_markdown()
            )
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
            serde_json::from_str(include_str!("../tests/block/callout_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                > ❗ this is callout
            "#}
        )
    }
}
