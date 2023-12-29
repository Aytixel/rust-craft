use std::sync::Arc;

use async_std::sync::RwLock;
use epicenter::AsyncDispatcher;
use serde_json::json;

use crate::{
    connection::PacketEvent,
    packet::{
        client::status::{PingRequest, StatusRequest},
        server::status::{PingResponse, StatusResponse},
        ClientStatus, ServerPacket, ServerStatus,
    },
};

pub struct StatusLogic {}

impl StatusLogic {
    pub async fn init(dispatcher_status_rwlock: Arc<RwLock<AsyncDispatcher>>) {
        dispatcher_status_rwlock
            .write()
            .await
            .listen(Self::handler)
            .await;
    }

    async fn handler(
        PacketEvent {
            packet_arc,
            client_arc,
        }: PacketEvent<ClientStatus>,
    ) {
        match *packet_arc.as_ref() {
            ClientStatus::StatusRequest(StatusRequest {}) => {
                client_arc
                    .send_packet(ServerPacket::from(ServerStatus::StatusResponse(
                        StatusResponse {
                            json_response: json!({
                                "version": {
                                    "name": client_arc.config_arc.version.name,
                                    "protocol": client_arc.config_arc.version.protocol,
                                },
                                "players": {
                                    "max": client_arc.config_arc.max_player.unwrap_or_default(),
                                    "online": 0,
                                    "sample": [],
                                },
                                "description": {
                                    "text": "Powered by RustCraft server",
                                },
                                "favicon": "",
                                "enforcesSecureChat": true,
                                "previewsChat": true,
                            })
                            .to_string(),
                        },
                    )))
                    .await;
            }
            ClientStatus::PingRequest(PingRequest { payload }) => {
                client_arc
                    .send_packet(ServerPacket::from(ServerStatus::PingResponse(
                        PingResponse { payload },
                    )))
                    .await;
            }
        }
    }
}
