use async_std::sync::{Arc, RwLock};
use epicenter::{AsyncDispatcher, Event};

use super::Client;

pub struct PacketEvent<T, U: Send + Sync + 'static> {
    pub packet: Arc<T>,
    pub client: Arc<Client<U>>,
}

impl<T, U: Send + Sync + 'static> PacketEvent<T, U> {
    pub fn new(packet: T, client: Arc<Client<U>>) -> Self {
        Self {
            packet: Arc::new(packet),
            client,
        }
    }
}

impl<T, U: Send + Sync + 'static> Clone for PacketEvent<T, U> {
    fn clone(&self) -> Self {
        Self {
            packet: self.packet.clone(),
            client: self.client.clone(),
        }
    }
}

impl<T, U: Send + Sync + 'static> Event for PacketEvent<T, U> {}

pub type EventDispatcher = Arc<RwLock<AsyncDispatcher>>;

#[derive(Clone, Default)]
pub struct EventDispatcherList {
    pub status: EventDispatcher,
    pub login: EventDispatcher,
    pub configuration: EventDispatcher,
    pub play: EventDispatcher,
}
