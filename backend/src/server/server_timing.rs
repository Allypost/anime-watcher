use std::{
    sync::RwLock,
    time::{Duration, Instant},
};

use axum::http::{HeaderName, HeaderValue};
use serde::{ser::SerializeStruct, Serialize};
use tower_http::set_header::MakeHeaderValue;

lazy_static::lazy_static! {
    pub static ref HEADER_NAME: HeaderName = HeaderName::from_static("server-timing");
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimings {
    items: RwLock<Vec<TimingItem>>,
}

impl Clone for ServerTimings {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl ServerTimings {
    pub fn new() -> Self {
        Self {
            items: RwLock::new(Vec::new()),
        }
    }

    pub fn to_header_value(&self) -> Option<String> {
        let items = self.items.read().ok()?;

        if items.is_empty() {
            return None;
        }

        let mut new_items = Vec::with_capacity(items.len());

        for item in items.iter() {
            new_items.push(item.to_string());
        }

        Some(new_items.join(", "))
    }

    pub fn add(&self, name: String, description: Option<String>) -> &Self {
        self.items.write().unwrap().push(TimingItem {
            name,
            description,
            state: TimingItemState::Empty,
        });

        self
    }

    pub fn add_started(&self, name: &str, description: Option<String>) -> &Self {
        self.add(name.to_string(), description).start(name)
    }

    pub fn start(&self, name: &str) -> &Self {
        let mut items = self.items.write().unwrap();
        let item = items.iter_mut().find(|item| item.name == name);

        if let Some(item) = item {
            item.state = TimingItemState::Started(Instant::now());
        }

        self
    }

    pub fn end(&self, name: &str) -> &Self {
        let mut items = self.items.write().unwrap();
        let item = items.iter_mut().find(|item| item.name == name);

        if let Some(item) = item {
            item.state = TimingItemState::Ended(item.duration().unwrap());
        }

        self
    }
}

impl<T> MakeHeaderValue<T> for ServerTimings {
    fn make_header_value(&mut self, _message: &T) -> Option<HeaderValue> {
        HeaderValue::from_str(&self.to_header_value()?).ok()
    }
}

#[derive(Debug, Clone)]
enum TimingItemState {
    Empty,
    Started(Instant),
    Ended(Duration),
}

#[derive(Debug, Clone)]
pub struct TimingItem {
    name: String,
    description: Option<String>,
    state: TimingItemState,
}

impl TimingItem {
    fn duration(&self) -> Option<Duration> {
        match self.state {
            TimingItemState::Started(start) => Some(start.elapsed()),
            TimingItemState::Ended(dur) => Some(dur),
            TimingItemState::Empty => None,
        }
    }
}

impl ToString for TimingItem {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str(&self.name);

        if let Some(desc) = &self.description {
            s.push_str(&format!(";desc={}", serde_json::to_string(desc).unwrap()));
        }

        #[allow(clippy::cast_precision_loss)]
        {
            if let Some(dur) = self.duration() {
                let micros = dur.as_micros();
                let millis_frac = micros as f64 / 1000_f64;
                s.push_str(&format!(";dur={}", millis_frac));
            }
        }

        s
    }
}

impl Serialize for TimingItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("TimingItem", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("desc", &self.description)?;
        s.serialize_field("dur", &self.duration())?;
        s.end()
    }
}
