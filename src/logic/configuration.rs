use mc_chat::{ChatComponent, ComponentStyle};

use crate::{
    connection::{ PacketEvent, EventDispatcher},
    packet::{
        client::configuration::{
            ClientInformation, FinishConfiguration, KeepAlive, PluginMessage, Pong,
            ResourcePackResponse,
        },
        ClientConfiguration, ServerConfiguration, ServerPacket, server::configuration::Disconnect,
    },
};

use super::Data;

pub struct ConfigurationLogic {}

impl ConfigurationLogic {
    pub async fn init(dispatcher_configuration: EventDispatcher) {
        dispatcher_configuration.write().await.listen(
            |PacketEvent { packet, client }: PacketEvent<ClientConfiguration, Data>| async move {
                match *packet.as_ref() {
                    ClientConfiguration::ClientInformation(ClientInformation {
                        ref locale,
                        view_distance,
                        chat_mode,
                        chat_colors, 
                        displayed_skin_parts,
                        main_hand,
                        enable_text_filtering,
                        allow_server_listing,
                    }) => {
                        client
                            .send_packet(ServerPacket::from(ServerConfiguration::Disconnect(
                                Disconnect {
                                    reason: ChatComponent::from_text("test", ComponentStyle::v1_16().bold(true)),
                                },
                            )))
                            .await;
                        client.disconnect().await;
                    },
                    ClientConfiguration::PluginMessage(PluginMessage {
                        ref channel,
                        ref data,
                    }) => {},
                    ClientConfiguration::FinishConfiguration(FinishConfiguration {}) => todo!(),
                    ClientConfiguration::KeepAlive(KeepAlive {
                        keep_alive_id,
                    }) => todo!(),
                    ClientConfiguration::Pong(Pong {
                        id,
                    }) => todo!(),
                    ClientConfiguration::ResourcePackResponse(ResourcePackResponse {
                        uuid,
                        result,
                    }) => todo!(),
                }
            },
        ).await;
    }
}
