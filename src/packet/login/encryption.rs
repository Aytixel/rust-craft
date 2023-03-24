use std::rc::Rc;

use boring::rsa::Padding;
use log::debug;
use serde_json::json;

use crate::client::{Client, ClientState};
use crate::data_type::{FromByte, FromVarInt, Packet, ToString, ToVarInt};
use crate::packet::login::{DisconnectPacket, LoginSuccessPacket, SetCompressionPacket};
use crate::packet::ClientLoginPacketId;
use crate::server::EncryptionData;

#[derive(Debug)]
pub struct EncryptionPacket {
    shared_secret: Vec<u8>,
    verify_token: Vec<u8>,
}

impl EncryptionPacket {
    pub fn new(encryption_data: Rc<EncryptionData>) -> Packet {
        let mut data = "".to_packet_string();

        data.append(&mut (encryption_data.der_public_key.len() as i32).to_varint());
        data.append(&mut encryption_data.der_public_key.clone());
        data.append(&mut (encryption_data.verify_token.len() as i32).to_varint());
        data.append(&mut encryption_data.verify_token.clone());

        Packet {
            id: ClientLoginPacketId::Encryption as i32,
            data,
        }
    }

    pub fn handle(client: &mut Client, packet: &Packet) -> Result<(), String> {
        let encryption_packet =
            EncryptionPacket::try_from(packet.clone(), client.encryption_data.clone())?;

        debug!("{:#?}", encryption_packet);

        if encryption_packet.verify_token != client.encryption_data.verify_token {
            client
                .send_packet(DisconnectPacket::new(
                    json!({"text": "The verify token is not correct"}).to_string(),
                ))
                .map_err(|_| "The verify token is not correct. Error sending DisconnectPacket")?;

            return Err("The verify token is not correct".to_string());
        }

        client.state = ClientState::Play;
        client
            .send_packet(SetCompressionPacket::new())
            .map_err(|_| "Error sending SetCompressionPacket")?;
        client.compressed = true;
        client
            .send_packet(LoginSuccessPacket::new(
                client.player_data.uuid,
                client.player_data.username.clone(),
            ))
            .map_err(|_| "Error sending LoginSuccessPacket")?;

        Ok(())
    }

    fn try_from(mut packet: Packet, encryption_data: Rc<EncryptionData>) -> Result<Self, String> {
        let mut shared_secret_length = packet.data.from_varint()? as usize;
        let mut shared_secret = vec![0; encryption_data.rsa.size() as usize];

        shared_secret_length = encryption_data
            .rsa
            .private_decrypt(
                &packet.data.from_byte_array(shared_secret_length)?,
                &mut shared_secret,
                Padding::PKCS1,
            )
            .map_err(|_| "Shared secret, decryption not possible")?;

        let mut verify_token_length = packet.data.from_varint()? as usize;
        let mut verify_token = vec![0; encryption_data.rsa.size() as usize];

        verify_token_length = encryption_data
            .rsa
            .private_decrypt(
                &packet.data.from_byte_array(verify_token_length)?,
                &mut verify_token,
                Padding::PKCS1,
            )
            .map_err(|_| "Verify token, decryption not possible")?;

        Ok(EncryptionPacket {
            shared_secret: shared_secret[..shared_secret_length].to_vec(),
            verify_token: verify_token[..verify_token_length].to_vec(),
        })
    }
}
