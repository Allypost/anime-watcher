use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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
        let month = date.month.unwrap_or_default().clamp(1, 12);
        let day = date.day.unwrap_or_default().clamp(1, 31);
        let hour = date.hour.unwrap_or_default().clamp(0, 23);
        let minute = date.minute.unwrap_or_default().clamp(0, 59);

        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|val| val.and_hms_opt(hour, minute, 0))
            .map(|x| Utc.from_utc_datetime(&x))
            .ok_or_else(|| {
                anyhow::anyhow!("Failed to convert date field to DateTime<Utc>: {:?}", date)
            })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldOrEmptyObject<T> {
    Field(T),
    Null(Value),
}

impl<T> Default for FieldOrEmptyObject<T> {
    fn default() -> Self {
        Self::Null(Value::Object(Map::default()))
    }
}

impl<T> From<FieldOrEmptyObject<T>> for Option<T> {
    fn from(field: FieldOrEmptyObject<T>) -> Self {
        match field {
            FieldOrEmptyObject::Field(field) => Some(field),
            FieldOrEmptyObject::Null(_) => None,
        }
    }
}

impl<T> From<Option<T>> for FieldOrEmptyObject<T> {
    fn from(field: Option<T>) -> Self {
        match field {
            Some(field) => Self::Field(field),
            None => Self::Null(Value::Null),
        }
    }
}

impl<T: TryInto<DateTime<Utc>>> TryFrom<FieldOrEmptyObject<T>> for DateTime<Utc> {
    type Error = anyhow::Error;

    fn try_from(field: FieldOrEmptyObject<T>) -> std::result::Result<Self, Self::Error> {
        match field {
            FieldOrEmptyObject::Field(value) => match value.try_into() {
                Ok(field) => Ok(field),
                Err(_e) => Err(anyhow::anyhow!("Failed to convert field to date")),
            },
            FieldOrEmptyObject::Null(_) => Err(anyhow::anyhow!("Field is null")),
        }
    }
}

impl<T> FieldOrEmptyObject<T> {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null(_))
    }

    pub fn as_option(&self) -> Option<&T> {
        match self {
            Self::Field(field) => Some(field),
            Self::Null(_) => None,
        }
    }
}
