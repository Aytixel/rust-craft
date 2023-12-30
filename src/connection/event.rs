use async_std::sync::{Arc, RwLock};
use epicenter::{AsyncDispatcher, Event};

use super::Client;

pub struct PacketEvent<T, U: Send + Sync + 'static> {
    pub packet_arc: Arc<T>,
    pub client_arc: Arc<Client<U>>,
}

impl<T, U: Send + Sync + 'static> PacketEvent<T, U> {
    pub fn new(packet: T, client_arc: Arc<Client<U>>) -> Self {
        Self {
            packet_arc: Arc::new(packet),
            client_arc,
        }
    }
}

impl<T, U: Send + Sync + 'static> Clone for PacketEvent<T, U> {
    fn clone(&self) -> Self {
        Self {
            packet_arc: self.packet_arc.clone(),
            client_arc: self.client_arc.clone(),
        }
    }
}

impl<T, U: Send + Sync + 'static> Event for PacketEvent<T, U> {}

#[derive(Clone, Default)]
pub struct EventDispatcher {
    pub status_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub login_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub configuration_rwlock: Arc<RwLock<AsyncDispatcher>>,
    pub play_rwlock: Arc<RwLock<AsyncDispatcher>>,
}
