use block::Block;
use comrak::{format_commonmark, Arena, Options};
use page::Page;

mod block;
mod page;
mod utils;
mod rich_text;

pub fn main() {
    let headline1: Block =
        serde_json::from_str(include_str!("tests/block/headline1_response.json")).unwrap();
    let headline2: Block =
        serde_json::from_str(include_str!("tests/block/headline2_response.json")).unwrap();
    let headline3: Block =
        serde_json::from_str(include_str!("tests/block/headline3_response.json")).unwrap();
    let paragraph: Block =
        serde_json::from_str(include_str!("tests/block/paragraph_response.json")).unwrap();

    let mut numbered_parent_item: Block =
        serde_json::from_str(include_str!("tests/block/numbered_list_item_response.json")).unwrap();
    let numbered_child_item1: Block =
        serde_json::from_str(include_str!("tests/block/numbered_list_item_response.json")).unwrap();
    let numbered_child_item2: Block =
        serde_json::from_str(include_str!("tests/block/numbered_list_item_response.json")).unwrap();

    numbered_parent_item.append(numbered_child_item1);
    numbered_parent_item.append(numbered_child_item2);

    let arena = Arena::new();

    let page = Page::from_blocks(vec![
        headline1,
        headline2,
        headline3,
        paragraph,
        numbered_parent_item,
    ]);

    let mut options = Options::default();

    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;

    let ast = page.to_ast(&arena);

    let mut output = vec![];
    format_commonmark(ast, &options, &mut output).unwrap();

    println!("{}", String::from_utf8(output).unwrap())
}
