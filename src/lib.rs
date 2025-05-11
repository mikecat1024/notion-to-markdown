use block::Block;
use comrak::Arena;

mod block;
mod rich_text;
mod test_utils;
mod utils;

pub fn construct_ast(response: &str) {
    let arena = Arena::new();

    let block: Block = serde_json::from_str(response).unwrap();
    block.to_ast(&arena);
}
