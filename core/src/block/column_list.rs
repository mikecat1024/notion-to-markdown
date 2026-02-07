use serde::Deserialize;

use super::{Block, BlockChildren, BlockMeta, MarkdownBlock};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ColumnList {
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
    #[serde(skip_serializing, default)]
    meta: BlockMeta,
}

impl ColumnList {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }

    pub(crate) fn with_meta(self, meta: BlockMeta) -> ColumnList {
        ColumnList {
            meta,
            children: self.children,
        }
    }
}

impl MarkdownBlock for ColumnList {
    fn to_markdown(&self) -> String {
        self.children.to_markdown(self.meta.depth)
    }
}

#[cfg(test)]
mod test {

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::Block;

    #[test]
    fn test_to_markdown() {
        let mut item: Block =
            serde_json::from_str(include_str!("../tests/block/column_list_response.json")).unwrap();

        let mut column1: Block =
            serde_json::from_str(include_str!("../tests/block/column_response.json")).unwrap();
        let mut column2 = column1.clone();

        let child: Block =
            serde_json::from_str(include_str!("../tests/block/paragraph_response.json")).unwrap();

        column1.append(child.clone());
        column2.append(child.clone());
        item.append(column1);
        item.append(column2);

        assert_eq!(
            item.to_markdown() + "\n",
            indoc! {r#"
                this is paragraph

                this is paragraph


            "#}
        )
    }
}
