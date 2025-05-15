use async_trait::async_trait;
pub use block::Block;
pub use page::Page;
use regex::Regex;

mod block;
mod page;
mod rich_text;

pub trait NotionClient {
    fn fetch_blocks(&self, page_id: &str) -> Vec<String>;
}

pub(crate) fn escape_page_title(title: &String) -> String {
    let re = Regex::new(r"[\s\u{200B}]").unwrap();
    re.replace_all(title, "_").to_string()
}

#[async_trait]
pub trait NotionApi {
    type Error;

    async fn retrieve_block_children<T>(
        &self,
        block_id: &str,
        start_cursor: Option<String>,
        page_size: Option<u32>,
    ) -> Result<T, Self::Error>;

    async fn retrieve_page<T>(&self, page_id: &str) -> Result<T, Self::Error>;

    async fn retrieve_database<T>(&self, database_id: &str) -> Result<T, Self::Error>;
}
