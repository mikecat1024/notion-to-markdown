use comrak::{
    nodes::{AstNode, NodeValue},
    Arena,
};
use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{Block, BlockAstWithChildren, BlockContent};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Quote {
    quote: BlockContent,
}

impl BlockAstWithChildren for Quote {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, _: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(arena, NodeValue::BlockQuote);
        let paragraph = Self::create_node(arena, NodeValue::Paragraph);

        self.quote
            .rich_text
            .to_ast(arena)
            .iter()
            .for_each(|ast| paragraph.append(ast));

        wrapper.append(paragraph);

        wrapper
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
            serde_json::from_str(include_str!("../tests/block/quote_response.json")).unwrap();

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
                > this is quote
            "#}
        )
    }
}
