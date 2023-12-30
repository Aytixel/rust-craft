use async_std::sync::{Arc, RwLock};
use epicenter::AsyncDispatcher;
use log::error;

use crate::{
    connection::PacketEvent,
    packet::{
        client::login::{EncryptionResponse, LoginAcknowledged, LoginPluginResponse, LoginStart},
        server::login::{EncryptionRequest, LoginSuccess, SetCompression},
        ClientLogin, ServerLogin, ServerPacket,
    },
    Data,
};

use super::Player;

pub struct LoginLogic {}

impl LoginLogic {
    pub async fn init(dispatcher_login_rwlock: Arc<RwLock<AsyncDispatcher>>) {
        dispatcher_login_rwlock
            .write()
            .await
            .listen(
                |PacketEvent {
                     packet_arc,
                     client_arc,
                 }: PacketEvent<ClientLogin, Data>| async move {
                    match *packet_arc.as_ref() {
                        ClientLogin::LoginStart(LoginStart {
                            ref name,
                            player_uuid,
                        }) => {
                            client_arc
                                .set_data(Data::new(Player::new(name.clone(), player_uuid)))
                                .await;

                            client_arc
                                .send_packet(ServerPacket::from(ServerLogin::EncryptionRequest(
                                    EncryptionRequest {
                                        server_id: String::new(),
                                        public_key_length: client_arc.encryptor_arc.public_key.len()
                                            as i32,
                                        public_key: client_arc.encryptor_arc.public_key.clone(),
                                        verify_key_length: client_arc
                                            .encryptor_arc
                                            .verify_token
                                            .len()
                                            as i32,
                                        verify_key: client_arc.encryptor_arc.verify_token.clone(),
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

                            match client_arc
                                .encryptor_arc
                                .decrypt(&verify_token, &mut decrypted_verify_token)
                            {
                                Ok(length) => {
                                    if client_arc.encryptor_arc.verify_token
                                        == &decrypted_verify_token[..length]
                                    {
                                        if let Err(error) =
                                            client_arc.enable_encryption(shared_secret).await
                                        {
                                            error!("{} : {}", client_arc.socket_addr, error);

                                            client_arc.disconnect().await;
                                        }

                                        client_arc
                                            .send_packet(ServerPacket::from(
                                                ServerLogin::SetCompression(SetCompression {
                                                    threshold: client_arc
                                                        .config_arc
                                                        .compression_threshold
                                                        as i32,
                                                }),
                                            ))
                                            .await;
                                        client_arc.enable_compression();

                                        let client_data_option = client_arc.data().await;
                                        let client_data = client_data_option.as_ref().unwrap();

                                        client_arc
                                            .send_packet(ServerPacket::from(
                                                ServerLogin::LoginSuccess(LoginSuccess {
                                                    uuid: client_data.player.uuid,
                                                    username: client_data.player.name.clone(),
                                                    number_of_properties: 0,
                                                }),
                                            ))
                                            .await;
                                    }
                                }
                                Err(error) => error!("{} : {}", client_arc.socket_addr, error),
                            }
                        }
                        ClientLogin::LoginPluginResponse(LoginPluginResponse {
                            message_id,
                            successful,
                            ref data,
                        }) => todo!(),
                        ClientLogin::LoginAcknowledged(LoginAcknowledged {}) => todo!(),
                    }
                },
            )
            .await;
    }
}
