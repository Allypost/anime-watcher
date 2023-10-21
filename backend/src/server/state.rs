use crate::db::AppDb;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: AppDb,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let db = AppDb::new().await?;

        Ok(Self { db })
    }
}
