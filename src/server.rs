use std::io::{self, ErrorKind};
use std::net::TcpListener;

use log::{debug, error};

use crate::client::Client;

pub struct Server {
    listener: TcpListener,
    client_vec: Vec<Client>,
}

impl Server {
    pub fn new(address: &'static str) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;

        listener.set_nonblocking(true)?;

        Ok(Self {
            listener,
            client_vec: vec![],
        })
    }

    pub fn update(&mut self) -> io::Result<()> {
        match self.listener.accept() {
            Ok((socket, socket_addr)) => {
                debug!("New tcp client: {socket_addr:?}");

                self.client_vec
                    .push(match Client::new(socket, socket_addr) {
                        Ok(v) => v,
                        Err(e) => {
                            error!("Cannot create tcp client: {e:?}");
                            return Ok(());
                        }
                    })
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => error!("Couldn't get tcp client: {e:?}"),
        }

        let mut client_to_disconnect = vec![];

        for client in self.client_vec.iter_mut() {
            if let Err(e) = client.update() {
                error!("{:?}", e);

                client_to_disconnect.push(client.socket_addr);

                if let Err(e) = client.disconnect() {
                    error!("{:?}", e)
                }

                debug!("Disconnecting tcp client: {:?}", client.socket_addr);
            }
        }

        self.client_vec
            .retain(|client: &Client| !client_to_disconnect.contains(&client.socket_addr));

        Ok(())
    }
}
