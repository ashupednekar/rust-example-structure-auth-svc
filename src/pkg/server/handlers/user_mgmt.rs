use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;
use standard_error::{StandardError, Status};

use crate::{
    pkg::users::{adaptors::register::RegisterActions, models::User},
    prelude::Result,
    state::AppState,
};

#[derive(Deserialize)]
pub struct RegistrationInput {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub async fn initiate_registration(
    Json(payload): Json<RegistrationInput>,
    State(state): State<AppState>,
) -> Result<String> {
    if payload.password != payload.confirm_password {
        return Err(StandardError::new("ERR-AUTH-001").code(StatusCode::BAD_REQUEST));
    }
    let user = User {
        username: payload.username,
        email: payload.email,
        password: payload.password,
        display_pic: None,
        verified: false,
    };
    user.save(&state.db_pool).await?;
    user.initiate_registration().await?;
    Ok(serde_json::to_string(&json!({ "msg": "success" }))?)
}
