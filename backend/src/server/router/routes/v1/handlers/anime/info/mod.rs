use axum::{extract::Query, Extension};
use axum_extra::extract::WithRejection;
use axum_macros::debug_handler;
use log::debug;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    metadata::{self, MetaEpisodeInfo, MetaSeriesInfo},
    server::{router::routes::v1::response::V1Response, server_timing::ServerTimings},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeInfoFloatingResponse {
    pub meta: MetaSeriesInfo,
    pub info: metadata::AnimeInfo,
}
#[debug_handler]
pub async fn anime_info_floating(
    WithRejection(Query(meta), _): WithRejection<Query<MetaSeriesInfo>, V1Response>,
) -> V1Response<AnimeInfoFloatingResponse> {
    let info = match metadata::series_info(meta.clone()).await {
        Ok(info) => info,
        Err(e) => {
            debug!("Error getting series info: {:?}", e);

            return V1Response::Error(StatusCode::BAD_REQUEST, e.into());
        }
    };

    V1Response::Success(AnimeInfoFloatingResponse { meta, info })
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeInfoFloatingResponse {
    pub meta: MetaEpisodeInfo,
    pub info: serde_json::Value,
}
#[debug_handler]
pub async fn episode_info_floating(
    WithRejection(Query(meta), _): WithRejection<Query<MetaEpisodeInfo>, V1Response>,
    Extension(server_timings): Extension<ServerTimings>,
) -> V1Response<EpisodeInfoFloatingResponse> {
    server_timings.add_started("episode_info", None);
    let info = metadata::episode_info(meta.clone()).await;
    server_timings.end("episode_info");
    let info = match info {
        Ok(info) => info,
        Err(e) => {
            debug!("Error getting episode info: {:?}", e);

            return V1Response::Error(StatusCode::BAD_REQUEST, e.into());
        }
    };

    V1Response::Success(EpisodeInfoFloatingResponse { meta, info })
}
