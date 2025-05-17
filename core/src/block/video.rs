use serde::Deserialize;

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Video {
    video: FileContent,
}
#[derive(Deserialize, Clone, Debug)]

struct FileContent {
    file: FileUrl,
}
#[derive(Deserialize, Clone, Debug)]

struct FileUrl {
    url: String,
}

impl MarkdownBlock for Video {
    fn to_markdown(&self) -> String {
        format!("[Video: {}]({})", self.video.file.url, self.video.file.url)
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
            serde_json::from_str(include_str!("../tests/block/video_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                [Video: https://pdfobject.com/pdf/sample.pdf](https://pdfobject.com/pdf/sample.pdf)
            "#}
        )
    }
}
