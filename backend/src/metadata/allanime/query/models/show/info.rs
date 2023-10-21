use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::metadata::{
    allanime::query::models::common::{DateTimeField, FieldOrEmptyObject, PageStatus},
    AnimeStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowInfo {
    pub show: Show,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowBase {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub english_name: String,
    pub native_name: String,
    pub slug_time: Value,
    pub thumbnail: String,
    pub last_episode_info: LastEpisodeInfo,
    pub last_episode_date: LastEpisodeDate,
    #[serde(rename = "type")]
    pub type_field: String,
    pub season: Season,
    pub score: Option<f64>,
    pub aired_start: DateTimeField,
    pub available_episodes: AvailableEpisodes,
    pub episode_duration: String,
    pub episode_count: String,
    pub last_update_end: String,
    pub description: String,
    pub alt_names: Vec<String>,
    pub average_score: Option<i64>,
    pub rating: Option<String>,
    pub broadcast_interval: String,
    pub banner: Option<String>,
    pub characters: Vec<Character>,
    pub available_episodes_detail: AvailableEpisodesDetail,
    pub name_only_string: String,
    pub related_shows: Vec<RelatedShow>,
    pub related_mangas: Vec<RelatedManga>,
    pub is_adult: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    #[serde(flatten)]
    pub base: ShowBase,
    pub status: AnimeStatus,
    pub thumbnails: Vec<String>,
    pub genres: Vec<String>,
    pub aired_end: FieldOrEmptyObject<DateTimeField>,
    pub studios: Vec<String>,
    pub country_of_origin: String,
    pub prevideos: Vec<String>,
    #[serde(alias = "musics")]
    pub music: Vec<Music>,
    pub tags: Vec<String>,
    pub page_status: PageStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEpisodeInfo {
    pub sub: Option<LastEpisodeInfoEntry>,
    pub dub: Option<LastEpisodeInfoEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEpisodeInfoEntry {
    pub episode_string: String,
    #[serde(rename = "notes")]
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEpisodeDate {
    pub sub: FieldOrEmptyObject<DateTimeField>,
    pub dub: FieldOrEmptyObject<DateTimeField>,
    pub raw: FieldOrEmptyObject<DateTimeField>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub quarter: String,
    pub year: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableEpisodes {
    pub sub: i64,
    pub dub: i64,
    pub raw: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub role: String,
    pub name: Name,
    pub image: Image,
    pub ani_list_id: i64,
    pub voice_actors: Vec<VoiceActor>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub full: String,
    pub native: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub large: String,
    pub medium: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceActor {
    pub language: String,
    pub ani_list_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableEpisodesDetail {
    pub sub: Vec<String>,
    pub dub: Vec<String>,
    pub raw: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedShow {
    pub relation: String,
    pub show_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedManga {
    pub relation: String,
    pub manga_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Music {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub format: String,
    pub music_id: Option<String>,
    pub url: Option<String>,
}
