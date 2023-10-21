#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::single_match_else)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::uninlined_format_args)]

use config::CONFIG;
use log::trace;

mod config;
mod db;
mod logger;
mod metadata;
mod server;

fn main() {
    logger::init();
    trace!("Config: {:?}", *CONFIG);

    server::run().unwrap();
}

#[allow(dead_code)]
fn main2() {
    unused::main();
}

// #[cfg(all(unix, not(unix)))]
#[allow(clippy::all)]
#[allow(dead_code)]
mod unused {
    use config::CONFIG;
    use log::{debug, trace};
    use metadata::AnimeInfo;
    use std::fs::File;
    use std::io::prelude::*;
    use tokio::{runtime, task::JoinSet};

    use crate::{config, logger, metadata};

    pub fn main() {
        logger::init();
        trace!("Config: {:?}", *CONFIG);

        let series = vec![
            "my-happy-marriage-18490",
            "link-click-season-2-18459",
            "reign-of-the-seven-spellblades-18488",
            "helck-18475",
            "zom-100-bucket-list-of-the-dead-18423",
            "jujutsu-kaisen-2nd-season-18413",
            "undead-murder-farce-18474",
            "mushoku-tensei-jobless-reincarnation-season-2-18418",
        ];

        let runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let mut results: Vec<AnimeInfo> = vec![];
        runtime.block_on(async {
            let mut set = JoinSet::new();
            for s in series {
                set.spawn(metadata::series_info(metadata::MetaSeriesInfo::Aniwatch(
                    metadata::aniwatch::AniwatchSeries {
                        id: s.to_string(),
                        estimate_release_time: true,
                    },
                )));
            }

            while let Some(res) = set.join_next().await {
                if let Err(e) = &res {
                    debug!("Error joining: {:?}", e);
                }
                let res = res.unwrap();

                match res {
                    Ok(info) => {
                        debug!("Got info for {}", &info.name);
                        results.push(info);
                    }
                    Err(e) => {
                        debug!("Error getting info: {:?}", e);
                    }
                }
            }
        });

        if true {
            let res_json = serde_json::to_string_pretty(&results).unwrap();
            println!("{}", &res_json);
            let mut f = File::create("./dev/res.json").unwrap();
            f.write_all(res_json.as_bytes()).unwrap();
        } else {
            runtime.block_on(async {
                let series_info = metadata::series_info(metadata::MetaSeriesInfo::Aniwatch(
                    metadata::aniwatch::AniwatchSeries {
                        id: "my-star-18330".to_string(),
                        estimate_release_time: true,
                    },
                ))
                .await;

                dbg!(series_info).unwrap();
            });
        }
    }
}
