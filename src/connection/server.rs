use std::net::SocketAddr;

use anyhow::Result;
use async_std::{
    channel::{unbounded, Receiver, Sender},
    net::TcpListener,
    sync::{Arc, Mutex},
    task::{self, JoinHandle},
};
use log::{error, warn};
use stopper::Stopper;

use super::{Client, ClientStop, Config, EventDispatcher};

pub struct Server {
    config_arc: Arc<Config>,
    socket_addr: String,
    client_vec_mutex: Arc<Mutex<Vec<Arc<Client>>>>,
    disconnect_channel: (Sender<SocketAddr>, Receiver<SocketAddr>),
    disconnect_handle_option: Option<(Stopper, JoinHandle<()>)>,
    accept_handle_option: Option<(Stopper, JoinHandle<()>)>,
    pub dispatcher: EventDispatcher,
}

impl Server {
    pub async fn new(socket_addr: String, config: Config) -> Result<Self> {
        let client_vec_mutex: Arc<Mutex<Vec<Arc<Client>>>> = Arc::new(Mutex::new(Vec::new()));
        let client_disconnect_channel = unbounded();
        let stopper = Stopper::new();

        Ok(Self {
            disconnect_handle_option: Some((
                stopper.clone(),
                task::spawn({
                    let client_vec_mutex = client_vec_mutex.clone();
                    let client_disconnect_receiver = client_disconnect_channel.1.clone();

                    async move {
                        while let Some(Ok(socket_addr)) =
                            stopper.stop_future(client_disconnect_receiver.recv()).await
                        {
                            let mut client_vec = client_vec_mutex.lock().await;

                            if let Some(index) = client_vec
                                .iter()
                                .position(|client| client.socket_addr == socket_addr)
                            {
                                client_vec.remove(index).stop().await;
                            }
                        }
                    }
                }),
            )),
            config_arc: Arc::new(config),
            socket_addr,
            client_vec_mutex,
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
                let config_arc = self.config_arc.clone();
                let socket_addr = self.socket_addr.clone();
                let client_vec_mutex = self.client_vec_mutex.clone();
                let client_disconnect_sender = self.disconnect_channel.0.clone();
                let dispatcher = self.dispatcher.clone();
                let listener = TcpListener::bind(socket_addr).await?;

                async move {
                    while let Some(connection) = stopper.stop_future(listener.accept()).await {
                        match connection {
                            Ok((stream, socket_addr)) => {
                                client_vec_mutex.lock().await.push(Client::new(
                                    stream,
                                    socket_addr,
                                    config_arc.clone(),
                                    client_disconnect_sender.clone(),
                                    dispatcher.clone(),
                                ));

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
        for client in self.client_vec_mutex.lock().await.drain(..) {
            client.stop().await;
        }
    }
}

impl Drop for Server {
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
