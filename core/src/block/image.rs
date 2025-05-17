use serde::Deserialize;

use super::MarkdownBlock;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Image {
    image: ImageContent,
}
#[derive(Deserialize, Clone, Debug)]

struct ImageContent {
    file: FileUrl,
}
#[derive(Deserialize, Clone, Debug)]

struct FileUrl {
    url: String,
}

impl MarkdownBlock for Image {
    fn to_markdown(&self) -> String {
        format!("![{}]({})", self.image.file.url, self.image.file.url)
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
            serde_json::from_str(include_str!("../tests/block/image_response.json")).unwrap();

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                ![https://picsum.photos/200/300](https://picsum.photos/200/300)
            "#}
        )
    }
}
