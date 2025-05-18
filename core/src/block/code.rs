use serde::Deserialize;

use crate::rich_text::{RichText, RichTextVec};

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
pub struct Code {
    code: CodeContent,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CodeContent {
    rich_text: Vec<RichText>,
    language: String,
}

impl MarkdownBlock for Code {
    fn to_markdown(&self) -> String {
        let inline = self.code.rich_text.to_markdown();
        format!("``` {}\n{}\n```", self.code.language, inline)
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
            serde_json::from_str(include_str!("../tests/block/code_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                ``` markdown
                this is markdown code
                ```
            "#}
        )
    }
}
