use std::{
    net::{SocketAddr, TcpListener},
    time::Duration,
};

use axum::{
    http::{HeaderName, HeaderValue, Request},
    middleware::{self, Next},
    response::Response,
    Extension, Server, ServiceExt,
};
use log::{debug, info, trace};
use reqwest::header;
use tower::{layer::Layer, ServiceBuilder};
use tower_http::{
    cors::{self, CorsLayer},
    normalize_path::NormalizePathLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, TraceLayer},
    ServiceBuilderExt,
};

use crate::config::CONFIG;
use crate::server::state::AppState;

mod router;
mod server_timing;
mod state;

lazy_static::lazy_static! {
    static ref CACHE_CONTROL: HeaderValue = HeaderValue::from_static("private, max-age=0");
}

async fn server_timings_fn<B>(
    Extension(server_timings): Extension<server_timing::ServerTimings>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    server_timings.add_started("app", None);

    let mut resp = next.run(request).await;
    if let Some(header_value) = server_timings.to_header_value() {
        resp.headers_mut().insert(
            server_timing::HEADER_NAME.clone(),
            header_value.parse().unwrap(),
        );
    }

    resp
}

#[tokio::main]
pub async fn run() -> anyhow::Result<()> {
    let listener = TcpListener::bind((CONFIG.server.host.clone(), CONFIG.server.port))?;
    trace!("Bound application on {:?}", listener.local_addr()?);
    info!(
        "Starting server on {location}",
        location = listener.local_addr()?,
    );

    let app_state = AppState::new().await?;
    debug!("Using app state: {:?}", app_state);

    app_state.db.init().await?;

    let server_timings = server_timing::ServerTimings::new();

    let x_request_id = HeaderName::from_static("x-request-id");
    let router = router::create_router()
        .route_layer(middleware::from_fn(server_timings_fn))
        .layer(
            CorsLayer::new()
                .allow_methods(cors::Any)
                .allow_origin(cors::Any),
        )
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(Extension(server_timings))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(|request: &Request<_>, _span: &_| {
                            let request_id = request
                                .extensions()
                                .get::<RequestId>()
                                .and_then(|id| id.header_value().to_str().ok())
                                .unwrap_or("-");
                            let headers = request.headers();
                            info!(
                                "[{id}] \"{method} {path} {http_type:?}\" {user_agent}",
                                id = request_id,
                                http_type = request.version(),
                                method = request.method(),
                                path = request.uri().path(),
                                user_agent = headers
                                    .get(header::USER_AGENT)
                                    .map_or("-", |x| x.to_str().unwrap_or("-")),
                            );
                        })
                        .on_response(|response: &Response<_>, latency, _span: &_| {
                            let header_unknown = HeaderValue::from_static("-");
                            let request_id = response
                                .extensions()
                                .get::<RequestId>()
                                .map_or(&header_unknown, RequestId::header_value)
                                .to_str()
                                .unwrap_or("-");

                            debug!(
                                "[{id}] RESP {status} in {duration:?}",
                                id = request_id,
                                status = response.status().as_u16(),
                                duration = latency
                            );
                        })
                        .on_body_chunk(())
                        .on_eos(|_trailers: Option<&_>, stream_duration, _span: &_| {
                            debug!("stream closed after {:?}", stream_duration);
                        })
                        .on_failure(|_error, _latency, _span: &_| {
                            debug!("something went wrong");
                        }),
                )
                .layer(TimeoutLayer::new(Duration::from_secs(60)))
                .map_response_body(axum::body::boxed)
                .layer(PropagateRequestIdLayer::new(x_request_id))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::CACHE_CONTROL,
                    |_response: &Response<_>| Some(CACHE_CONTROL.clone()),
                ))
                .layer(SetResponseHeaderLayer::appending(
                    header::DATE,
                    |_response: &Response<_>| {
                        Some(chrono::Utc::now().to_rfc2822().parse().unwrap())
                    },
                ))
                .layer(Extension(app_state)),
        );
    let service = NormalizePathLayer::trim_trailing_slash().layer(router);

    info!("Server ready");
    Server::from_tcp(listener)?
        .serve(service.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .map_err(Into::into)
}
