use std::sync::atomic::Ordering;

use anyhow::Result;
use async_std::{
    net::TcpListener,
    sync::{Arc, Mutex, RwLock},
    task::{self, JoinHandle},
};
use epicenter::{AsyncDispatcher, Event};
use log::{debug, error, warn};

use super::{Client, ClientDisconnect, Config};

pub struct PacketEvent<T> {
    pub packet_arc: Arc<T>,
    pub client_arc: Arc<Client>,
}

impl<T> PacketEvent<T> {
    pub fn new(packet: T, client_arc: Arc<Client>) -> Self {
        Self {
            packet_arc: Arc::new(packet),
            client_arc,
        }
    }
}

impl<T> Clone for PacketEvent<T> {
    fn clone(&self) -> Self {
        Self {
            packet_arc: self.packet_arc.clone(),
            client_arc: self.client_arc.clone(),
        }
    }
}

impl<T> Event for PacketEvent<T> {}

pub struct Server {
    config_arc: Arc<Config>,
    socket_addr: String,
    client_vec_mutex: Arc<Mutex<Vec<Arc<Client>>>>,
    handle_option: Option<JoinHandle<()>>,
    pub dispatcher_status_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub dispatcher_login_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub dispatcher_configuration_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub dispatcher_play_rwlock: Arc<RwLock<AsyncDispatcher>>,
}

impl Server {
    pub async fn new(socket_addr: String, config: Config) -> Result<Self> {
        Ok(Self {
            config_arc: Arc::new(config),
            socket_addr,
            client_vec_mutex: Arc::new(Mutex::new(Vec::new())),
            handle_option: None,
            dispatcher_status_rwlock: Arc::new(RwLock::new(AsyncDispatcher::new())),
            dispatcher_login_rwlock: Arc::new(RwLock::new(AsyncDispatcher::new())),
            dispatcher_configuration_rwlock: Arc::new(RwLock::new(AsyncDispatcher::new())),
            dispatcher_play_rwlock: Arc::new(RwLock::new(AsyncDispatcher::new())),
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        let config_arc = self.config_arc.clone();
        let socket_addr = self.socket_addr.clone();
        let client_vec_mutex = self.client_vec_mutex.clone();
        let dispatcher_status_rwlock = self.dispatcher_status_rwlock.clone();
        let dispatcher_login_rwlock = self.dispatcher_login_rwlock.clone();
        let dispatcher_configuration_rwlock = self.dispatcher_configuration_rwlock.clone();
        let dispatcher_play_rwlock = self.dispatcher_play_rwlock.clone();
        let listener = TcpListener::bind(socket_addr).await?;

        self.handle_option = Some(task::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, socket_addr)) => {
                        client_vec_mutex.lock().await.push(Client::new(
                            stream,
                            socket_addr,
                            config_arc.clone(),
                            dispatcher_status_rwlock.clone(),
                            dispatcher_login_rwlock.clone(),
                            dispatcher_configuration_rwlock.clone(),
                            dispatcher_play_rwlock.clone(),
                        ));

                        warn!("{socket_addr} : New connection");
                    }
                    Err(error) => error!("Error accepting a new connection : {error}"),
                }

                {
                    let mut client_vec = client_vec_mutex.lock().await;
                    let mut index = 0;

                    while let Some(client) = client_vec.get(index) {
                        if !client.running_atomic.load(Ordering::Relaxed) {
                            client_vec.remove(index).disconnect().await;
                            continue;
                        }

                        index += 1;
                    }

                    debug!("Client connection count : {}", client_vec.len());
                }
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
