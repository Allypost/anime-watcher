use anyhow::{anyhow, Result};
use reqwest::{Client, Response};

const CLIENT_ID: &str = "16c0cefeeb62cb0fb474388753256fa5";

pub const BASE_URL: &str = "https://api.myanimelist.net/v2";

pub async fn get_page(url: &str) -> Result<Response> {
    let url = if url.starts_with(BASE_URL) {
        url.to_string()
    } else {
        format!("{BASE_URL}{url}")
    };

    Client::builder()
        .deflate(true)
        .gzip(true)
        .build()?
        .get(url)
        .header("Accept", "*/*")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36")
        .header("X-Mal-Client-Id", CLIENT_ID)
        .send()
        .await
        .map_err(|e| anyhow!(e))
}
