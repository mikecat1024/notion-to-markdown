use serde::Deserialize;

use crate::{
    block::BlockChildren,
    rich_text::{RichText, RichTextVec},
};

use super::{Block, BlockMeta, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
pub struct ToDo {
    to_do: ToDoContent,
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
    #[serde(skip_serializing, default)]
    meta: BlockMeta,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ToDoContent {
    rich_text: Vec<RichText>,
    checked: bool,
}

impl ToDo {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }

    pub(crate) fn with_meta(self, meta: BlockMeta) -> ToDo {
        ToDo {
            meta,
            children: self.children,
            to_do: self.to_do,
        }
    }
}

impl MarkdownBlock for ToDo {
    fn to_markdown(&self) -> String {
        let checked_x = if self.to_do.checked { "x" } else { " " };

        if self.children.is_empty() {
            format!("- [{}] {}", checked_x, self.to_do.rich_text.to_markdown())
        } else {
            let children_markdown = self.children.to_markdown(self.meta.depth + 1);
            format!(
                "- [{}] {}\n{}",
                checked_x,
                self.to_do.rich_text.to_markdown(),
                children_markdown
            )
        }
    }
}

#[cfg(test)]
mod test {

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::Block;

    #[test]
    fn test_to_markdown() {
        let mut unchecked: Block =
            serde_json::from_str(include_str!("../tests/block/unchecked_to_do_response.json"))
                .unwrap();
        let child = unchecked.clone();
        let checked: Block =
            serde_json::from_str(include_str!("../tests/block/checked_to_do_response.json"))
                .unwrap();

        unchecked.append(child);
        unchecked.append(checked);

        assert_eq!(
            unchecked.to_markdown() + "\n",
            indoc! {r#"
                - [ ] this is to do item
                  - [ ] this is to do item
                  - [x] this is to do item

            "#}
        )
    }
}
