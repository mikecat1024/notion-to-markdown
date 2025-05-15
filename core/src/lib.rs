pub use block::Block;
pub use page::Page;

mod block;
mod page;
mod rich_text;

pub trait NotionClient {
    fn fetch_blocks(&self, page_id: &str) -> Vec<String>;
}

pub(crate) fn escape_page_title(title: &str) -> String {
    title
        .chars()
        .map(|c| {
            if c.is_whitespace() || c == '\u{200B}' {
                '_'
            } else {
                c
            }
        })
        .collect()
}

pub trait NotionApi {
    type Error;

    fn retrieve_block_children<T>(
        &self,
        block_id: &str,
        start_cursor: Option<String>,
        page_size: Option<u32>,
    ) -> impl std::future::Future<Output = Result<T, Self::Error>> + Send;

    fn retrieve_page<T>(
        &self,
        page_id: &str,
    ) -> impl std::future::Future<Output = Result<T, Self::Error>> + Send;

    fn retrieve_database<T>(
        &self,
        database_id: &str,
    ) -> impl std::future::Future<Output = Result<T, Self::Error>> + Send;
}
