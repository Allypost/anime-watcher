use anyhow::{anyhow, Result};
use futures::future;
use log::{debug, trace};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::task;

use crate::metadata::{aniwatch::request, EpisodeInfo, SeriesTranslation};

pub async fn get_list(anime_id: &str) -> Result<Vec<EpisodeInfo>> {
    debug!("Getting episode list for {id}", id = anime_id);
    let url = format!("/ajax/v2/episode/list/{anime_id}");
    let resp = request::get_page(&url)
        .await?
        .error_for_status()?
        .text()
        .await?;
    trace!("Got response for {id:?}", id = anime_id);
    let resp: ApiHtmlResponse = serde_json::from_str(&resp).map_err(|e| {
        anyhow!(json!({
            "error": e.to_string(),
            "url": url,
            "response": resp,
        }))
    })?;

    trace!("Parsing document for {id:?}", id = anime_id);
    let document = Html::parse_fragment(&resp.html);
    trace!("Parsed document for {id:?}", id = anime_id);

    let class_ep_item = Selector::parse(".ep-item").unwrap();

    trace!("Parsing episode data for {id:?}", id = anime_id);
    document
        .select(&class_ep_item)
        .map(|el| -> Result<EpisodeInfo> {
            let el = el.value();

            let id = el
                .attr("data-id")
                .ok_or_else(|| anyhow!("Couldn't extract id"))?
                .to_string();

            let title = el
                .attr("title")
                .ok_or_else(|| anyhow!("Couldn't extract title"))?
                .to_string();

            let episode_number = el
                .attr("data-number")
                .ok_or_else(|| anyhow!("Couldn't extract episode number"))?
                .parse::<f64>()?;

            let url = el
                .attr("href")
                .ok_or_else(|| anyhow!("Couldn't extract url"))?
                .to_string();

            Ok(EpisodeInfo {
                id,
                title,
                episode_number,
                url: format!("{BASE_URL}{url}", BASE_URL = request::BASE_URL, url = url),
                translation: SeriesTranslation::Unknown,
            })
        })
        .collect()
}

#[derive(Debug, Clone, Deserialize)]
struct ApiHtmlResponse {
    html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeSource {
    id: String,
    name: String,
    url: String,
    #[serde(rename = "type")]
    source_type: SeriesTranslation,
}

fn get_episode_sources_from_html(html: &str) -> Vec<EpisodeSource> {
    let document = Html::parse_fragment(html);

    let selector = Selector::parse(".servers-sub, .servers-dub").unwrap();
    trace!("Using selector: {:?}", selector);
    let server_item_selector = Selector::parse(".server-item").unwrap();

    let res = document
        .select(&selector)
        .collect::<Vec<_>>()
        .into_iter()
        .flat_map(|el| el.select(&server_item_selector))
        .filter_map(|el| {
            trace!("Parsing episode source on element: {:?}", el.value());
            let id = el.value().attr("data-id")?.to_string();
            let name = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
            let source_type = el
                .value()
                .attr("data-type")?
                .parse::<SeriesTranslation>()
                .ok()?;

            Some(EpisodeSource {
                id,
                name,
                source_type,
                url: String::new(),
            })
        })
        .collect();

    res
}

pub async fn get_info(anime_id: &str, episode_id: &str) -> Result<serde_json::Value> {
    debug!(
        "Getting episode info for anime={anime:?} episode={episode:?}",
        anime = anime_id,
        episode = episode_id,
    );

    let episodes_list = {
        let id = anime_id
            .split('-')
            .last()
            .ok_or_else(|| anyhow!("Couldn't extract id from anime id: {id:?}", id = anime_id))?
            .to_string();
        task::spawn(async move { get_list(&id).await })
    };

    trace!("Getting api episode info for {id:?}", id = episode_id);
    let resp: ApiHtmlResponse = request::get_page(&format!(
        "/ajax/v2/episode/servers?episodeId={episode_id}",
        episode_id = episode_id
    ))
    .await?
    .error_for_status()?
    .json()
    .await?;

    trace!(
        "Parsing document for sources for episode {id:?}",
        id = episode_id
    );
    let sources = task::spawn_blocking(move || get_episode_sources_from_html(&resp.html))
        .await?
        .into_iter()
        .map(|mut source| async {
            trace!("Getting source info for {id:?}", id = source.id);
            let url = format!(
                "/ajax/v2/episode/sources?id={episode_id}",
                episode_id = source.id,
            );
            let resp = request::get_page(&url)
                .await
                .ok()?
                .error_for_status()
                .ok()?
                .text()
                .await
                .ok()?;

            trace!("Parsing source info for {id:?}", id = source.id);

            let resp: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&resp)
                .map_err(|e| {
                    anyhow!(json!({
                        "error": e.to_string(),
                        "response": resp,
                    }))
                })
                .ok()?;

            source.url = resp
                .get("link")
                .and_then(|x| x.as_str())
                .map(std::string::ToString::to_string)?;

            Some(source)
        })
        .map(task::spawn)
        .collect::<Vec<_>>();

    trace!(
        "Waiting for sources and episodes anime={anime:?} episode={episode:?}",
        anime = anime_id,
        episode = episode_id,
    );
    let (episodes_list, sources) = tokio::join!(episodes_list, future::join_all(sources));

    let sources = sources
        .into_iter()
        .filter_map(std::result::Result::ok)
        .flatten()
        .collect::<Vec<_>>();

    let episodes_list = episodes_list??;

    let episode = episodes_list
        .into_iter()
        .find(|x| x.id == episode_id)
        .ok_or_else(|| anyhow!("Couldn't find episode"))?;

    Ok(json!({
        "episode": episode,
        "sources": sources,
    }))
}
