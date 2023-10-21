pub mod util {
    pub mod bool_str {
        use serde::{Deserialize, Deserializer, Serializer};

        #[allow(clippy::trivially_copy_pass_by_ref)]
        pub fn serialize<S>(b: &bool, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_bool(*b)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            match s.as_str() {
                "false" | "0" | "no" => Ok(false),
                _ => Ok(true),
            }
        }
    }
}

pub mod prelude {
    pub use anyhow::{anyhow, bail, Context, Result};
    pub use chrono::prelude::*;
    pub use lazy_static::lazy_static;
    pub use log::{debug, error, info, trace, warn};
    pub use serde::{Deserialize, Serialize};
}
