use client::NotionClient;
use notion_to_markdown_core::BlockChildren;
use std::{env, fs};

const OUTPUT_PATH: &str = "output.md";
const TOKEN_ENV_VAR: &str = "NOTION_TOKEN";

#[cfg(feature = "log")]
fn init_cli_environment() {
    env_logger::init();
    dotenvy::dotenv().ok();
}

#[cfg(not(feature = "log"))]
fn init_cli_environment() {}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    init_cli_environment();
    let token = env::var(TOKEN_ENV_VAR).expect("NOTION_TOKEN is not set");

    let markdown = NotionClient::new(token)
        .retrieve_block_children("2f853222ff1e80829678eeb55e7add95".into(), None, None)
        .await
        .unwrap()
        .to_markdown(0);

    fs::write(OUTPUT_PATH, markdown).unwrap();
    println!("Wrote markdown to {}", OUTPUT_PATH);
}
