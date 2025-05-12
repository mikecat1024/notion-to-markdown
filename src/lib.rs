use block::Block;
use comrak::{format_commonmark, Arena, Options};
use page::Page;

mod block;
mod page;
mod rich_text;

pub fn main() {
    let block: Block =
        serde_json::from_str(include_str!("tests/block/paragraph_response.json")).unwrap();

    let page = Page::from_blocks(vec![block]);

    let arena = Arena::new();

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
