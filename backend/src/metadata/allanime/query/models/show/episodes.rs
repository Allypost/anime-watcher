use serde::{Deserialize, Serialize};

use crate::metadata::allanime::query::models::common::VideoInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShowEpisodes {
    pub episode_infos: Vec<ShowEpisode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShowEpisode {
    pub episode_id_num: f64,
    #[serde(rename = "notes")]
    pub title: Option<String>,
    pub thumbnails: Option<Vec<String>>,
    pub upload_dates: Option<UploadDates>,
    #[serde(rename = "vidInforssub")]
    pub video_info_sub: Option<VideoInfo>,
    #[serde(rename = "vidInforsdub")]
    pub video_info_dub: Option<VideoInfo>,
    #[serde(rename = "vidInforsraw")]
    pub video_info_raw: Option<VideoInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UploadDates {
    pub sub: Option<String>,
    pub dub: Option<String>,
}
