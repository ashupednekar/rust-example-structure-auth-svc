use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{pkg::middlewares::auth::auth_middleware, state::AppState};

use super::handlers::probes::livez;



pub fn build_routes() -> Router{
    let state = AppState::new();
    Router::new()
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .route("/livez/", get(livez))
        .with_state(state)
}
