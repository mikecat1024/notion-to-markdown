use comrak::{
    nodes::{AstNode, NodeLink, NodeValue},
    Arena,
};
use serde::Deserialize;

use super::BlockAstWithoutChildren;

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

impl BlockAstWithoutChildren for Image {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
        Self::create_node(
            arena,
            NodeValue::Image(NodeLink {
                url: self.image.file.url.clone(),
                title: String::new(), // The title always empty string
            }),
        )
    }
}

#[cfg(test)]
mod test {

    use comrak::{format_commonmark, Arena, Options};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::Block;

    #[test]
    fn test_to_markdown() {
        let item: Block =
            serde_json::from_str(include_str!("../tests/block/image_response.json")).unwrap();

        let arena = Arena::new();
        let ast = item.to_ast(&arena);

        let mut options = Options::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;

        let mut output = vec![];
        format_commonmark(ast, &options, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            indoc! {r#"
                ![](https://picsum.photos/200/300)
            "#}
        )
    }
}
