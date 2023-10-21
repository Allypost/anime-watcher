use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::Serialize;

use crate::server::router::response::{error::ApiError, ApiResponse};

pub const API_VERSION: &str = "v1";

#[derive(Debug)]
pub enum V1Response<TData: Serialize + Send = ()> {
    Success(TData),
    Error(StatusCode, ApiError),
    ErrorEmpty(StatusCode),
}

// unsafe impl Send for V1Response {}

impl<TData: Serialize + Send> From<V1Response<TData>> for ApiResponse<TData> {
    fn from(r: V1Response<TData>) -> Self {
        let resp = ApiResponse::new(API_VERSION, StatusCode::OK);

        match r {
            V1Response::Success(data) => resp.with_success_body(data),
            V1Response::Error(status, error) => {
                resp.with_status_code(status).with_error_body(error)
            }
            V1Response::ErrorEmpty(status) => resp.with_status_code(status),
        }
    }
}

impl<TData: Serialize + Send> IntoResponse for V1Response<TData> {
    fn into_response(self) -> Response {
        let resp: ApiResponse<TData> = self.into();

        (resp.meta.status_code, Json(resp)).into_response()
    }
}

impl From<JsonRejection> for V1Response {
    fn from(rejection: JsonRejection) -> Self {
        V1Response::Error(rejection.status(), rejection.body_text().into())
    }
}

impl From<QueryRejection> for V1Response {
    fn from(rejection: QueryRejection) -> Self {
        V1Response::Error(rejection.status(), rejection.body_text().into())
    }
}
