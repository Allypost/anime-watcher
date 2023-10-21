pub use date_field::*;
pub use page_status::*;
use serde::{Deserialize, Serialize};
pub use video_info::*;

pub mod date_field;
pub mod page_status;
pub mod video_info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse<T> {
    pub data: T,
}
