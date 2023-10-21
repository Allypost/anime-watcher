use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageStatus {
    #[serde(rename = "_id")]
    pub id: String,
    pub notes: Value,
    pub page_id: String,
    pub show_id: String,
    pub views: String,
    pub likes_count: String,
    pub comment_count: String,
    pub dislikes_count: String,
    pub review_count: String,
    pub user_score_count: String,
    pub user_score_total_value: f64,
    pub user_score_aver_value: f64,
}
