use std::cell::RefCell;

use comrak::arena_tree::Node;
use comrak::nodes::{Ast, AstNode, NodeCode, NodeLink, NodeValue};
use comrak::Arena;
use serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RichText {
    Text {
        plain_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Annotations,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub code: bool,
}

impl Default for Annotations {
    fn default() -> Annotations {
        Annotations {
            bold: false,
            italic: false,
            strikethrough: false,
            code: false,
        }
    }
}

impl RichText {
    fn wrap<'a>(
        arena: &'a Arena<AstNode<'a>>,
        children: Vec<&'a AstNode<'a>>,
        value: NodeValue,
    ) -> &'a AstNode<'a> {
        let parent = arena.alloc(Node::new(RefCell::new(Ast::new(value, Default::default()))));
        children.iter().for_each(|node| parent.append(node));
        parent
    }

    fn text_to_ast<'a>(
        arena: &'a Arena<AstNode<'a>>,
        plain_text: &String,
        href: &Option<String>,
        annotations: &Annotations,
    ) -> Vec<&'a AstNode<'a>> {
        let leading_space = plain_text
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        let trimmed_content = plain_text.trim().to_string();
        let trailing_space =
            if trimmed_content.is_empty() && plain_text.chars().all(|c| c.is_whitespace()) {
                String::new()
            } else {
                plain_text
                    .chars()
                    .rev()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
            };

        let node = if annotations.code {
            arena.alloc(Node::new(RefCell::new(Ast::new(
                // In Comrak, NodeCode.literal causes a panic in format_commonmark when its length is 0,
                // so if the literal is an empty string, use NodeValue::Raw("``") instead.
                //
                // See https://github.com/kivikakk/comrak/blob/ce1837224bc25b4133068771fab43889ad32fd7e/src/cm.rs#L675
                {
                    if plain_text.is_empty() {
                        NodeValue::Raw("``".into())
                    } else {
                        NodeValue::Code(NodeCode {
                            num_backticks: 1,
                            literal: plain_text.clone(), // Code block contains spaces.
                        })
                    }
                },
                Default::default(),
            ))))
        } else {
            let special_chars = [
                '*', '_', '#', '+', '-', '=', '!', '[', ']', '(', ')', '{', '}', '<', '>', '&',
                '\'', '"', '\\',
            ];

            if trimmed_content.chars().any(|c| special_chars.contains(&c)) {
                // The inability to manipulate Ast.content causes symbols
                // and other characters to be escaped in NodeValue::Text.
                // To prevent this, NodeValue::Raw is used.
                arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::Raw(trimmed_content.clone()),
                    Default::default(),
                ))))
            } else {
                arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::Text(trimmed_content.clone()),
                    Default::default(),
                ))))
            }
        };

        println!("{:#?}", node);

        let node = if annotations.bold {
            Self::wrap(arena, vec![node], NodeValue::Strong)
        } else {
            node
        };

        let node = if annotations.italic {
            Self::wrap(arena, vec![node], NodeValue::Emph)
        } else {
            node
        };

        let node = if annotations.strikethrough {
            Self::wrap(arena, vec![node], NodeValue::Strikethrough)
        } else {
            node
        };

        let children = if annotations.code {
            vec![node]
        } else {
            let mut children: Vec<&AstNode> = vec![];

            if !leading_space.is_empty() {
                children.push(arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::Text(leading_space.clone()),
                    Default::default(),
                )))));
            }

            children.push(node);

            if !trailing_space.is_empty() {
                children.push(arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::Text(trailing_space.clone()),
                    Default::default(),
                )))));
            }

            children
        };

        if let Some(url) = href {
            vec![Self::wrap(
                arena,
                children,
                NodeValue::Link(NodeLink {
                    url: url.to_string(),
                    title: String::new(),
                }),
            )]
        } else {
            if annotations.code {
                vec![node]
            } else {
                children
            }
        }
    }

    pub fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>) -> Vec<&AstNode<'a>> {
        match self {
            RichText::Text {
                plain_text,
                href,
                annotations,
            } => Self::text_to_ast(arena, plain_text, href, annotations),
        }
    }
}

#[cfg(test)]
mod tests {

    use comrak::{
        arena_tree::Node,
        format_commonmark,
        nodes::{Ast, NodeValue},
        parse_document, Arena,
    };
    use rstest::rstest;
    use std::cell::RefCell;

    use crate::{
        rich_text::{Annotations, RichText},
        test_utils::ast_eq,
        utils::gfm_options,
    };

    fn annotate_text(text: String, annotations: Annotations, href: Option<String>) -> String {
        let leading_space = text
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();

        let trimmed_content = text.trim().to_string();

        let trailing_space =
            if trimmed_content.is_empty() && text.chars().all(|c| c.is_whitespace()) {
                String::new()
            } else {
                text.chars()
                    .rev()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
            };

        let mut expected_markdown = trimmed_content;

        if annotations.code {
            expected_markdown = format!("`{}`", text);
        }
        if annotations.bold {
            expected_markdown = format!("**{}**", expected_markdown);
        }
        if annotations.italic {
            expected_markdown = format!("*{}*", expected_markdown);
        }
        if annotations.strikethrough {
            expected_markdown = format!("~~{}~~", expected_markdown);
        }

        if let Some(url) = href {
            if annotations.code {
                format!("[{}]({})", expected_markdown, url)
            } else {
                format!(
                    "[{}{}{}]({})",
                    leading_space, expected_markdown, trailing_space, url
                )
            }
        } else {
            if annotations.code {
                expected_markdown
            } else {
                format!("{}{}{}", leading_space, expected_markdown, trailing_space)
            }
        }
    }

    #[rstest]
    fn test_rich_text_to_ast(
        // Note: This test assumes the input text is "Hello World" without punctuation or being empty.
        // Variations like "Hello World!" or an empty string may cause this test to fail due to differences
        // in how the AST parser handles punctuation and empty content.
        // The behavior for such cases will be tested separately in `test_rich_text_to_markdown`.
        #[values("Hello World")] text: String,
        #[values(None, Some("https://example.com".to_string()))] href: Option<String>,
        #[values(false, true)] code: bool,
        #[values(false, true)] bold: bool,
        #[values(false, true)] italic: bool,
        #[values(false, true)] strikethrough: bool,
    ) {
        let rich_text = RichText::Text {
            plain_text: text.clone(),
            href: href.clone(),
            annotations: Annotations {
                bold,
                italic,
                strikethrough,
                code,
            },
        };

        let expected_markdown = annotate_text(
            text,
            Annotations {
                bold,
                italic,
                strikethrough,
                code,
            },
            href,
        );

        let arena = Arena::new();

        let expected_document = parse_document(&arena, &expected_markdown, &gfm_options());

        let notion_nodes = rich_text.to_ast(&arena);
        let notion_paragraph = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::Paragraph,
            Default::default(),
        ))));

        notion_nodes
            .iter()
            .for_each(|node| notion_paragraph.append(node));

        let notion_document = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::Document,
            Default::default(),
        ))));

        notion_document.append(&notion_paragraph);

        assert!(
            ast_eq(notion_document, expected_document),
            "Expected {:#?}, but found {:#?}",
            expected_document,
            notion_document
        );
    }

    #[rstest]
    fn test_rich_text_to_markdown(
        // If text is such as " Hello World! " or " Hello World!", the test cases should be fails
        // because Comrak's serializer adds padding spaces around inline code spans that start
        // or end with a space character, in order to preserve their exact contents.
        //
        // However, according to the CommonMark (and GFM) specification,
        // leading and trailing spaces in code spans are trimmed or preserved（in the case of non unicode spacing),
        // unless the span consists entirely of spaces.
        //
        // As a result, Comrak's output differs from the GFM-compliant expectation.
        //
        // See https://github.com/kivikakk/comrak/blob/ce1837224bc25b4133068771fab43889ad32fd7e/src/cm.rs#L665
        #[values("Hello World!", " ", "", "![Hello World!](https://example.com)")] text: String,
        #[values(None, Some("https://example.com".into()))] href: Option<String>,
        #[values(false, true)] code: bool,
        #[values(false, true)] bold: bool,
        #[values(false, true)] italic: bool,
        #[values(false, true)] strikethrough: bool,
    ) {
        let rich_text = RichText::Text {
            plain_text: text.clone(),
            href: href.clone(),
            annotations: Annotations {
                bold,
                italic,
                strikethrough,
                code,
            },
        };

        let expected_markdown = annotate_text(
            text,
            Annotations {
                bold,
                italic,
                strikethrough,
                code,
            },
            href,
        );

        let arena = Arena::new();

        let notion_nodes = rich_text.to_ast(&arena);
        let notion_paragraph = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::Paragraph,
            Default::default(),
        ))));

        notion_nodes
            .iter()
            .for_each(|node| notion_paragraph.append(node));

        let notion_document = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::Document,
            Default::default(),
        ))));

        notion_document.append(&notion_paragraph);

        let mut notion_markdown = vec![];
        let _ = format_commonmark(&notion_document, &gfm_options(), &mut notion_markdown);

        assert_eq!(
            {
                if expected_markdown.is_empty() {
                    "".into()
                } else {
                    expected_markdown + "\n"
                }
            },
            String::from_utf8(notion_markdown).unwrap(),
        );
    }
}
