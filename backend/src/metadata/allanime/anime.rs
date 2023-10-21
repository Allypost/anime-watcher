use crate::metadata::{allanime::query, common::prelude::*, AnimeInfo, SeriesTranslation};

use super::query::models::episode::Episode;

pub async fn get_series_info(series_id: &str) -> Result<AnimeInfo> {
    query::show_info(series_id).await
}

pub async fn get_episode_info(
    series_id: &str,
    episode_number: f64,
    episode_type: SeriesTranslation,
) -> Result<Episode> {
    let mut res = query::episode_info(series_id, episode_number, episode_type).await?;

    for source in &mut res.source_urls {
        source.decode();
    }

    Ok(res)
}
