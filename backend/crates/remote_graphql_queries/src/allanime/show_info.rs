use chrono::{prelude::*, Duration};
use serde::Serialize;

use super::{
    common_::{
        AiringStatus, BigInt, DateTimeField, Object, SubDubRaw, TranslationType,
        VaildCountryOriginEnumType,
    },
    schema,
};

#[derive(cynic::QueryVariables, Debug)]
pub struct ShowInfoVariables {
    pub show_id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ShowInfoVariables")]
pub struct ShowInfo {
    #[arguments(_id: $show_id)]
    pub show: Option<Show>,
    #[arguments(showId: $show_id, episodeNumStart: 0.0, episodeNumEnd: 999_999.0)]
    #[cynic(flatten)]
    pub episode_infos: Vec<EpisodeInfo>,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
pub struct Show {
    #[cynic(rename = "_id")]
    pub id: Option<String>,
    pub mal_id: Option<BigInt>,
    pub english_name: Option<String>,
    pub description: Option<String>,
    #[cynic(rename = "status")]
    pub airing_status: Option<String>,
    pub genres: Option<Vec<Option<String>>>,
    pub broadcast_interval: Option<BigInt>,
    pub last_episode_timestamp: Option<Object<SubDubRaw<i64>>>,
    pub determined_interval: Option<Object<SubDubRaw<i64>>>,
    pub aired_start: Option<Object<DateTimeField>>,
    pub native_name: Option<String>,
    pub country_of_origin: Option<VaildCountryOriginEnumType>,
    pub available_episodes: Option<Object<SubDubRaw<u32>>>,
    pub episode_count: Option<BigInt>,
    #[cynic(rename = "name")]
    pub romanji_name: Option<String>,
}

impl Show {
    fn relevant_broadcast_interval(&self, for_type: &TranslationType) -> Option<Duration> {
        if for_type != &TranslationType::Dub && self.broadcast_interval.is_some() {
            return self
                .broadcast_interval
                .as_ref()
                .and_then(|x| x.0.parse::<i64>().ok())
                .map(Duration::milliseconds);
        }

        if self.determined_interval.is_some() {
            return self
                .determined_interval
                .as_ref()
                .map(|x| x.0.clone())
                .and_then(|x| match for_type {
                    TranslationType::Sub => x.sub,
                    TranslationType::Dub => x.dub,
                    TranslationType::Raw => x.raw,
                })
                .map(Duration::milliseconds);
        }

        None
    }

    #[must_use]
    pub fn airing_status_for_sub(&self) -> AiringStatus {
        self.airing_status(&TranslationType::Sub)
    }

    #[must_use]
    pub fn airing_status(&self, for_type: &TranslationType) -> AiringStatus {
        self.available_episodes
            .as_ref()
            .map(|x| x.0.clone())
            .and_then(|x| match for_type {
                TranslationType::Sub => x.sub,
                TranslationType::Dub => x.dub,
                TranslationType::Raw => x.raw,
            })
            .map_or(AiringStatus::Unknown, |available_episodes| {
                if available_episodes == 0 {
                    return AiringStatus::Unaired;
                }

                let episode_count = self.episode_count.clone().map(i64::from);
                let episode_count = match episode_count {
                    Some(episode_count) => episode_count,
                    None => return AiringStatus::Unknown,
                };

                let available_episodes = i64::from(available_episodes);

                if available_episodes >= episode_count {
                    AiringStatus::Completed
                } else {
                    AiringStatus::Airing
                }
            })
    }

    #[must_use]
    pub fn estimate_release_time(&self) -> Option<DateTime<Utc>> {
        let ep_type = TranslationType::Sub;
        let tz_offset = Duration::hours(4);
        let airing_status = self.airing_status(&ep_type);

        if airing_status == AiringStatus::Completed {
            return None;
        }

        if self.aired_start.is_some() && airing_status == AiringStatus::Unaired {
            let aired_start = self
                .aired_start
                .as_ref()
                .map(|x| x.0.clone())
                .unwrap_or_default();
            let aired_start: DateTime<Utc> = aired_start.try_into().ok()?;
            let aired_start = aired_start + tz_offset + Duration::hours(1);

            return Some(aired_start);
        } else if self.last_episode_timestamp.is_some() && airing_status == AiringStatus::Airing {
            let relevant_broadcast_interval = self.relevant_broadcast_interval(&ep_type);

            if let Some(relevant_broadcast_interval) = relevant_broadcast_interval {
                return self
                    .last_episode_timestamp
                    .as_ref()
                    .map(|x| x.0.clone())
                    .and_then(|x| match ep_type {
                        TranslationType::Sub => x.sub,
                        TranslationType::Dub => x.dub,
                        TranslationType::Raw => x.raw,
                    })
                    .and_then(|x| NaiveDateTime::from_timestamp_opt(x, 0))
                    .map(|x| Utc.from_utc_datetime(&x) + tz_offset + relevant_broadcast_interval);
            }
        }

        None
    }
}

#[derive(cynic::QueryFragment, Debug)]
pub struct EpisodeInfo {
    #[cynic(rename = "_id")]
    pub id: Option<String>,
    #[cynic(rename = "notes")]
    pub title: Option<String>,
    pub description: Option<String>,
    pub episode_id_num: Option<f64>,
    #[cynic(rename = "vidInforssub")]
    pub sub_info: Option<Object>,
    #[cynic(rename = "vidInforsdub")]
    pub dub_info: Option<Object>,
}
