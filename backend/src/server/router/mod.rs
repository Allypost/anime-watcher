#![allow(clippy::unused_async)]

use axum::{
    response::IntoResponse,
    routing::{get, on, MethodFilter},
    Extension, Router,
};
use axum_macros::debug_handler;
use reqwest::StatusCode;

use self::response::ApiResponse;

use super::state::AppState;

mod response;
mod routes;

#[debug_handler]
async fn db_ping(Extension(app_state): Extension<AppState>) -> impl IntoResponse {
    let conn = app_state.db.connection();

    match conn.ping().await {
        Ok(()) => ApiResponse::empty("v0", StatusCode::OK),
        Err(e) => ApiResponse::empty("v0", StatusCode::INTERNAL_SERVER_ERROR)
            .with_error_body(anyhow::anyhow!(e).into()),
    }
}

#[debug_handler]
async fn handle_fallback() -> impl IntoResponse {
    let msg = format!(
        "Unknown route. Current api is {api} and lives at /{api}",
        api = "v1"
    );

    ApiResponse::empty("v0", StatusCode::NOT_FOUND).with_error_body(msg.into())
}

pub fn create_router() -> Router {
    Router::new()
        .route(
            "/health",
            on(MethodFilter::all(), || async {
                ApiResponse::empty("v0", StatusCode::OK)
            }),
        )
        .route("/db-ping", get(db_ping))
        .nest("/v1", routes::v1::create_router())
        .fallback(handle_fallback)
}
