use serde::Deserialize;

use crate::{
    block::INDENT,
    rich_text::{RichText, RichTextVec},
};

use super::{Block, BlockMeta, MarkdownBlockWithChildren};

#[derive(Deserialize, Clone, Debug)]
pub struct Table {
    #[serde(skip_serializing, default)]
    children: Vec<Block>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TableRow {
    table_row: TableRowContent,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
struct TableRowContent {
    pub cells: Vec<Vec<RichText>>,
}

impl Table {
    pub(crate) fn append(&mut self, child: Block) {
        self.children.push(child);
    }
}

impl MarkdownBlockWithChildren for Table {
    fn to_markdown(&self, meta: &BlockMeta) -> String {
        let table: Vec<Vec<String>> = self
            .children
            .iter()
            .filter_map(|child| match child {
                Block::TableRow { table_row, .. } => Some(
                    table_row
                        .table_row
                        .cells
                        .iter()
                        .map(|x| x.to_markdown())
                        .collect::<Vec<String>>(),
                ),
                _ => None,
            })
            .collect();

        if table.is_empty() {
            return String::new();
        }

        let columns_count = table[0].len();
        let mut columns_widths = vec![0; columns_count];
        for row in &table {
            for (i, cell) in row.iter().enumerate().take(columns_count) {
                let w: usize = cell.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum();
                columns_widths[i] = columns_widths[i].max(w);
            }
        }

        let pad_cell = |cell: &str, target: usize| {
            let cw: usize = cell.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum();
            let padding = target.saturating_sub(cw);
            format!("{}{}", cell, " ".repeat(padding))
        };

        let format_row = |row: &[String]| {
            let cells: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, cell)| pad_cell(cell, columns_widths[i]))
                .collect();
            format!("| {} |", cells.join(" | "))
        };

        let mut markdown = String::new();
        markdown.push_str(&format_row(&table[0]));
        markdown.push('\n');

        let separators: Vec<String> = columns_widths
            .iter()
            .map(|&w| "-".repeat(w.max(3)))
            .collect();
        markdown.push_str(&format!("| {} |\n", separators.join(" | ")));

        for row in &table[1..] {
            let mut cells = row.clone();
            cells.resize(columns_count, String::new());
            markdown.push_str(&format_row(&cells));
            markdown.push('\n');
        }

        format!("{}{}", INDENT.repeat(meta.depth), markdown)
    }
}

#[cfg(test)]
mod test {

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::block::Block;

    #[test]
    fn test_to_markdown() {
        let mut table: Block =
            serde_json::from_str(include_str!("../tests/block/table_response.json")).unwrap();
        let row1: Block =
            serde_json::from_str(include_str!("../tests/block/table_row_response.json")).unwrap();
        let row2: Block =
            serde_json::from_str(include_str!("../tests/block/table_row_response.json")).unwrap();

        table.append(row1);
        table.append(row2);

        assert_eq!(
            table.to_markdown(),
            indoc! {r#"
                | this  | is  | table row |
                | ----- | --- | --------- |
                | this  | is  | table row |
            "#}
        )
    }
}
