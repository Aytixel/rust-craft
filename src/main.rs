use std::{rc::Rc, thread::sleep, time::Duration};

use log::info;
use server::Server;

use crate::{datapack::Datapack, version_info::VersionInfo};

mod client;
mod data_type;
mod datapack;
mod packet;
mod server;
mod version_info;

fn main() -> Result<(), String> {
    env_logger::init();

    let version_info = Rc::new(VersionInfo::new()?);
    let datapack = Rc::new(Datapack::new("./")?);
    let mut server = Server::new("0.0.0.0:25565", version_info, datapack)?;

    info!("Server listening on: 0.0.0.0:25565");

    loop {
        server.update()?;

        sleep(Duration::from_millis(10));
    }
}
