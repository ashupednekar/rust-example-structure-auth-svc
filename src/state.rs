use std::sync::Arc;

use sqlx::PgPool;

use crate::{prelude::Result, conf::settings};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: Arc<PgPool>
}

impl AppState {
    pub async fn new() -> Result<AppState> {
        let db_pool = Arc::new(PgPool::connect(&settings.database_url).await?);
        Ok(AppState {
            db_pool
        })
    }
}
