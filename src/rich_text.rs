use std::cell::RefCell;

use comrak::arena_tree::Node;
use comrak::nodes::{Ast, AstNode, NodeCode, NodeLink, NodeValue};
use comrak::Arena;
use serde;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Annotations {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    code: bool,
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

        let mut is_annotated = false;

        let node = if annotations.code {
            is_annotated = true;
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
            arena.alloc(Node::new(RefCell::new(Ast::new(
                NodeValue::Raw(trimmed_content.clone()),
                Default::default(),
            ))))
        };

        // Wrap with NodeValue::Strong
        let node = if annotations.bold {
            if is_annotated || !plain_text.is_empty() {
                Self::wrap(arena, vec![node], NodeValue::Strong)
            } else {
                is_annotated = true;
                arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::Text("****".into()),
                    Default::default(),
                ))))
            }
        } else {
            node
        };

        // Wrap with NodeValue::Emph
        let node = if annotations.italic {
            if is_annotated || !plain_text.is_empty() {
                Self::wrap(arena, vec![node], NodeValue::Emph)
            } else {
                is_annotated = true;
                arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::Text("**".into()),
                    Default::default(),
                ))))
            }
        } else {
            node
        };

        // Wrap with NodeValue::Strikethrough
        let node = if annotations.strikethrough {
            if is_annotated || !plain_text.is_empty() {
                Self::wrap(arena, vec![node], NodeValue::Strikethrough)
            } else {
                arena.alloc(Node::new(RefCell::new(Ast::new(
                    NodeValue::Text("~~~~".into()),
                    Default::default(),
                ))))
            }
        } else {
            node
        };

        // Join leading and trailing spaces
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

        // Wrap with NodeValue::Link
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

    pub fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>) -> Vec<&'a AstNode<'a>> {
        match self {
            RichText::Text {
                plain_text,
                href,
                annotations,
            } => Self::text_to_ast(arena, plain_text, href, annotations),
        }
    }
}
