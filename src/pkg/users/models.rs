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
