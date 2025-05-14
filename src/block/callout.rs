use comrak::{
    nodes::{AstNode, NodeValue},
    Arena,
};
use serde::Deserialize;

use crate::rich_text::RichText;

use super::{Block, BlockAst};

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

impl BlockAst for Callout {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, _: &Vec<Block>) -> &'a AstNode<'a> {
        let wrapper = Self::create_node(arena, NodeValue::BlockQuote);
        let paragraph = Self::create_node(arena, NodeValue::Paragraph);

        let rich_text_asts: Vec<&'a AstNode<'a>> = self
            .callout
            .rich_text
            .iter()
            .enumerate()
            .map(|(i, rich_text)| {
                if i == 0 {
                    match rich_text {
                        RichText::Text {
                            plain_text,
                            href,
                            annotations,
                        } => RichText::Text {
                            plain_text: format!("{} {}", self.callout.icon.emoji, plain_text),
                            href: href.clone(),
                            annotations: annotations.clone(),
                        }
                        .to_ast(&arena),
                    }
                } else {
                    rich_text.to_ast(&arena)
                }
            })
            .flatten()
            .collect();

        rich_text_asts.iter().for_each(|ast| paragraph.append(ast));
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
            serde_json::from_str(include_str!("../tests/block/callout_response.json")).unwrap();

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
                > ❗ this is callout
            "#}
        )
    }
}
