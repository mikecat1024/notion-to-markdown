use serde::Deserialize;

use crate::rich_text::RichTextVec;

use super::{INDENT, MarkdownBlock, heading_1::Heading1, heading_2::Heading2, heading_3::Heading3};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TableOfContents {
    #[serde(skip_serializing, default)]
    headings: Vec<Heading>,
}

#[derive(Deserialize, Clone, Debug)]
enum Heading {
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
}

impl MarkdownBlock for TableOfContents {
    fn to_markdown(&self) -> String {
        self.headings
            .iter()
            .map(|heading| match heading {
                Heading::Heading1(item) => {
                    format!(
                        "{}- [{}](#{})",
                        INDENT.repeat(0),
                        item.heading_1.rich_text.to_markdown(),
                        item.heading_1.rich_text.to_markdown()
                    )
                }
                Heading::Heading2(item) => {
                    format!(
                        "{}- [{}](#{})",
                        INDENT.repeat(1),
                        item.heading_2.rich_text.to_markdown(),
                        item.heading_2.rich_text.to_markdown()
                    )
                }
                Heading::Heading3(item) => {
                    format!(
                        "{}- [{}](#{})",
                        INDENT.repeat(2),
                        item.heading_3.rich_text.to_markdown(),
                        item.heading_3.rich_text.to_markdown()
                    )
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::MarkdownBlock;
    use crate::{
        heading_1::Heading1,
        heading_2::Heading2,
        heading_3::Heading3,
        table_of_contents::{Heading, TableOfContents},
    };

    #[test]
    fn test_to_markdown() {
        let mut item: TableOfContents = serde_json::from_str(include_str!(
            "../tests/block/table_of_contents_response.json"
        ))
        .unwrap();

        let heading1: Heading1 =
            serde_json::from_str(include_str!("../tests/block/headline1_response.json")).unwrap();

        let heading2: Heading2 =
            serde_json::from_str(include_str!("../tests/block/headline2_response.json")).unwrap();
        let heading3: Heading3 =
            serde_json::from_str(include_str!("../tests/block/headline3_response.json")).unwrap();

        item.headings.push(Heading::Heading1(heading1));
        item.headings.push(Heading::Heading2(heading2.clone()));
        item.headings.push(Heading::Heading3(heading3));
        item.headings.push(Heading::Heading2(heading2));

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                    - [this is headline1](#this is headline1)
                      - [this is headline2](#this is headline2)
                        - [this is headline3](#this is headline3)
                      - [this is headline2](#this is headline2)
                "#}
        )
    }
}
