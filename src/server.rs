use std::io::ErrorKind;
use std::net::TcpListener;
use std::rc::Rc;

use crate::{client::Client, version_info::VersionInfo};
use boring::pkey::Private;
use boring::rsa::Rsa;
use log::{debug, error};
use rand::{thread_rng, Rng};

pub struct EncryptionData {
    pub rsa: Rsa<Private>,
    pub der_public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl EncryptionData {
    fn new() -> Self {
        let rsa = Rsa::generate(1024).expect("Failed to generate a private key");
        let der_public_key = rsa
            .public_key_to_der()
            .expect("Failed to convert public key to the right format");

        Self {
            rsa,
            der_public_key,
            verify_token: thread_rng().gen::<[u8; 4]>().into(),
        }
    }
}

pub struct Server {
    listener: TcpListener,
    client_vec: Vec<Client>,
    encryption_data: Rc<EncryptionData>,
    version_info: Rc<VersionInfo>,
}

impl Server {
    pub fn new(address: &'static str, version_info: Rc<VersionInfo>) -> Result<Self, String> {
        let listener = TcpListener::bind(address).map_err(|e| e.to_string())?;

        listener.set_nonblocking(true).map_err(|e| e.to_string())?;

        Ok(Self {
            listener,
            client_vec: vec![],
            encryption_data: Rc::new(EncryptionData::new()),
            version_info,
        })
    }

    pub fn update(&mut self) -> Result<(), String> {
        match self.listener.accept() {
            Ok((socket, socket_addr)) => {
                debug!("New tcp client: {socket_addr}");

                self.client_vec.push(
                    match Client::new(
                        socket,
                        socket_addr,
                        self.encryption_data.clone(),
                        self.version_info.clone(),
                    ) {
                        Ok(v) => v,
                        Err(e) => {
                            error!("Cannot create tcp client: {e}");
                            return Ok(());
                        }
                    },
                )
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => error!("Couldn't get tcp client: {e}"),
        }

        let mut client_to_disconnect = vec![];

        for client in self.client_vec.iter_mut() {
            if let Err(e) = client.update() {
                error!("{e}");

                client_to_disconnect.push(client.socket_addr);

                if let Err(e) = client.disconnect() {
                    error!("{e}")
                }

                debug!("Disconnecting tcp client: {}", client.socket_addr);
            }
        }

        self.client_vec
            .retain(|client: &Client| !client_to_disconnect.contains(&client.socket_addr));

        Ok(())
    }
}
