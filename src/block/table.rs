use comrak::{
    nodes::{AstNode, NodeTable, NodeValue, TableAlignment},
    Arena,
};
use serde::Deserialize;

use crate::rich_text::RichText;

use super::{Block, BlockAst};

#[derive(Deserialize, Clone, Debug)]
pub struct Table {
    table: TableContent,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TableContent {
    table_width: usize,
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

impl BlockAst for Table {
    fn to_ast<'a>(&self, arena: &'a Arena<AstNode<'a>>, children: &Vec<Block>) -> &'a AstNode<'a> {
        let rows: Vec<&TableRow> = children
            .iter()
            .filter_map(|child| match child {
                Block::TableRow { table_row, .. } => Some(table_row),
                _ => None,
            })
            .collect();

        let table = Self::create_node(
            arena,
            NodeValue::Table(NodeTable {
                alignments: vec![TableAlignment::Center; self.table.table_width as usize],
                num_columns: self.table.table_width,
                num_rows: rows.len(),
                num_nonempty_cells: 0,
            }),
        );

        rows.iter().enumerate().for_each(|(i, row)| {
            let row_ast = Self::create_node(arena, NodeValue::TableRow(i == 0));
            row.table_row.cells.iter().for_each(|cell| {
                let cell_ast = Self::create_node(arena, NodeValue::TableCell);
                cell.iter().for_each(|rich_text| {
                    let asts = rich_text.to_ast(arena);
                    println!("{:#?}", asts);

                    asts.iter().for_each(|ast| cell_ast.append(ast));
                });
                row_ast.append(cell_ast);
            });
            table.append(row_ast);
        });

        table
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
        let mut table: Block =
            serde_json::from_str(include_str!("../tests/block/table_response.json")).unwrap();
        let row1: Block =
            serde_json::from_str(include_str!("../tests/block/table_row_response.json")).unwrap();
        let row2: Block =
            serde_json::from_str(include_str!("../tests/block/table_row_response.json")).unwrap();

        table.append(row1);
        table.append(row2);

        let arena = Arena::new();
        let ast = table.to_ast(&arena);

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
                | this  | is  | table row |
                | :-: | :-: | :-: |
                | this  | is  | table row |
            "#}
        )
    }
}
