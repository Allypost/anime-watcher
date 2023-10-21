use std::{collections::HashMap, str::FromStr};

use common::prelude::*;
use remote_graphql_queries::allanime::common_::AiringStatus;

pub mod allanime;
pub mod aniwatch;
pub mod aniwave;
mod common;
pub mod myanimelist;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SeriesTranslation {
    Dub,
    Sub,
    Unknown,
}

impl FromStr for SeriesTranslation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dub" => Ok(Self::Dub),
            "sub" => Ok(Self::Sub),
            _ => Ok(Self::Unknown),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AltName {
    pub name: String,
    #[serde(rename = "lang")]
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AnimeStatus {
    Unknown,
    #[serde(alias = "not_yet_aired")]
    Unaired,
    #[serde(alias = "Currently Airing")]
    #[serde(alias = "Releasing")]
    #[serde(alias = "currently_airing")]
    Airing,
    #[serde(alias = "finished_airing")]
    Completed,
}
impl Default for AnimeStatus {
    fn default() -> Self {
        Self::Unknown
    }
}
impl FromStr for AnimeStatus {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(&format!("\"{}\"", s))
    }
}
impl From<AiringStatus> for AnimeStatus {
    fn from(value: AiringStatus) -> Self {
        match value {
            AiringStatus::Unknown => Self::Unknown,
            AiringStatus::Unaired => Self::Unaired,
            AiringStatus::Airing => Self::Airing,
            AiringStatus::Completed => Self::Completed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AnimeInfo {
    pub id: String,
    pub mal_id: Option<u32>,
    pub name: String,
    pub alt_names: Vec<AltName>,
    pub description: Option<String>,
    pub url: String,
    pub episodes: Vec<EpisodeInfo>,
    pub status: AnimeStatus,
    pub genres: Vec<String>,
    pub next_release_estimate: Option<DateTime<Utc>>,
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeInfo {
    pub id: String,
    pub title: String,
    pub translation: SeriesTranslation,
    pub episode_number: f64,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnimeSite {
    Aniwatch,
    Aniwave,
    Allanime,
}
impl ToString for AnimeSite {
    fn to_string(&self) -> String {
        match self {
            Self::Aniwatch => "aniwatch".to_string(),
            Self::Aniwave => "aniwave".to_string(),
            Self::Allanime => "allanime".to_string(),
        }
    }
}
impl From<AnimeSite> for String {
    fn from(site: AnimeSite) -> Self {
        site.to_string()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "site")]
pub enum MetaSeriesInfo {
    Aniwatch(aniwatch::AniwatchSeries),
    Aniwave(aniwave::AniwaveSeries),
    Allanime(allanime::AllanimeSeries),
    #[serde(rename_all = "camelCase")]
    Test {
        test_id: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "site")]
pub enum MetaEpisodeInfo {
    Aniwatch(aniwatch::AniwatchEpisode),
    Allanime(allanime::AllanimeEpisode),
}

pub async fn series_info(info: MetaSeriesInfo) -> Result<AnimeInfo> {
    match info {
        MetaSeriesInfo::Aniwatch(info) => aniwatch::series_info(info).await,
        MetaSeriesInfo::Aniwave(info) => aniwave::series_info(info).await,
        MetaSeriesInfo::Allanime(info) => allanime::series_info(info).await,
        MetaSeriesInfo::Test { test_id } => Ok(AnimeInfo {
            id: test_id,
            name: "Test".to_string(),
            url: "https://example.com".to_string(),
            ..Default::default()
        }),
    }
}

pub async fn episode_info(info: MetaEpisodeInfo) -> Result<serde_json::Value> {
    match info {
        MetaEpisodeInfo::Allanime(info) => allanime::episode_info(info)
            .await
            .map(|x| serde_json::json!(x)),
        MetaEpisodeInfo::Aniwatch(info) => aniwatch::episode_info(info).await,
    }
}
