pub mod connection;
pub mod logic;
pub mod packet;
pub mod version;

use anyhow::Result;
use async_ctrlc::CtrlC;
use log::info;

use crate::{
    connection::{Config, Server},
    logic::StatusLogic,
    version::Version,
};

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let version = Version::new().await?;
    let config = Config::new(version);

    info!("{:#?}", config);

    let mut server = Server::new("0.0.0.0:25565".to_string(), config).await?;

    StatusLogic::init(server.dispatcher_status_rwlock.clone()).await;

    server.start().await?;

    CtrlC::new()?.await;

    server.stop().await;
    server.disconnect().await;

    info!("Server shutdown");

    Ok(())
}
