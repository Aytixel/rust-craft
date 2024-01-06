use log::{error, warn};
use serde_json::json;

use crate::{
    connection::{EventDispatcher, PacketEvent},
    packet::{
        client::login::{EncryptionResponse, LoginStart},
        server::login::{Disconnect, EncryptionRequest, LoginSuccess, SetCompression},
        ClientLogin, ServerLogin, ServerPacket,
    },
    Data,
};

use super::Player;

pub struct LoginLogic {}

impl LoginLogic {
    pub async fn init(dispatcher_login: EventDispatcher) {
        dispatcher_login
            .write()
            .await
            .listen(
                |PacketEvent {
                     packet,
                     client,
                 }: PacketEvent<ClientLogin, Data>| async move {
                    match *packet.as_ref() {
                        ClientLogin::LoginStart(LoginStart {
                            ref name,
                            player_uuid,
                        }) => {
                            if client.wrong_protocol_version() {
                                warn!("{} : Wrong protocol version", client.socket_addr);

                                client
                                    .send_packet(ServerPacket::from(ServerLogin::Disconnect(
                                        Disconnect {
                                            reason: json!({
                                                "text": format!("§c§lWrong game version\n\n§fPlease retry with version : §a{}", client.config.version.name),
                                            })
                                            .to_string(),
                                        },
                                    )))
                                    .await;
                                client.disconnect().await;
                                return;
                            }

                            client
                                .set_data(Data::new(Player::new(name.clone(), player_uuid)))
                                .await;
                            client
                                .send_packet(ServerPacket::from(ServerLogin::EncryptionRequest(
                                    EncryptionRequest {
                                        server_id: String::new(),
                                        public_key_length: client.encryptor.public_key.len()
                                            as i32,
                                        public_key: client.encryptor.public_key.clone(),
                                        verify_key_length: client
                                            .encryptor
                                            .verify_token
                                            .len()
                                            as i32,
                                        verify_key: client.encryptor.verify_token.clone(),
                                    },
                                )))
                                .await;
                        }
                        ClientLogin::EncryptionResponse(EncryptionResponse {
                            ref shared_secret,
                            ref verify_token,
                            ..
                        }) => {
                            let mut decrypted_verify_token = Vec::new();

                            match client
                                .encryptor
                                .decrypt(&verify_token, &mut decrypted_verify_token)
                            {
                                Ok(length) => {
                                    if client.encryptor.verify_token
                                        == &decrypted_verify_token[..length]
                                    {
                                        if let Err(error) =
                                            client.enable_encryption(shared_secret).await
                                        {
                                            error!("{} : {}", client.socket_addr, error);

                                            client.disconnect().await;
                                        }

                                        client
                                            .send_packet(ServerPacket::from(
                                                ServerLogin::SetCompression(SetCompression {
                                                    threshold: client
                                                        .config
                                                        .compression_threshold
                                                        as i32,
                                                }),
                                            ))
                                            .await;
                                        client.enable_compression();

                                        let client_data_option = client.data().await;
                                        let client_data = client_data_option.as_ref().unwrap();

                                        client
                                            .send_packet(ServerPacket::from(
                                                ServerLogin::LoginSuccess(LoginSuccess {
                                                    uuid: client_data.player.uuid,
                                                    username: client_data.player.name.clone(),
                                                    number_of_properties: 0,
                                                    property: Vec::new(),
                                                }),
                                            ))
                                            .await;
                                    }
                                }
                                Err(error) => error!("{} : {}", client.socket_addr, error),
                            }
                        }
                        _ => {}
                    }
                },
            )
            .await;
    }
}
