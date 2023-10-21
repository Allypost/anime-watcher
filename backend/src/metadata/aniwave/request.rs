use anyhow::{anyhow, Result};
use log::trace;
use reqwest::{Client, ClientBuilder, Response};

pub const BASE_URL: &str = "https://aniwave.to";
pub const USER_AGENT: &str = "AniWave Video Downloader";

pub fn create_client() -> ClientBuilder {
    Client::builder()
        .deflate(true)
        .gzip(true)
        .user_agent(USER_AGENT)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    Html,
    Api,
}

pub async fn get_page(url: &str, as_request: RequestType) -> Result<Response> {
    let url = if url.starts_with('/') {
        format!("{BASE_URL}{url}")
    } else {
        url.to_string()
    };

    trace!("Getting page {}", &url);

    let request = create_client()
        .build()?
        .get(url)
        .header("Accept", "*/*")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36")
    ;

    let request = match as_request {
        RequestType::Html => request.header("Accept", "text/html"),
        RequestType::Api => request
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Accept", "application/json"),
    };

    request.send().await.map_err(|e| anyhow!(e))
}
