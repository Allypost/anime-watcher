use axum::{routing::get, Router};
use reqwest::StatusCode;
use tower_http::validate_request::ValidateRequestHeaderLayer;

use self::response::V1Response;

pub mod handlers;
mod response;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(handlers::index::index))
        .nest(
            "/anime",
            Router::new()
                .route("/", get(handlers::anime::list).put(handlers::anime::add))
                .nest(
                    "/:series_id",
                    Router::new()
                        .route(
                            "/",
                            get(handlers::anime::info)
                                .patch(handlers::anime::update)
                                .delete(handlers::anime::remove),
                        )
                        .route("/mal-info", get(handlers::anime::mal_info))
                        .route("/details", get(handlers::anime::info_extended))
                        .route(
                            "/sources",
                            get(handlers::anime::sources::list_for_series)
                                .put(handlers::anime::sources::add),
                        ),
                )
                .nest(
                    "/sources",
                    Router::new()
                        .route(
                            "/",
                            get(handlers::anime::sources::list).put(handlers::anime::sources::add),
                        )
                        .route(
                            "/:source_id",
                            get(handlers::anime::sources::info)
                                .patch(handlers::anime::sources::update)
                                .delete(handlers::anime::sources::remove),
                        ),
                )
                .route("/info", get(handlers::anime::info::anime_info_floating))
                .route(
                    "/info/for-episode",
                    get(handlers::anime::info::episode_info_floating),
                ),
        )
        .fallback(|| async { V1Response::<()>::ErrorEmpty(StatusCode::NOT_FOUND) })
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
}
