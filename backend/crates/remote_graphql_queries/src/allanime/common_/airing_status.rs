use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AiringStatus {
    Unknown,
    Unaired,
    Airing,
    Completed,
}
impl Default for AiringStatus {
    fn default() -> Self {
        Self::Unknown
    }
}
impl FromStr for AiringStatus {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(&format!("\"{}\"", s))
    }
}
