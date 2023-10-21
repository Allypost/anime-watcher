use anyhow::{anyhow, Result};
use reqwest::{Client, ClientBuilder, Response};

pub const BASE_URL: &str = "https://aniwatch.to";
pub const USER_AGENT: &str = "Zoro.to Video Downloader";

pub fn create_client() -> ClientBuilder {
    Client::builder()
        .deflate(true)
        .gzip(true)
        .user_agent(USER_AGENT)
}

pub async fn get_page(url: &str) -> Result<Response> {
    let url = if url.starts_with('/') {
        format!("{BASE_URL}{url}")
    } else {
        url.to_string()
    };

    create_client()
        .build()?
        .get(url)
        .header("Accept", "*/*")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|e| anyhow!(e))
}
