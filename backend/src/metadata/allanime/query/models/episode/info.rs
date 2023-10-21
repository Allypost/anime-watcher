use anyhow::bail;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::metadata::allanime::query::models::{
    common::{DateTimeField, FieldOrEmptyObject, PageStatus, VideoInfo},
    show::ShowBase,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeInfo {
    pub episode: Episode,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub episode_info: EpisodeInfoBase,
    pub episode_string: String,
    pub upload_date: FieldOrEmptyObject<DateTimeField>,
    pub source_urls: Vec<SourceUrl>,
    pub show: ShowBase,
    pub page_status: PageStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeInfoBase {
    #[serde(rename = "notes")]
    pub title: String,
    pub thumbnails: Vec<String>,
    #[serde(rename = "vidInforssub")]
    pub video_info_sub: Option<VideoInfo>,
    #[serde(rename = "vidInforsdub")]
    pub video_info_dub: Option<VideoInfo>,
    #[serde(rename = "vidInforsraw")]
    pub video_info_raw: Option<VideoInfo>,
    pub upload_dates: Option<UploadDates>,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadDates {
    pub sub: Option<DateTime<Utc>>,
    pub dub: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceUrl {
    pub source_url: String,
    pub priority: f64,
    pub source_name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub class_name: String,
    pub streamer_id: String,
    pub downloads: Option<Downloads>,
    pub sandbox: Option<String>,
}

lazy_static::lazy_static! {
    static ref BASE_EMBED_URL: Url = Url::parse("https://embed.ssbcontent.site").unwrap();
}

impl SourceUrl {
    pub fn try_decode_source(&self) -> anyhow::Result<String> {
        let chars = self.source_url.chars().collect::<Vec<char>>();
        let first_chars = (chars[0], chars[1]);
        let (key, skip) = match first_chars {
            ('-', '-') => ("s5feqxw21", 2),
            ('#', '-') => ("1234567890123456789012345", 2),
            ('#', '#') => ("1234567890123456789", 2),
            ('-', '#') => ("feqx1", 2),
            ('#', _) => ("allanimenews", 1),
            _ => {
                bail!("Cannot decode source_url: {:?}", self.source_url);
            }
        };

        let key = key.chars().map(u32::from).collect::<Vec<_>>();

        let decoded = chars[skip..]
            .chunks_exact(2)
            .filter_map(|x| {
                let num = format!("{}{}", x[0], x[1]);
                let num = u32::from_str_radix(&num, 16).ok()?;
                let num = key.iter().fold(num, |acc, x| acc ^ *x);

                char::from_u32(num)
            })
            .collect::<String>();

        if decoded.starts_with('/') {
            let decoded = decoded.replace("?id=", ".json?id=");
            return Ok((*BASE_EMBED_URL).join(&decoded)?.to_string());
        }

        Ok(decoded)
    }

    pub fn decoded(&mut self) -> &Self {
        if let Ok(decoded) = self.try_decode_source() {
            self.source_url = decoded;
        }

        self
    }

    pub fn decode(&mut self) {
        self.decoded();
    }

    #[allow(dead_code)]
    pub fn is_embed(&self) -> bool {
        self.source_url.starts_with(BASE_EMBED_URL.as_str())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    pub source_name: String,
    pub download_url: String,
}
