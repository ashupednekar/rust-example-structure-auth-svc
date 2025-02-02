use crate::prelude::Result;
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::{prelude::FromRow, PgPool};

#[derive(Debug, Deserialize, FromRow)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
    pub display_pic: Option<String>,
    pub verified: bool,
}

impl User {
    pub async fn save(&self, pool: &PgPool) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users (username, email, password, display_pic, verified) 
            VALUES ($1, $2, $3, $4, $5)
            "#,
            self.username,
            self.email,
            self.password,
            self.display_pic,
            self.verified
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Session {
    pub username: String,
    pub token: String,
    pub create_dt: NaiveDateTime,
}
