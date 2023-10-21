use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod airing_status;
pub mod date_time_field;
pub mod field_or_empty;
pub mod translation_type;

pub use super::schema;
pub use airing_status::*;
pub use date_time_field::*;
pub use field_or_empty::*;
pub use translation_type::*;

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum VaildCountryOriginEnumType {
    All,
    Jp,
    Cn,
    Kr,
    Other,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct BigInt(pub String);

impl From<BigInt> for i64 {
    fn from(big_int: BigInt) -> Self {
        big_int.0.parse().unwrap_or_default()
    }
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Object<T = Value>(pub T);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubDubRaw<T> {
    pub sub: Option<T>,
    pub dub: Option<T>,
    pub raw: Option<T>,
}
