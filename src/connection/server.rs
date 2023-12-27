use std::sync::atomic::Ordering;

use anyhow::Result;
use async_std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    task::{self, JoinHandle},
};
use log::{debug, warn};

use super::{Client, Config};

pub struct Server {
    config_arc: Arc<Config>,
    socket_addr: String,
    client_vec_mutex: Arc<Mutex<Vec<Client>>>,
    handle_option: Option<JoinHandle<()>>,
}

impl Server {
    pub async fn new(socket_addr: String, config: Config) -> Result<Self> {
        Ok(Self {
            config_arc: Arc::new(config),
            socket_addr,
            client_vec_mutex: Arc::new(Mutex::new(Vec::new())),
            handle_option: None,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        let config_arc = self.config_arc.clone();
        let socket_addr = self.socket_addr.clone();
        let client_vec_mutex = self.client_vec_mutex.clone();
        let listener = TcpListener::bind(socket_addr).await?;

        self.handle_option = Some(task::spawn(async move {
            loop {
                let config_arc = config_arc.clone();

                match listener.accept().await {
                    Ok((stream, socket_addr)) => {
                        client_vec_mutex.lock().await.push(Client::new(
                            stream,
                            socket_addr,
                            config_arc,
                        ));

                        warn!("{socket_addr} : New connection");
                    }
                    Err(error) => debug!("Error accepting a new connection : {error}"),
                }

                let mut client_vec: Vec<Client> =
                    { client_vec_mutex.lock().await.drain(..).collect() };
                let mut index = 0;

                while let Some(client) = client_vec.get(index) {
                    if !client.running_atomic.load(Ordering::Relaxed) {
                        client_vec.remove(index).disconnect().await;
                        continue;
                    }

                    index += 1;
                }

                debug!("Client connection count : {}", client_vec.len());

                client_vec_mutex.lock().await.extend(client_vec);
            }
        }));

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(handle) = self.handle_option.take() {
            handle.cancel().await;
        }
    }

    pub async fn disconnect(&mut self) {
        for client in self.client_vec_mutex.lock().await.drain(..) {
            client.disconnect().await;
        }
    }
}
