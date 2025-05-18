use serde;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum RichText {
    Text {
        plain_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        annotations: Annotations,
    },
    Mention {
        mention: Mention,
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        annotations: Annotations,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum Mention {
    LinkMention(LinkMention),
    User(UserMention),
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct UserMention {
    user: UserMentionContent,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct UserMentionContent {
    name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct LinkMention {
    link_mention: LinkMentionContent,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct LinkMentionContent {
    title: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Annotations {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    code: bool,
    underline: bool,
}

impl Default for Annotations {
    fn default() -> Annotations {
        Annotations {
            bold: false,
            italic: false,
            strikethrough: false,
            code: false,
            underline: false,
        }
    }
}

impl RichText {
    fn bold(text: &str) -> String {
        format!("**{}**", text)
    }

    fn italic(text: &str) -> String {
        format!("_{}_", text)
    }

    fn strikethrough(text: &str) -> String {
        format!("~~{}~~", text)
    }

    fn code(text: &str) -> String {
        format!("`{}`", text)
    }

    fn underline(text: &str) -> String {
        format!("<u>{}</u>", text)
    }

    fn link(text: &str, url: &str) -> String {
        format!("[{}]({})", text, url)
    }

    fn text_to_markdown(
        plain_text: &String,
        href: &Option<String>,
        annotations: &Annotations,
    ) -> String {
        let leading_space = plain_text
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        let trimmed_plain_text = plain_text.trim().to_string();
        let trailing_space =
            if trimmed_plain_text.is_empty() && plain_text.chars().all(|c| c.is_whitespace()) {
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

        let mut markdown_text = if annotations.code {
            Self::code(&plain_text)
        } else {
            trimmed_plain_text
        };

        if annotations.bold {
            markdown_text = Self::bold(&markdown_text)
        }

        if annotations.italic {
            markdown_text = Self::italic(&markdown_text)
        }

        if annotations.strikethrough {
            markdown_text = Self::strikethrough(&markdown_text)
        }

        if annotations.underline {
            markdown_text = Self::underline(&markdown_text)
        }

        if !annotations.code {
            markdown_text = format!("{}{}{}", leading_space, markdown_text, trailing_space)
        }

        if let Some(url) = href {
            markdown_text = Self::link(&markdown_text, url)
        }

        markdown_text
    }

    fn link_mention_to_markdown(
        title: &String,
        href: &Option<String>,
        annotations: &Annotations,
    ) -> String {
        Self::text_to_markdown(title, href, annotations)
    }

    pub(crate) fn to_markdown(&self) -> String {
        match self {
            RichText::Text {
                plain_text,
                href,
                annotations,
            } => Self::text_to_markdown(plain_text, href, annotations),
            RichText::Mention {
                mention,
                href,
                annotations,
            } => match mention {
                Mention::LinkMention(item) => {
                    Self::link_mention_to_markdown(&item.link_mention.title, href, annotations)
                }
                Mention::User(item) => {
                    Self::link_mention_to_markdown(&item.user.name, href, annotations)
                }
            },
        }
    }
}

pub trait RichTextVec {
    fn to_markdown(&self) -> String;
}

impl RichTextVec for Vec<RichText> {
    fn to_markdown(&self) -> String {
        self.iter()
            .map(|rich_text| rich_text.to_markdown())
            .collect()
    }
}
