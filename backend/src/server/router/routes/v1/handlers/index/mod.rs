use axum_macros::debug_handler;

use crate::server::router::routes::v1::response::V1Response;

#[debug_handler]
pub async fn index() -> V1Response<&'static str> {
    V1Response::Success("Hello, World!")
}
