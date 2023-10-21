use super::{common::prelude::*, AnimeInfo};

mod anime;
mod request;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniwaveSeries {
    #[serde(rename = "seriesId")]
    pub id: String,
}

pub async fn series_info(info: AniwaveSeries) -> Result<AnimeInfo> {
    anime::get_info(&info.id).await
}
