use std::time::Duration;

use log::{info, trace};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::CONFIG;

#[derive(Debug, Clone)]
pub struct AppDb {
    conn: DatabaseConnection,
}

impl AppDb {
    pub async fn new() -> anyhow::Result<Self> {
        let db_url = CONFIG
            .database
            .url
            .clone()
            .unwrap_or("sqlite::memory:".into());

        let mut opt = ConnectOptions::new(db_url);

        opt.connect_timeout(Duration::from_secs(2))
            .acquire_timeout(Duration::from_secs(2))
            .sqlx_logging(cfg!(debug_assertions))
            .sqlx_logging_level(log::LevelFilter::Trace);

        trace!("Connecting to database: {:?}", opt);

        let db = Database::connect(opt).await?;

        Ok(Self { conn: db })
    }

    pub fn connection(&self) -> DatabaseConnection {
        self.conn.clone()
    }

    pub async fn init(&self) -> anyhow::Result<()> {
        trace!("Initializing database");

        {
            info!("Running migrations");
            Migrator::up(&self.conn, None).await?;
        }

        Ok(())
    }
}
