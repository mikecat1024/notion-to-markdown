use client::NotionClient;
use dotenvy::dotenv;
use notion_to_markdown_core::BlockChildren;
use std::{env, fs};

const OUTPUT_PATH: &str = "output.md";
const TOKEN_ENV_VAR: &str = "NOTION_TOKEN";

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    let token = env::var(TOKEN_ENV_VAR).expect("NOTION_TOKEN is not set");

    let markdown = NotionClient::new(token)
        .retrieve_block_children("2ed53222ff1e800390c6d4d47f1c12fe".into(), None, None)
        .await
        .unwrap()
        .to_markdown(0);

    fs::write(OUTPUT_PATH, markdown).unwrap();
    println!("Wrote markdown to {}", OUTPUT_PATH);
}
