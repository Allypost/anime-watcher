#![allow(dead_code)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;

use crate::metadata::{myanimelist::request, AnimeStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Picture {
    large: Option<String>,
    medium: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AlternativeTitles {
    synonyms: Option<Vec<String>>,
    en: Option<String>,
    ja: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Genre {
    id: u32,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Broadcast {
    /// Day of the week broadcast in Japan time.
    /// Day of the week or `other`
    day_of_the_week: String,
    /// for example: "01:25"
    start_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "camelCase"))]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct StartSeason {
    year: u32,
    season: Season,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Studio {
    id: u32,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "kebab-case"))]
pub enum Nsfw {
    #[serde(alias = "white")]
    /// This work is safe for work
    Safe,

    #[serde(alias = "gray")]
    /// This work may be not safe for work
    MaybeUnsafe,

    #[serde(alias = "black")]
    /// This work is not safe for work
    Unsafe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "camelCase"))]
pub enum MediaType {
    Unknown,
    Tv,
    Ova,
    Movie,
    Special,
    Ona,
    Music,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub enum Source {
    Other,
    Original,
    Manga,
    #[serde(rename = "4_koma_manga")]
    FourKomaManga,
    WebManga,
    DigitalManga,
    Novel,
    LightNovel,
    VisualNovel,
    Game,
    CardGame,
    Book,
    PictureBook,
    Radio,
    Music,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "PascalCase"))]
pub enum Rating {
    /// All Ages
    G,
    /// Children
    Pg,
    /// Teens 13 and Older
    #[serde(alias = "pg_13")]
    Pg13,
    /// 17+ (violence & profanity)
    R,
    /// Profanity & Mild Nudity
    #[serde(alias = "r+")]
    RPlus,
    /// Hentai
    Rx,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldNamesAsArray)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AnimeDetails {
    id: u32,
    title: String,
    main_picture: Option<Picture>,
    /// "synonyms" or ISO 639-1
    alternative_titles: Option<AlternativeTitles>,
    start_date: Option<String>,
    end_date: Option<String>,
    /// Synopsis.
    /// The API strips BBCode tags from the result.
    synopsis: Option<String>,
    /// Mean score.
    /// When the `mean` can not be calculated, such as when the number of user scores is small, the result does not include this field.
    mean: Option<f64>,
    /// When the `rank` can not be calculated, such as when the number of user scores is small, the result does not include this field.
    rank: Option<u32>,
    popularity: Option<u32>,
    /// Number of users who have this work in their list.
    num_list_users: u32,
    num_scoring_users: u32,
    nsfw: Option<Nsfw>,
    genres: Option<Vec<Genre>>,
    created_at: Option<String>,
    updated_at: Option<String>,
    media_type: MediaType,
    /// Airing status.
    status: AnimeStatus,
    /// The total number of episodes of this series. If unknown, it is 0.
    num_episodes: u32,
    start_season: Option<StartSeason>,
    /// Broadcast date.
    broadcast: Option<Broadcast>,
    /// Original work.
    source: Option<Source>,
    /// Average length of episode in seconds.
    average_episode_duration: Option<u32>,
    rating: Option<Rating>,
    studios: Option<Vec<Studio>>,
    pictures: Option<Vec<Picture>>,
    background: Option<String>,
}

const DETAIL_FIELDS: &[&str] = AnimeDetails::FIELD_NAMES_AS_ARRAY;

pub async fn get_details(anime_id: u32) -> Result<AnimeDetails> {
    let url = format!(
        "/anime/{anime_id}?fields={fields}",
        anime_id = anime_id,
        fields = DETAIL_FIELDS.join(",")
    );
    let resp = request::get_page(&url)
        .await?
        .error_for_status()?
        .text()
        .await?;
    let resp: AnimeDetails = serde_json::from_str(&resp)?;

    Ok(resp)
}
