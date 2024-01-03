use std::{collections::HashMap, net::SocketAddr};

use anyhow::Result;
use async_std::{
    channel::{unbounded, Receiver, Sender},
    net::TcpListener,
    sync::{Arc, Mutex},
    task::{self, JoinHandle},
};
use log::{error, warn};
use stopper::Stopper;

use super::{Client, ClientStop, Config, EventDispatcher, RsaEncryptor};

pub struct Server<T: Send + Sync + 'static> {
    config: Arc<Config>,
    encryptor: Arc<RsaEncryptor>,
    socket_addr: String,
    client_hashmap_mutex: Arc<Mutex<HashMap<SocketAddr, Arc<Client<T>>>>>,
    disconnect_channel: (Sender<SocketAddr>, Receiver<SocketAddr>),
    disconnect_handle_option: Option<(Stopper, JoinHandle<()>)>,
    accept_handle_option: Option<(Stopper, JoinHandle<()>)>,
    pub dispatcher: EventDispatcher,
}

impl<T: Send + Sync + 'static> Server<T> {
    pub async fn new(socket_addr: String, config: Config, encryptor: RsaEncryptor) -> Result<Self> {
        let client_hashmap_mutex: Arc<Mutex<HashMap<SocketAddr, Arc<Client<T>>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let client_disconnect_channel = unbounded();
        let stopper = Stopper::new();

        Ok(Self {
            disconnect_handle_option: Some((
                stopper.clone(),
                task::spawn({
                    let client_hashmap_mutex = client_hashmap_mutex.clone();
                    let client_disconnect_receiver = client_disconnect_channel.1.clone();

                    async move {
                        while let Some(Ok(socket_addr)) =
                            stopper.stop_future(client_disconnect_receiver.recv()).await
                        {
                            if let Some(client) =
                                client_hashmap_mutex.lock().await.remove(&socket_addr)
                            {
                                client.stop().await;
                            }
                        }
                    }
                }),
            )),
            config: Arc::new(config),
            encryptor: Arc::new(encryptor),
            socket_addr,
            client_hashmap_mutex,
            disconnect_channel: client_disconnect_channel,
            accept_handle_option: None,
            dispatcher: EventDispatcher::default(),
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        let stopper = Stopper::new();

        self.accept_handle_option = Some((
            stopper.clone(),
            task::spawn({
                let config = self.config.clone();
                let encryptor = self.encryptor.clone();
                let socket_addr = self.socket_addr.clone();
                let client_hashmap_mutex = self.client_hashmap_mutex.clone();
                let client_disconnect_sender = self.disconnect_channel.0.clone();
                let dispatcher = self.dispatcher.clone();
                let listener = TcpListener::bind(socket_addr).await?;

                async move {
                    while let Some(connection) = stopper.stop_future(listener.accept()).await {
                        match connection {
                            Ok((stream, socket_addr)) => {
                                client_hashmap_mutex.lock().await.insert(
                                    socket_addr,
                                    Client::new(
                                        stream,
                                        socket_addr,
                                        config.clone(),
                                        encryptor.clone(),
                                        client_disconnect_sender.clone(),
                                        dispatcher.clone(),
                                    ),
                                );

                                warn!("{socket_addr} : New connection");
                            }
                            Err(error) => error!("Error accepting a new connection : {error}"),
                        }
                    }
                }
            }),
        ));

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(handle) = self.accept_handle_option.take() {
            handle.0.stop();
            handle.1.await;
        }
    }

    pub async fn disconnect(&mut self) {
        for (_, client) in self.client_hashmap_mutex.lock().await.drain() {
            client.stop().await;
        }
    }
}

impl<T: Send + Sync + 'static> Drop for Server<T> {
    fn drop(&mut self) {
        if let Some(disconnect_handle) = self.disconnect_handle_option.take() {
            task::block_on(async move {
                disconnect_handle.0.stop();
                disconnect_handle.1.await;
            });
        }

        task::block_on(async move {
            self.stop().await;
            self.disconnect().await;
        });
    }
}
