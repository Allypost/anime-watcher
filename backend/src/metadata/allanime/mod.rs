use serde_with::{serde_as, DisplayFromStr};

use self::query::models::episode::Episode;

use super::{common::prelude::*, AnimeInfo, SeriesTranslation};

mod anime;
pub mod query;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllanimeSeries {
    #[serde(rename = "seriesId")]
    pub id: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllanimeEpisode {
    #[serde(flatten)]
    pub series: AllanimeSeries,
    #[serde_as(as = "DisplayFromStr")]
    pub episode_number: f64,
    pub episode_type: SeriesTranslation,
}

pub async fn series_info(info: AllanimeSeries) -> Result<AnimeInfo> {
    anime::get_series_info(&info.id).await
}

pub async fn episode_info(info: AllanimeEpisode) -> Result<Episode> {
    anime::get_episode_info(&info.series.id, info.episode_number, info.episode_type).await
}
