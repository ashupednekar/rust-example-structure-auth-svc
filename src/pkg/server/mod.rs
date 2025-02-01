pub mod handlers;
mod router;

use tokio::net::TcpListener;
use crate::{conf::settings, pkg::server::router::build_routes, prelude::Result};



pub async fn listen() -> Result<()>{
    let listener = TcpListener::bind(
        &format!("0.0.0.0:{}", &settings.listen_port)
    ).await.unwrap();
       tracing::info!("listening at: {}", &settings.listen_port);
    axum::serve(listener, build_routes())
        .await?;
    Ok(())
}
