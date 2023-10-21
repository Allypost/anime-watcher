use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DateTimeField {
    year: Option<i32>,
    month: Option<u32>,
    #[serde(rename = "date")]
    day: Option<u32>,
    hour: Option<u32>,
    minute: Option<u32>,
}

impl TryFrom<DateTimeField> for DateTime<Utc> {
    type Error = anyhow::Error;

    fn try_from(date: DateTimeField) -> std::result::Result<Self, Self::Error> {
        let year = date.year.unwrap_or_default();
        let month = date.month.unwrap_or_default();
        let day = date.day.unwrap_or_default();
        let hour = date.hour.unwrap_or_default();
        let minute = date.minute.unwrap_or_default();

        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|val| val.and_hms_opt(hour, minute, 0))
            .map(|x| Utc.from_utc_datetime(&x))
            .ok_or_else(|| {
                anyhow::anyhow!("Failed to convert date field to DateTime<Utc>: {:?}", date)
            })
    }
}
