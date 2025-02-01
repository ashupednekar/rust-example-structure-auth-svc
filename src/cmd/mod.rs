use crate::{pkg::server, prelude::Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about="lets you run auth-svc commands")]
struct Cmd{
    #[command(subcommand)]
    command: Option<SubCommandType>
}

#[derive(Subcommand)]
enum SubCommandType{
    Listen,
    Migrate,
}

pub async fn run() -> Result<()>{
    let args = Cmd::parse();
    match args.command{
        Some(SubCommandType::Listen) => {
            server::listen().await?; 
        },
        Some(SubCommandType::Migrate) => {
            //run diesel/sqlx migrations
        },
        None => {
            tracing::error!("no subcommand passed")
        }
    }
    Ok(())
}



