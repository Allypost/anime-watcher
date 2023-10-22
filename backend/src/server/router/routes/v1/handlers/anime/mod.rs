use axum::{extract::Path, Extension, Json};
use axum_extra::extract::WithRejection;
use axum_macros::debug_handler;
use log::trace;
use reqwest::StatusCode;
use sea_orm::{prelude::*, QueryOrder, Set, Unchanged};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    metadata::{self, MetaSeriesInfo},
    server::{
        router::routes::v1::response::V1Response, server_timing::ServerTimings, state::AppState,
    },
};

pub mod info;
pub mod sources;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPayload {
    pub name: String,
    pub description: Option<String>,
    pub mal_id: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddResponse {
    pub payload: AddPayload,
    pub result: entity::series::Model,
}
#[debug_handler]
pub async fn add(
    Extension(app_state): Extension<AppState>,
    WithRejection(Json(payload), _): WithRejection<Json<AddPayload>, V1Response>,
) -> V1Response<AddResponse> {
    let db = app_state.db.connection();

    trace!("Adding anime: {:?}", payload);
    let new = entity::series::ActiveModel {
        name: Set(payload.name.clone()),
        description: Set(payload.description.clone()),
        mal_id: Set(payload.mal_id),
        ..Default::default()
    };

    match new.insert(&db).await {
        Ok(result) => V1Response::Success(AddResponse { payload, result }),
        Err(e)
            if e.sql_err()
                .map(|x| matches!(x, SqlErr::UniqueConstraintViolation(_)))
                .unwrap_or_default() =>
        {
            V1Response::Error(
                StatusCode::CONFLICT,
                anyhow::anyhow!("Anime already exists").into(),
            )
        }
        Err(e) => V1Response::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to add anime: {}", e).into(),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MalInfoResponse {
    pub anime: entity::series::Model,
    pub mal_info: metadata::myanimelist::anime::details::AnimeDetails,
}
#[debug_handler]
pub async fn mal_info(
    Extension(app_state): Extension<AppState>,
    Path(series_id): Path<i32>,
) -> V1Response<MalInfoResponse> {
    let db = app_state.db.connection();

    let anime = entity::series::Entity::find_by_id(series_id).one(&db).await;

    let anime = match anime {
        Ok(Some(anime)) => anime,
        Ok(None) => {
            return V1Response::Error(
                StatusCode::NOT_FOUND,
                anyhow::anyhow!("Anime not found").into(),
            );
        }
        Err(e) => {
            return V1Response::Error(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow::anyhow!("Failed to fetch anime: {}", e).into(),
            );
        }
    };

    let mal_id = match anime.mal_id {
        Some(mal_id) => mal_id,
        None => {
            return V1Response::Error(
                StatusCode::NOT_FOUND,
                anyhow::anyhow!("Anime doesn't have a MyAnimeList id associated with it").into(),
            );
        }
    };

    #[allow(clippy::cast_sign_loss)]
    let mal_info = metadata::myanimelist::anime::details::get_details(mal_id as u32).await;

    match mal_info {
        Ok(mal_info) => V1Response::Success(MalInfoResponse { anime, mal_info }),
        Err(e) => V1Response::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to fetch MAL info: {}", e).into(),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePayload {
    pub name: String,
    pub description: Option<String>,
    pub mal_id: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateResponse {
    pub payload: UpdatePayload,
    pub result: entity::series::Model,
}
#[debug_handler]
pub async fn update(
    Extension(app_state): Extension<AppState>,
    Path(series_id): Path<i32>,
    WithRejection(Json(payload), _): WithRejection<Json<UpdatePayload>, V1Response>,
) -> V1Response<UpdateResponse> {
    let db = app_state.db.connection();

    trace!("Updating anime: {:?}", payload);
    let model = entity::series::ActiveModel {
        id: Unchanged(series_id),
        name: Set(payload.name.clone()),
        description: Set(payload.description.clone()),
        mal_id: Set(payload.mal_id),
        ..Default::default()
    };

    match model.update(&db).await {
        Ok(result) => V1Response::Success(UpdateResponse { payload, result }),
        Err(e) => V1Response::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to update anime: {}", e).into(),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveResponse {
    pub series_id: i32,
}
#[debug_handler]
pub async fn remove(
    Extension(app_state): Extension<AppState>,
    Path(series_id): Path<i32>,
) -> V1Response<RemoveResponse> {
    let db = app_state.db.connection();

    trace!("Removing anime: {:?}", series_id);
    match entity::series::Entity::delete_by_id(series_id)
        .exec(&db)
        .await
    {
        Ok(_) => V1Response::Success(RemoveResponse { series_id }),
        Err(e) => V1Response::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to remove anime: {}", e).into(),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponseItem {
    pub anime: entity::series::Model,
    pub sources: Vec<entity::series_sources::Model>,
}
pub type ListResponse = Vec<ListResponseItem>;
#[debug_handler]
pub async fn list(Extension(app_state): Extension<AppState>) -> V1Response<ListResponse> {
    let db = app_state.db.connection();

    let list = match entity::series::Entity::find()
        .order_by_desc(entity::series::Column::Id)
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

    let list = list
        .into_iter()
        .map(|(anime, sources)| ListResponseItem { anime, sources })
        .collect::<Vec<_>>();

    V1Response::Success(list)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub anime: entity::series::Model,
    pub sources: Vec<entity::series_sources::Model>,
}
#[debug_handler]
pub async fn info(
    Extension(app_state): Extension<AppState>,
    Path(series_id): Path<i32>,
) -> V1Response<InfoResponse> {
    let db = app_state.db.connection();

    let anime = entity::series::Entity::find_by_id(series_id)
        .find_with_related(entity::series_sources::Entity)
        .order_by_desc(entity::series_sources::Column::Id)
        .all(&db)
        .await;

    let anime = match anime {
        Ok(anime) => anime,
        Err(e) => {
            return V1Response::Error(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow::anyhow!("Failed to fetch anime: {}", e).into(),
            );
        }
    };

    match anime {
        a if a.len() == 1 => {
            let (anime, sources) = a.into_iter().next().unwrap();
            V1Response::Success(InfoResponse { anime, sources })
        }

        _ => V1Response::ErrorEmpty(StatusCode::NOT_FOUND),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoExtendedResponse {
    pub anime: entity::series::Model,
    pub sources: Vec<serde_json::Value>,
}
#[debug_handler]
pub async fn info_extended(
    Extension(app_state): Extension<AppState>,
    Extension(server_timings): Extension<ServerTimings>,
    Path(series_id): Path<i32>,
) -> V1Response<InfoExtendedResponse> {
    let db = app_state.db.connection();

    server_timings.add_started("db", None);
    let anime = entity::series::Entity::find_by_id(series_id)
        .find_with_related(entity::series_sources::Entity)
        .all(&db)
        .await;
    server_timings.end("db");
    let anime = match anime {
        Ok(anime) => anime,
        Err(e) => {
            return V1Response::Error(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow::anyhow!("Failed to fetch anime: {}", e).into(),
            );
        }
    };

    let (anime, sources) = match anime.into_iter().next() {
        Some(anime) => anime,
        None => {
            return V1Response::Error(
                StatusCode::NOT_FOUND,
                anyhow::anyhow!("Anime not found").into(),
            );
        }
    };

    let sources = sources
        .into_iter()
        .filter_map(|source| {
            let meta = serde_json::from_value::<MetaSeriesInfo>(json!({
                "site": source.series_site,
                "seriesId": source.series_site_id,
            }));

            let meta = match meta {
                Ok(meta) => meta,
                Err(e) => {
                    trace!("Error parsing source metadata: {:?}", e);
                    return None;
                }
            };

            server_timings.add_started(format!("source_{}", source.series_site).as_ref(), None);
            let task = tokio::task::spawn(async move {
                let data = metadata::series_info(meta.clone()).await.ok()?;

                Some(json!({
                    "source": meta,
                    "data": data,
                }))
            });

            Some(task)
        })
        .collect::<Vec<_>>();

    let sources = futures::future::join_all(sources)
        .await
        .into_iter()
        .filter_map(std::result::Result::ok)
        .flatten()
        .collect::<Vec<_>>();

    V1Response::Success(InfoExtendedResponse { anime, sources })
}
