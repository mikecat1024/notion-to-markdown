use comrak::{
    nodes::{AstNode, NodeCodeBlock, NodeValue},
    Arena,
};
use serde::Deserialize;

use crate::rich_text::RichText;

use super::{Block, BlockAst};

#[derive(Deserialize, Clone, Debug)]
pub struct Code {
    code: CodeContent,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CodeContent {
    pub rich_text: Vec<RichText>,
    pub language: String,
}

impl BlockAst for Code {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, _: &Vec<Block>) -> &'a AstNode<'a> {
        // Markdown formatting such as **bold** or ~~strikethrough~~ does not render inside code blocks.
        // All text is treated as plain code.

        Self::create_node(
            arena,
            NodeValue::CodeBlock(NodeCodeBlock {
                fence_char: b'`',
                fenced: true,
                info: self.code.language.clone(),
                fence_length: 3,
                fence_offset: 0,
                literal: self
                    .code
                    .rich_text
                    .iter()
                    .map(|rich_text| match rich_text {
                        RichText::Text { plain_text, .. } => plain_text,
                    })
                    .cloned()
                    .collect::<String>(),
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
        let paragraph: Block =
            serde_json::from_str(include_str!("../tests/block/code_response.json")).unwrap();

        let arena = Arena::new();
        let ast = paragraph.to_ast(&arena);

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
                ``` markdown
                this is markdown code
                ```
            "#}
        )
    }
}
