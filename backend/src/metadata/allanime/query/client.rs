use std::{collections::HashMap, time::Duration};

use crate::metadata::common::prelude::*;
use axum::http::{HeaderMap, HeaderValue};
use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use super::models::common::QueryResponse;

pub const BASE_SITE_URL: &str = "https://allanime.to";
pub const BASE_API_URL: &str = "https://api.allanime.day/api";
pub const USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/118.0";

lazy_static! {
    static ref DEFAULT_HEADERS: HeaderMap = HeaderMap::from_iter([
        (
            header::ACCEPT,
            HeaderValue::from_static("application/json, text/plain, */*"),
        ),
        (
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_static("en-US,en;q=0.5"),
        ),
        (
            "X-Requested-With".parse().unwrap(),
            HeaderValue::from_static("XMLHttpRequest"),
        ),
        (header::ORIGIN, HeaderValue::from_static(BASE_SITE_URL)),
        (header::REFERER, HeaderValue::from_static(BASE_SITE_URL)),
    ]);
}

pub fn query_client() -> Client {
    Client::builder()
        .connect_timeout(Duration::from_secs(3))
        .user_agent(USER_AGENT)
        .default_headers(DEFAULT_HEADERS.clone())
        .build()
        .unwrap()
}

lazy_static! {
    pub static ref QUERY_CLIENT: Client = query_client();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedQuery {
    pub version: i64,
    pub sha256_hash: String,
}

impl PersistedQuery {
    pub fn v1(hash: &str) -> Self {
        Self {
            version: 1,
            sha256_hash: hash.to_owned(),
        }
    }
}

impl From<String> for PersistedQuery {
    fn from(hash: String) -> Self {
        Self::v1(&hash)
    }
}

impl From<&str> for PersistedQuery {
    fn from(hash: &str) -> Self {
        Self::v1(hash)
    }
}

pub async fn do_query<TResp, TQuery>(query: TQuery, variables: Value) -> Result<TResp>
where
    TResp: DeserializeOwned + 'static,
    TQuery: Into<PersistedQuery>,
{
    let query = query.into();

    trace!(
        "Doing query for: query={query:?} variables={vars:?}",
        query = query,
        vars = variables,
    );

    let query: HashMap<&str, Value> = HashMap::from_iter([
        ("variables", variables),
        (
            "extensions",
            json!({
                "persistedQuery": query,
            }),
        ),
    ]);

    let encoded_query = query
        .iter()
        .map(|(k, v)| {
            let v = serde_json::to_string(&v).expect("Failed to encode query");

            format!("{}={}", k, urlencoding::encode(&v))
        })
        .collect::<Vec<_>>()
        .join("&");

    let url = format!("{}?{}", BASE_API_URL, encoded_query);

    trace!("Fetching URL: {:?}", url);

    let resp = QUERY_CLIENT.get(&url).send().await?;

    trace!("Response: {:?}", resp);

    let resp_body = resp.text().await?;

    match serde_json::from_str::<QueryResponse<TResp>>(&resp_body).map(|x| x.data) {
        Ok(data) => Ok(data),
        Err(err) => {
            let err_msg = err.to_string();
            Err(anyhow!(err).context(json!({
                "err": err_msg,
                "url": url,
                "response": resp_body,
            })))
        }
    }
}
