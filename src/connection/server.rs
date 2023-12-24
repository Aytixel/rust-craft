use anyhow::Result;
use async_std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    task::{self, JoinHandle},
};
use log::{debug, warn};
use slab::Slab;

use super::{Client, Config};

pub struct Server {
    config_arc: Arc<Config>,
    socket_addr: String,
    client_slab_mutex: Arc<Mutex<Slab<Client>>>,
    thread_handle_option: Option<JoinHandle<()>>,
}

impl Server {
    pub async fn new(socket_addr: String, config: Config) -> Result<Self> {
        Ok(Self {
            config_arc: Arc::new(config),
            socket_addr,
            client_slab_mutex: Arc::new(Mutex::new(Slab::new())),
            thread_handle_option: None,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        let socket_addr = self.socket_addr.clone();
        let client_slab_rwlock = self.client_slab_mutex.clone();
        let config_arc = self.config_arc.clone();
        let listener = TcpListener::bind(socket_addr).await?;

        self.thread_handle_option = Some(task::spawn(async move {
            loop {
                let config_arc = config_arc.clone();

                match listener.accept().await {
                    Ok((stream, socket_addr)) => {
                        client_slab_rwlock.lock().await.insert(Client::new(
                            stream,
                            socket_addr,
                            config_arc,
                        ));

                        warn!("Connection from : {}", socket_addr);
                    }
                    Err(error) => debug!("Error accepting a new connection : {}", error),
                }
            }
        }));

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(thread_handle) = self.thread_handle_option.take() {
            thread_handle.cancel().await;
        }
    }

    pub async fn disconnect(&mut self) {
        for client in self.client_slab_mutex.lock().await.drain() {
            client.disconnect().await;
        }
    }
}
