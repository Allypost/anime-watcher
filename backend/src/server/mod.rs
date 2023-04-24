use log::info;
use rocket::Config;
use std::env;

mod routes;

#[rocket::main]
pub async fn run() -> Result<(), rocket::Error> {
    let port = env::var("PORT").unwrap_or("3001".to_string());
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    info!(
        "Starting server on {location}",
        location = format!("{host}:{port}")
    );

    let figment = Config::figment()
        .merge(("address", host))
        .merge(("port", port.parse::<u16>().unwrap()));

    rocket::custom(figment)
        .mount("/api", routes::build())
        .launch()
        .await?;

    Ok(())
}
