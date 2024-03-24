use async_std::{fs::File, io::ReadExt};
use serde::Serialize;
use serde_json::to_string;

use crate::{
    connection::{ EventDispatcher, PacketEvent},
    packet::{
        client::configuration::{
            ClientInformation, FinishConfiguration, KeepAlive, PluginMessage, Pong,
            ResourcePackResponse,
        }, server::configuration::Disconnect, ClientConfiguration, ServerConfiguration, ServerPacket
    }, r#struct::{RegistryCodec, TextComponent, TextStyle},
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
                        let mut file = File::open("./registry_data.json")
                            .await.unwrap();
                        let mut buffer = Vec::new();
                
                        file.read_to_end(&mut buffer).await.unwrap();
                
                        let registery: RegistryCodec = serde_json::from_slice(&buffer).unwrap();

                        println!("{:#?}", registery.registries);

                        client
                            .send_packet(ServerPacket::from(ServerConfiguration::Disconnect(
                                Disconnect {
                                    reason: TextComponent::from_text("test", TextStyle::new().bold(true)),
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
