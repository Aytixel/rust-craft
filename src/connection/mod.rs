mod client;
mod config;
mod server;

use async_std::sync::{Arc, RwLock};
use epicenter::{AsyncDispatcher, Event};

pub use client::*;
pub use config::*;
pub use server::*;

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

#[derive(Clone, Default)]
pub struct EventDispatcher {
    pub status_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub login_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub configuration_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub play_rwlock: Arc<RwLock<AsyncDispatcher>>,
}
