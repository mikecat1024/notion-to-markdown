use notion_to_markdown_core::Block;
use reqwest::Client;
use std::{error, fmt, time::Duration};
use tokio::time::sleep;

#[derive(Debug)]
pub enum NotionClientError {
    Http(reqwest::Error),
    Status(reqwest::StatusCode),
}

impl fmt::Display for NotionClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotionClientError::Http(e) => write!(f, "HTTP error: {}", e),
            NotionClientError::Status(code) => write!(f, "Unexpected status code: {}", code),
        }
    }
}

impl error::Error for NotionClientError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            NotionClientError::Http(e) => Some(e),
            NotionClientError::Status(_) => None,
        }
    }
}

impl From<reqwest::Error> for NotionClientError {
    fn from(err: reqwest::Error) -> Self {
        NotionClientError::Http(err)
    }
}

pub struct NotionClient {
    client: Client,
    token: String,
    version: String,
}

#[derive(serde::Deserialize)]
pub struct BlockChildrenResponse {
    pub results: Vec<Block>,
    pub next_cursor: Option<String>,
}

impl NotionClient {
    pub fn new(token: String) -> Self {
        NotionClient {
            client: Client::new(),
            token,
            version: "2022-06-28".into(),
        }
    }

    async fn _retrieve_block_children(
        &self,
        block_id: &str,
        start_cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<BlockChildrenResponse, NotionClientError> {
        let mut req = self
            .client
            .get(&format!(
                "https://api.notion.com/v1/blocks/{}/children",
                block_id
            ))
            .bearer_auth(&self.token)
            .header("Notion-Version", &self.version);

        if let Some(cursor) = start_cursor {
            req = req.query(&[("start_cursor", cursor)]);
        }
        if let Some(size) = page_size {
            req = req.query(&[("page_size", size.to_string())]);
        }

        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(NotionClientError::Status(status));
        }
        let body = resp.json::<BlockChildrenResponse>().await?;
        Ok(body)
    }

    pub async fn retrieve_block_children(
        &self,
        block_id: &str,
        initial_cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<Vec<Block>, NotionClientError> {
        let mut all_results = Vec::new();
        let mut cursor = initial_cursor.map(|s| s.to_string());

        loop {
            let resp = loop {
                match self
                    ._retrieve_block_children(block_id, cursor.as_deref(), page_size)
                    .await
                {
                    Ok(res) => break res,
                    Err(NotionClientError::Status(code))
                        if code == reqwest::StatusCode::TOO_MANY_REQUESTS =>
                    {
                        sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            };

            all_results.extend(resp.results);

            if let Some(next) = resp.next_cursor {
                cursor = Some(next);
            } else {
                break;
            }
        }

        Ok(all_results)
    }
}
