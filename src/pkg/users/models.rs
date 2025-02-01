use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::prelude::FromRow;


#[derive(Debug, Deserialize, FromRow)]
pub struct User{
    pub email: String,
    pub username: String,
    pub password: String,
    pub secret_question: String,
    pub secret_answer: String,
    pub display_pic: String
}


#[derive(Debug, Deserialize, FromRow)]
pub struct Session {
    pub username: String,
    pub token: String,
    pub create_dt: NaiveDateTime,  
}


