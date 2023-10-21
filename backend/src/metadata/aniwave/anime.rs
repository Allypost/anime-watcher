use std::collections::HashMap;

use anyhow::{anyhow, Result};
use chrono::{NaiveDateTime, TimeZone, Utc};
use log::{debug, trace};
use scraper::{Html, Node, Selector};
use tokio::task;

use crate::metadata::{aniwave::request, AltName, AnimeInfo, AnimeStatus};

async fn get_info_page_html(series_id: &str) -> Result<String> {
    trace!("Getting info page html for series {}", series_id);

    let html = request::get_page(&format!("/watch/{}", series_id), request::RequestType::Html)
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(html)
}

#[allow(clippy::too_many_lines)]
fn get_anime_info_from_html(page_html: &str) -> Result<AnimeInfo> {
    trace!("Parsing info page html");
    let document = Html::parse_document(page_html);
    trace!("Parsed info page html");

    trace!("Getting main container");
    let el_main = document
        .select(&Selector::parse("#watch-main").unwrap())
        .next()
        .ok_or_else(|| anyhow!("Could not find main container"))?;

    trace!("Getting anime id");
    let anime_id = el_main
        .value()
        .attr("data-id")
        .ok_or_else(|| anyhow!("Could not find anime id on main container"))?
        .to_string();

    debug!("Got anime id {}", &anime_id);

    trace!("Getting anime details element");
    let el_details = document
        .select(&Selector::parse("#w-info .info").unwrap())
        .next()
        .ok_or_else(|| anyhow!("Could not find anime details"))?;

    trace!("Getting anime description");
    let description = el_details
        .select(&Selector::parse(".synopsis .content").unwrap())
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(" "))
        .map(|el| el.trim().to_string());

    trace!("Getting anime name element");
    let el_name = el_details
        .select(&Selector::parse("h1.title").unwrap())
        .next()
        .ok_or_else(|| anyhow!("Could not find anime name element"))?;
    let english_name = el_name.text().collect::<Vec<_>>().join(" ");
    let english_name = english_name.trim();

    trace!("Got anime name: {name:?}", name = english_name);

    let mut alt_names = vec![];

    // Japanese romanji
    {
        let japanese_name = el_name.value().attr("data-jp");
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

    let mut meta: HashMap<String, serde_json::Value> = HashMap::new();

    // Other details
    {
        trace!("Getting anime metadata");
        let item_selector = Selector::parse(".bmeta .meta > div").unwrap();
        let detail_els = el_details.select(&item_selector);

        let span_selector = Selector::parse("span").unwrap();
        let a_selector = Selector::parse("a").unwrap();

        for detail_el in detail_els {
            let mut children = detail_el.children();

            let mut detail_name = match children.next().map(|x| x.value()) {
                Some(Node::Text(text)) => text.text.trim().trim_end_matches(':').to_string(),
                None => {
                    trace!("No children");
                    continue;
                }
                Some(node) => {
                    trace!("First child is not text: {:?}", &node);
                    continue;
                }
            };
            detail_name.make_ascii_lowercase();

            let detail_value_el = children.next().map(|x| x.value());
            let detail_value_el = match detail_value_el {
                Some(Node::Element(el)) if el.name() == "span" => {
                    let el = detail_el.select(&span_selector).next().unwrap();

                    el
                }
                Some(node_ref) => {
                    trace!("Second child is not span element: {:?}", &node_ref);
                    continue;
                }
                None => {
                    trace!("No second child");
                    continue;
                }
            };

            match detail_name.as_str() {
                "type" | "country" | "premiered" | "status" | "episodes" | "broadcast" => {
                    let val = detail_value_el.text().collect::<Vec<_>>().join(" ");
                    meta.insert(detail_name, val.trim().into());
                }
                "genres" | "studios" | "producers" => {
                    let val = detail_value_el
                        .select(&a_selector)
                        .map(|x| x.text().collect::<Vec<_>>().join(" ").trim().to_string())
                        .collect::<Vec<_>>();
                    meta.insert(detail_name, val.into());
                }
                "duration" => {
                    let val = detail_value_el.text().collect::<Vec<_>>().join(" ");
                    let val = val.replace(' ', "");
                    let val = duration_str::parse_chrono(&val);

                    if let Ok(val) = val {
                        meta.insert(detail_name, val.to_string().into());
                    }
                }
                "mal" => {
                    let val = match detail_value_el
                        .text()
                        .next()
                        .map(|x| x.trim().parse::<f64>())
                    {
                        Some(Ok(val)) => val,
                        _ => {
                            trace!("Unknown MAL value: {:?}", &detail_value_el);
                            continue;
                        }
                    };
                    meta.insert("malScore".into(), val.into());
                }
                "date aired" => {}
                _ => {
                    trace!("Unknown detail name: {:?}", &detail_name);
                    continue;
                }
            }
        }
    }

    let status = meta
        .get("status")
        .map(std::borrow::ToOwned::to_owned)
        .map(serde_json::from_value::<AnimeStatus>)
        .and_then(std::result::Result::ok)
        .unwrap_or_default();

    let url = {
        if let Some(url) = el_main.value().attr("data-url") {
            url.to_string()
        } else {
            document
                .select(&Selector::parse("link[rel=canonical]").unwrap())
                .next()
                .map(|x| x.value())
                .and_then(|x| x.attr("href"))
                .map_or_else(
                    || {
                        format!(
                            "{base}/watch/{anime_id}",
                            base = request::BASE_URL,
                            anime_id = anime_id
                        )
                    },
                    std::string::ToString::to_string,
                )
        }
    };

    let next_release_estimate = el_main
        .select(&Selector::parse(".next-episode .count-down").unwrap())
        .next()
        .and_then(|x| {
            let unix_secs = x.value().attr("data-target")?.parse::<i64>().ok()?;
            let datetime = NaiveDateTime::from_timestamp_millis(unix_secs * 1000)?;
            let datetime = Utc.from_utc_datetime(&datetime);
            Some(datetime)
        });

    let genres = meta.remove("genres").map_or_else(Vec::new, |x| {
        serde_json::from_value::<Vec<String>>(x.clone()).unwrap_or_default()
    });

    Ok(AnimeInfo {
        id: anime_id,
        name: english_name.to_string(),
        url,
        alt_names,
        description,
        meta,
        status,
        next_release_estimate,
        genres,
        mal_id: None,
        episodes: vec![],
    })
}

pub async fn get_episode_list(aniwave_series_id: &str) -> Result<()> {
    trace!("Getting episode list for series id {}", aniwave_series_id);

    let resp = request::get_page(
        &format!("/ajax/episode/list/{id}", id = aniwave_series_id),
        request::RequestType::Api,
    )
    .await?
    .text()
    .await?;

    dbg!(resp);

    Ok(())
}

pub async fn get_info(series_id: &str) -> Result<AnimeInfo> {
    debug!("Getting info for series {}", series_id);

    let info_page_html = get_info_page_html(series_id).await?;

    let info = {
        task::spawn_blocking(move || get_anime_info_from_html(&info_page_html))
            .await
            .unwrap()?
    };

    get_episode_list(&info.id).await?;

    Ok(info)
}
