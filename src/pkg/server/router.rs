use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{prelude::Result, pkg::middlewares::auth::auth_middleware, state::AppState};

use super::handlers::probes::livez;



pub async fn build_routes() -> Result<Router>{
    let state = AppState::new().await?;
    Ok(Router::new()
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .route("/livez/", get(livez))
        .with_state(state))
}
