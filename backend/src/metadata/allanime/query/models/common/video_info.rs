use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    pub vid_resolution: u32,
    pub vid_path: String,
    pub vid_size: u64,
    pub vid_duration: f64,
}
