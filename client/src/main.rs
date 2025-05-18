use client::NotionClient;
use notion_to_markdown_core::BlockChildren;

#[tokio::main]
async fn main() {
    println!(
        "{}",
        NotionClient::new("".into())
            .retrieve_block_children("".into(), None, None)
            .await
            .unwrap()
            .to_markdown(0)
    )
}
