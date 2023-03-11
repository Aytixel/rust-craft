use std::{io, thread::sleep, time::Duration};

use log::info;
use server::Server;

mod client;
mod data_type;
mod packet;
mod server;

fn main() -> io::Result<()> {
    env_logger::init();

    let mut server = Server::new("0.0.0.0:25565")?;

    info!("Server listening on: 0.0.0.0:25565");

    loop {
        server.update()?;

        sleep(Duration::from_millis(10));
    }
}
