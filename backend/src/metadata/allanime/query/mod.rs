use std::collections::HashMap;

use scraper::Html;
use serde_json::json;

use crate::metadata::{
    self, allanime::query::client::BASE_SITE_URL, common::prelude::*, SeriesTranslation,
};

use self::models::{
    episode::info::{Episode, EpisodeInfo},
    show::{ShowEpisode, ShowEpisodes},
};

mod client;
pub mod models;

mod show_info_2 {
    use serde::{Deserialize, Serialize};

    use crate::metadata::AnimeStatus;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub show: Show,
        #[serde(rename = "episodeInfos")]
        pub episodes: Vec<Episode>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Show {
        pub mal_id: String,
        pub english_name: String,
        pub description: Option<String>,
        pub status: AnimeStatus,
        pub genres: Vec<String>,
        pub broadcast_interval: String,
        pub last_episode_timestamp: SubDubRawTimestamps<i64>,
        pub determined_interval: SubDubRawTimestamps<i64>,
        pub country_of_origin: String,
        pub native_name: String,
        #[serde(rename = "name")]
        pub romanji_name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Episode {
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(rename = "notes")]
        pub title: Option<String>,
        pub description: Option<String>,
        pub episode_id_num: f64,
        pub upload_dates: Option<SubDubRawTimestamps<String>>,
    }

    #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
    pub struct SubDubRawTimestamps<T> {
        pub sub: Option<T>,
        pub dub: Option<T>,
        pub raw: Option<T>,
    }
}

#[allow(clippy::too_many_lines)]
pub async fn show_info(id: &str) -> Result<metadata::AnimeInfo> {
    use remote_graphql_queries::prelude::*;
    trace!("Getting show info for {}", id);

    let resp: allanime::show_info::ShowInfo = allanime::do_query(
        allanime::show_info::ShowInfo::build(allanime::show_info::ShowInfoVariables {
            show_id: id.to_string(),
        }),
    )
    .await?;

    let show = resp.show.ok_or_else(|| {
        anyhow::anyhow!("Failed to get show info for show={:?} from allanime", id)
    })?;
    let episode_infos = resp.episode_infos;

    let next_release_estimate = show.estimate_release_time();
    let show_status = show.airing_status_for_sub().into();
    let show_id = show.id.clone().unwrap_or_default();
    let show_title_slug = show
        .romanji_name
        .clone()
        .unwrap_or_default()
        .to_lowercase()
        .chars()
        .filter(|x| x.is_alphanumeric() || *x == ' ')
        .map(|x| x.to_string())
        .collect::<String>()
        .replace(' ', "-");

    let episodes = episode_infos
        .into_iter()
        .filter_map(|x| {
            let mut ret = vec![];

            let episode_number = x.episode_id_num?;
            let title = x.title.clone().unwrap_or_default();

            if x.sub_info.is_some() {
                ret.push(metadata::EpisodeInfo {
                    id: show_id.clone(),
                    episode_number,
                    title: title.clone(),
                    translation: SeriesTranslation::Sub,
                    url: format!(
                        "{base}/bangumi/{id}/{slug}/p-{num}-{t}",
                        base = BASE_SITE_URL,
                        id = id,
                        slug = show_title_slug.clone(),
                        num = episode_number,
                        t = "sub",
                    ),
                });
            }

            if x.dub_info.is_some() {
                ret.push(metadata::EpisodeInfo {
                    id: show_id.clone(),
                    episode_number,
                    title: title.clone(),
                    translation: SeriesTranslation::Dub,
                    url: format!(
                        "{base}/bangumi/{id}/{slug}/p-{num}-{t}",
                        base = BASE_SITE_URL,
                        id = id,
                        slug = show_title_slug.clone(),
                        num = episode_number,
                        t = "dub"
                    ),
                });
            }

            Some(ret)
        })
        .flatten()
        .collect::<Vec<_>>();

    let description = show.description.map(|x| {
        let doc = Html::parse_document(&x);

        doc.root_element()
            .text()
            .collect::<String>()
            .trim()
            .to_string()
    });

    let mut meta: HashMap<String, serde_json::Value> = HashMap::new();
    meta.insert("countryOfOrigin".into(), json!(show.country_of_origin));

    Ok(metadata::AnimeInfo {
        id: id.to_string(),
        name: show.english_name.unwrap_or_default(),
        url: format!(
            "{base}/bangumi/{id}/{slug}",
            base = BASE_SITE_URL,
            id = id,
            slug = show_title_slug.clone(),
        ),
        alt_names: vec![
            metadata::AltName {
                name: show.romanji_name.unwrap_or_default(),
                language: "ja-EN".to_string(),
            },
            metadata::AltName {
                name: show.native_name.unwrap_or_default(),
                language: "ja-JA".to_string(),
            },
        ],
        description,
        status: show_status,
        episodes,
        genres: show
            .genres
            .map(|x| x.into_iter().flatten().collect())
            .unwrap_or_default(),
        mal_id: show.mal_id.and_then(|x| x.0.parse().ok()),
        next_release_estimate,
        meta,
    })
}

#[allow(dead_code)]
pub async fn show_episodes(id: &str) -> Result<Vec<ShowEpisode>> {
    trace!("Getting show episodes for {}", id);

    let resp: ShowEpisodes = client::do_query(
        "c8f3ac51f598e630a1d09d7f7fb6924cff23277f354a23e473b962a367880f7d",
        json!({
            "showId": id,
            "episodeNumStart": 1,
            "episodeNumEnd": 9999,
        }),
    )
    .await
    .context(format!("Failed to get episodes for show={:?}", id))?;

    Ok(resp.episode_infos)
}

pub async fn episode_info(
    series_id: &str,
    episode_number: f64,
    episode_type: SeriesTranslation,
) -> Result<Episode> {
    trace!(
        "Getting episode info for series={} episode={}",
        series_id,
        episode_number
    );

    let resp: EpisodeInfo = client::do_query(
        "5f1a64b73793cc2234a389cf3a8f93ad82de7043017dd551f38f65b89daa65e0",
        json!({
            "showId": series_id,
            "translationType": match episode_type {
                SeriesTranslation::Dub => "dub",
                SeriesTranslation::Sub => "sub",
                SeriesTranslation::Unknown => "raw",
            },
            "episodeString": episode_number.to_string(),
        }),
    )
    .await
    .context(format!(
        "Failed to get info for show={show:?} episode={episode:?} type={episode_type:?}",
        show = series_id,
        episode = episode_number,
        episode_type = episode_type,
    ))?;

    Ok(resp.episode)
}
