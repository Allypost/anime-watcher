use super::{
    common::{prelude::*, util},
    AnimeInfo,
};

mod anime;
mod episode;
mod request;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniwatchSeries {
    #[serde(rename = "seriesId")]
    pub id: String,
    #[serde(default, with = "util::bool_str")]
    pub estimate_release_time: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniwatchEpisode {
    #[serde(flatten)]
    pub series: AniwatchSeries,
    pub episode_id: String,
}

pub async fn series_info(info: AniwatchSeries) -> Result<AnimeInfo> {
    anime::get_info(&info.id, info.estimate_release_time).await
}

pub async fn episode_info(info: AniwatchEpisode) -> Result<serde_json::Value> {
    episode::get_info(&info.series.id, &info.episode_id).await
}
