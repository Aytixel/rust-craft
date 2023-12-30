pub mod connection;
pub mod logic;
pub mod packet;
pub mod version;

use anyhow::Result;
use async_ctrlc::CtrlC;
use log::info;

use crate::{
    connection::{Config, RsaEncryptor, Server},
    logic::{Data, LoginLogic, StatusLogic},
    version::Version,
};

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let version = Version::new().await?;
    let config = Config::new(version);
    let encryptor = RsaEncryptor::new()?;

    info!("{:#?}", config);

    let mut server = Server::<Data>::new("0.0.0.0:25565".to_string(), config, encryptor).await?;

    StatusLogic::init(server.dispatcher.status_rwlock.clone()).await;
    LoginLogic::init(server.dispatcher.login_rwlock.clone()).await;

    server.start().await?;

    CtrlC::new()?.await;

    info!("Server shutdown");

    Ok(())
}
