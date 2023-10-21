use axum::{
    extract::Json,
    extract::{FromRef, Path},
    response::IntoResponse,
    Extension,
};
use axum_extra::extract::{OptionalPath, WithRejection};
use axum_macros::debug_handler;
use log::trace;
use reqwest::StatusCode;
use sea_orm::{prelude::*, ActiveValue, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    metadata::AnimeSite,
    server::{router::routes::v1::response::V1Response, state::AppState},
};

#[debug_handler]
pub async fn list(Extension(app_state): Extension<AppState>) -> impl IntoResponse {
    let db = app_state.db.connection();

    let info = match entity::series_sources::Entity::find()
        .order_by_desc(entity::series_sources::Column::Id)
        .all(&db)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            return V1Response::Error(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow::anyhow!("Failed to fetch anime list: {}", e).into(),
            );
        }
    };

    V1Response::Success(json!({
        "series": info,
    }))
}

#[debug_handler]
pub async fn list_for_series(
    Extension(app_state): Extension<AppState>,
    Path(series_id): Path<i32>,
) -> impl IntoResponse {
    let db = app_state.db.connection();

    let info = match entity::series::Entity::find_by_id(series_id)
        .find_with_related(entity::series_sources::Entity)
        .all(&db)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            return V1Response::Error(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow::anyhow!("Failed to fetch anime list: {}", e).into(),
            );
        }
    };
    match info {
        x if x.len() == 1 => {
            let (series, sources) = x.into_iter().next().unwrap();
            V1Response::Success(json!({
                "series": series,
                "sources": sources,
            }))
        }

        _ => V1Response::ErrorEmpty(StatusCode::NOT_FOUND),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAnimeSourcePayload {
    pub series_id: Option<i32>,
    pub series_site: AnimeSite,
    pub series_site_id: String,
}
#[debug_handler]
pub async fn add(
    Extension(app_state): Extension<AppState>,
    OptionalPath(series_id): OptionalPath<i32>,
    WithRejection(Json(payload), _): WithRejection<Json<AddAnimeSourcePayload>, V1Response>,
) -> impl IntoResponse {
    let db = app_state.db.connection();

    let series_id = match series_id.or(payload.series_id) {
        Some(series_id) => series_id,
        None => {
            return V1Response::Error(
                StatusCode::BAD_REQUEST,
                anyhow::anyhow!("No series ID provided (\"seriesId\" field missing)").into(),
            );
        }
    };

    trace!("Adding anime source: {:?}", payload);
    let new = entity::series_sources::ActiveModel {
        for_series_id: ActiveValue::Set(series_id),
        series_site: ActiveValue::Set(payload.series_site.to_string()),
        series_site_id: ActiveValue::Set(payload.series_site_id.clone()),
        ..Default::default()
    };

    match new.insert(&db).await {
        Ok(new) => V1Response::Success(json!({
            "payload": payload,
            "result": new,
        })),
        Err(e)
            if e.sql_err()
                .map(|x| matches!(x, SqlErr::UniqueConstraintViolation(_)))
                .unwrap_or_default() =>
        {
            V1Response::Error(
                StatusCode::CONFLICT,
                anyhow::anyhow!(
                    "Anime source for site {:?} already exists",
                    payload.series_site
                )
                .into(),
            )
        }
        Err(e) => V1Response::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to add anime: {}", e).into(),
        ),
    }
}

#[debug_handler]
pub async fn info(
    Extension(app_state): Extension<AppState>,
    Path(source_id): Path<i32>,
) -> impl IntoResponse {
    let db = app_state.db.connection();

    let info = match entity::series_sources::Entity::find_by_id(source_id)
        .one(&db)
        .await
    {
        Ok(info) => info,
        Err(e) => {
            return V1Response::Error(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow::anyhow!("Failed to fetch anime info: {}", e).into(),
            );
        }
    };

    V1Response::Success(json!({
        "source": info,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAnimeSourcePayload {
    pub for_series_id: Option<i32>,
    pub series_site: Option<AnimeSite>,
    pub series_site_id: Option<String>,
}
impl FromRef<UpdateAnimeSourcePayload> for entity::series_sources::ActiveModel {
    fn from_ref(input: &UpdateAnimeSourcePayload) -> Self {
        let mut new = entity::series_sources::ActiveModel::new();

        if let Some(for_series_id) = &input.for_series_id {
            new.for_series_id = ActiveValue::Set(*for_series_id);
        }

        if let Some(series_site) = &input.series_site {
            new.series_site = ActiveValue::Set(series_site.to_string());
        }

        if let Some(series_site_id) = &input.series_site_id {
            new.series_site_id = ActiveValue::Set(series_site_id.clone());
        }

        new
    }
}
#[debug_handler]
pub async fn update(
    Extension(app_state): Extension<AppState>,
    Path(source_id): Path<i32>,
    WithRejection(Json(payload), _): WithRejection<Json<UpdateAnimeSourcePayload>, V1Response>,
) -> impl IntoResponse {
    let db = app_state.db.connection();

    trace!("Updating anime source: {:?}", payload);
    let mut new = entity::series_sources::ActiveModel::from_ref(&payload);
    new.id = ActiveValue::Unchanged(source_id);

    match new.update(&db).await {
        Ok(new) => V1Response::Success(json!({
            "payload": payload,
            "result": new,
        })),
        Err(e) => V1Response::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to update anime: {}", e).into(),
        ),
    }
}

#[debug_handler]
pub async fn remove(
    Extension(app_state): Extension<AppState>,
    Path(source_id): Path<i32>,
) -> impl IntoResponse {
    let db = app_state.db.connection();

    trace!("Removing anime source: {:?}", source_id);
    match entity::series_sources::Entity::delete_by_id(source_id)
        .exec(&db)
        .await
    {
        Ok(_) => V1Response::Success(json!({
            "sourceId": source_id,
        })),
        Err(e) => V1Response::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to remove anime: {}", e).into(),
        ),
    }
}
