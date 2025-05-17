use serde::Deserialize;

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct File {
    file: FileContent,
}
#[derive(Deserialize, Clone, Debug)]

struct FileContent {
    file: FileUrl,
    name: String,
}
#[derive(Deserialize, Clone, Debug)]

struct FileUrl {
    url: String,
}

impl MarkdownBlock for File {
    fn to_markdown(&self) -> String {
        format!("[{}]({})", self.file.name, self.file.file.url)
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
            serde_json::from_str(include_str!("../tests/block/file_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                [sample.pdf](https://pdfobject.com/pdf/sample.pdf)
            "#}
        )
    }
}
