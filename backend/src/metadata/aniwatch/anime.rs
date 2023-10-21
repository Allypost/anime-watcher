use scraper::{Html, Selector};
use serde::Deserialize;
use std::collections::HashMap;
use tokio::{join, task};

use crate::metadata::{aniwatch::episode, common::prelude::*, AltName, AnimeInfo, AnimeStatus};

use super::request;

#[allow(dead_code)]
pub async fn get_id(series_id: &str) -> Result<String> {
    let url = format!("/watch/{series_id}");
    let page_html = request::get_page(&url)
        .await?
        .error_for_status()?
        .text()
        .await?;

    let document = Html::parse_document(&page_html);

    let id_sync_data = Selector::parse("#syncData").unwrap();

    let sync_data = document.select(&id_sync_data).next();
    let sync_data = if let Some(data) = sync_data {
        data
    } else {
        bail!("Could not find sync data")
    };

    let sync_data = sync_data.inner_html();
    let sync_data: SyncData = serde_json::from_str(&sync_data)?;

    Ok(sync_data.anime_id)
}

#[allow(clippy::too_many_lines)]
fn get_anime_info_from_html(page_html: &str) -> Result<AnimeInfo> {
    trace!("Parsing info page html");
    let document = Html::parse_document(page_html);
    trace!("Parsed info page html");

    trace!("Getting anime id");
    let anime_id = document
        .select(&Selector::parse("#wrapper").unwrap())
        .next()
        .ok_or_else(|| anyhow!("Could not find container"))?
        .value()
        .attr("data-id")
        .ok_or_else(|| anyhow!("Could not find anime id"))?
        .to_string();

    debug!("Got anime id {}", &anime_id);

    trace!("Getting anime details element");
    let el_details = document
        .select(&Selector::parse("#ani_detail").unwrap())
        .next()
        .ok_or_else(|| anyhow!("Could not find anime details"))?;

    trace!("Getting anime description");
    let description = el_details
        .select(&Selector::parse(".anisc-detail .film-description").unwrap())
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(" "))
        .map(|el| el.trim().to_string());

    trace!("Getting anime name element");
    let el_name = el_details
        .select(&Selector::parse(".anisc-detail .film-name").unwrap())
        .next()
        .ok_or_else(|| anyhow!("Could not find anime name element"))?;

    let english_name = el_name.text().collect::<Vec<_>>().join(" ");
    let english_name = english_name.trim();

    trace!("Got anime name: {name:?}", name = english_name);

    trace!("Getting sync data");
    let sync_data = {
        let id_sync_data = Selector::parse("#syncData").unwrap();

        let sync_data = document.select(&id_sync_data).next();
        let Some(sync_data) = sync_data else {
            bail!("Could not find sync data")
        };

        let sync_data = sync_data.inner_html();

        serde_json::from_str::<SyncData>(&sync_data)?
    };
    trace!("Got sync data: {data:?}", data = sync_data);

    let mut alt_names = vec![];

    // Japanese romanji
    {
        let japanese_name = el_name.value().attr("data-jname");
        if let Some(japanese_name) = japanese_name {
            trace!("Got japanese name: {name:?}", name = japanese_name);
            if japanese_name != english_name {
                alt_names.push(AltName {
                    name: japanese_name.to_string(),
                    language: "ja-JA".to_string(),
                });
            }
        }
    }

    let mut meta = HashMap::new();

    // Other details
    {
        trace!("Getting anime metadata");
        let item_selector = Selector::parse(".anisc-info .item").unwrap();
        let detail_els = el_details.select(&item_selector);

        for detail_el in detail_els {
            let detail_name = detail_el
                .select(&Selector::parse(".item-head").unwrap())
                .next();

            if detail_name.is_none() {
                continue;
            }
            let detail_name = detail_name.unwrap().text().collect::<Vec<_>>().join(" ");
            let mut detail_name = detail_name.trim().trim_end_matches(':').to_string();
            detail_name.make_ascii_lowercase();

            let detail_type = detail_el
                .value()
                .attr("class")
                .and_then(|x| x.split(' ').last());

            match detail_name.as_str() {
                "japanese" => {
                    let detail_value = detail_el
                        .select(&Selector::parse(".name").unwrap())
                        .next()
                        .map(|el| el.text().collect::<Vec<_>>().join(" "));

                    if let Some(name) = detail_value {
                        alt_names.push(AltName {
                            name: name.trim().to_string(),
                            language: "ja-JA".to_string(),
                        });
                    }
                }

                _ if Some("item-title") == detail_type => {
                    let detail_value = detail_el
                        .select(&Selector::parse(".name").unwrap())
                        .map(|el| el.text().collect::<Vec<_>>().join(" "))
                        .collect::<Vec<_>>()
                        .join(", ");

                    if !detail_value.is_empty() {
                        meta.insert(detail_name.to_string(), detail_value.into());
                    }
                }

                _ if Some("item-list") == detail_type => {
                    let detail_value = detail_el
                        .select(&Selector::parse("*:not(.item-head)").unwrap())
                        .map(|el| el.text().collect::<Vec<_>>().join(" "))
                        .collect::<Vec<_>>()
                        .join(", ");

                    if !detail_value.is_empty() {
                        meta.insert(detail_name.to_string(), detail_value.into());
                    }
                }

                _ => {}
            }
        }

        if meta.contains_key("mal score") {
            let mal_score: serde_json::Value = meta.remove("mal score").unwrap();
            meta.insert("malScore".to_string(), mal_score);
        }

        if let Some(dur) = meta.get("duration").and_then(serde_json::Value::as_str) {
            if let Ok(dur) = duration_str::parse_chrono(dur) {
                meta.insert("duration".to_string(), dur.to_string().into());
            }
        }
    }

    let status = meta
        .get("status")
        .map(std::borrow::ToOwned::to_owned)
        .map(serde_json::from_value::<AnimeStatus>)
        .and_then(std::result::Result::ok)
        .unwrap_or_default();

    let genres = meta.remove("genres").map_or_else(Vec::new, |x| {
        x.as_str()
            .map(|x| x.split(',').map(|x| x.trim().to_string()).collect())
            .unwrap_or_default()
    });

    Ok(AnimeInfo {
        id: anime_id,
        name: english_name.to_string(),
        mal_id: sync_data.mal_id.and_then(|x| x.parse().ok()),
        url: sync_data.series_url,
        alt_names,
        description,
        meta,
        status,
        genres,
        episodes: vec![],
        next_release_estimate: None,
        // ..AnimeInfo::default()
    })
}

fn get_anime_estimated_time_from_html(page_html: &str) -> Result<DateTime<Utc>> {
    trace!("Parsing watch page html");
    let document = Html::parse_document(page_html);
    trace!("Parsed watch page html");

    trace!("Getting estimated release time");
    let release_time = document
        .select(&Selector::parse("#schedule-date").unwrap())
        .next()
        .ok_or_else(|| anyhow!("Could not find estimated release time element"))?
        .value()
        .attr("data-value")
        .ok_or_else(|| anyhow!("Could not parse estimated release time"))?
        .to_string();

    trace!("Got raw estimated release time {}", &release_time);

    let parsed = NaiveDateTime::parse_from_str(&release_time, "%Y-%m-%d %H:%M:%S")?;
    let parsed = Utc.from_utc_datetime(&parsed);

    debug!("Parsed estimated release time {}", &parsed);

    Ok(parsed)
}

async fn maybe_get_watch_page_html(
    series_id: &str,
    with_esitmated_release: bool,
) -> Result<Option<String>> {
    if !with_esitmated_release {
        return Ok(None);
    }

    let watch_page_html = request::get_page(&format!("/watch/{series_id}"))
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(Some(watch_page_html))
}

async fn get_info_page_html(series_id: &str) -> Result<String> {
    let html = request::get_page(&format!("/{series_id}"))
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(html)
}

pub async fn get_info(series_id: &str, with_esitmated_release: bool) -> Result<AnimeInfo> {
    debug!("Getting info for series {}", series_id);

    let (info_page_html, watch_page_html) = join!(
        get_info_page_html(series_id),
        maybe_get_watch_page_html(series_id, with_esitmated_release),
    );

    let mut info = {
        task::spawn_blocking(move || get_anime_info_from_html(&info_page_html?))
            .await
            .unwrap()?
    };

    if let Some(page_html) = watch_page_html? {
        if let Ok(release_time) =
            task::spawn_blocking(move || get_anime_estimated_time_from_html(&page_html)).await?
        {
            info.next_release_estimate = Some(release_time);
        }
    }

    info.episodes = episode::get_list(&info.id).await.unwrap_or_default();

    Ok(info)
}

#[derive(Debug, Clone, Deserialize)]

struct SyncData {
    // page: String,
    anime_id: String,
    mal_id: Option<String>,
    // anilist_id: Option<String>,
    series_url: String,
}
