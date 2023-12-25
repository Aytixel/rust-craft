pub mod connection;
pub mod packet;
pub mod version;

use anyhow::Result;
use async_ctrlc::CtrlC;
use log::info;

use crate::{
    connection::{Config, Server},
    version::Version,
};

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let version = Version::new().await?;
    let server_config = Config::default();
    let mut server = Server::new("0.0.0.0:25565".to_string(), server_config).await?;

    info!("{:#?}", version);

    server.start().await?;

    CtrlC::new()?.await;

    server.stop().await;
    server.disconnect().await;

    info!("Server shutdown");

    Ok(())
}
