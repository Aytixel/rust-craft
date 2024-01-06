use serde_json::json;

use crate::{
    connection::{EventDispatcher, PacketEvent},
    packet::{
        client::status::{PingRequest, StatusRequest},
        server::status::{PingResponse, StatusResponse},
        ClientStatus, ServerPacket, ServerStatus,
    },
    Data,
};

pub struct StatusLogic {}

impl StatusLogic {
    pub async fn init(dispatcher_status: EventDispatcher) {
        dispatcher_status.write().await.listen(Self::handler).await;
    }

    async fn handler(PacketEvent { packet, client }: PacketEvent<ClientStatus, Data>) {
        match *packet.as_ref() {
            ClientStatus::StatusRequest(StatusRequest {}) => {
                client
                    .send_packet(ServerPacket::from(ServerStatus::StatusResponse(
                        StatusResponse {
                            json_response: json!({
                                "version": {
                                    "name": client.config.version.name,
                                    "protocol": client.config.version.protocol,
                                },
                                "players": {
                                    "max": client.config.max_player.unwrap_or_default(),
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
                client
                    .send_packet(ServerPacket::from(ServerStatus::PingResponse(
                        PingResponse { payload },
                    )))
                    .await;
            }
        }
    }
}
