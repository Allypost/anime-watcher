use anyhow::{anyhow, Result};
use cynic::{Operation, QueryFragment, QueryVariables};
use log::trace;
use serde::{de::DeserializeOwned, Serialize};

#[cynic::schema("allanime")]
pub mod schema {}

pub mod common_;
pub mod show_info;

pub const BASE_API_URL: &str = "https://api.allanime.day/api";

pub async fn do_query<TQuery, TQueryVars>(op: Operation<TQuery, TQueryVars>) -> Result<TQuery>
where
    TQuery: QueryFragment + std::fmt::Debug + DeserializeOwned + 'static,
    TQueryVars: QueryVariables + std::fmt::Debug + Serialize + 'static,
{
    trace!(
        "Sending query: {name:?} with vars {vars:?}",
        name = &op.operation_name,
        vars = &op.variables,
    );

    let resp = reqwest::Client::new()
        .post(BASE_API_URL)
        .json(&op)
        .send()
        .await?
        .error_for_status()?;

    let resp_body = resp.text().await?;

    let result = serde_json::from_str::<cynic::GraphQlResponse<TQuery>>(&resp_body)?;

    if let Some(errors) = result.errors {
        return Err(anyhow!(errors
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")));
    }

    result
        .data
        .ok_or_else(|| anyhow!("No data returned for query"))
}
