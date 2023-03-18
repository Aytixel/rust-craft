use std::rc::Rc;

use boring::rsa::Rsa;

use crate::{
    data_type::{Packet, ToString, ToVarInt},
    packet::ClientLoginPacketId,
    server::EncryptionData,
};

#[derive(Debug)]
pub struct EncryptionPacket;

impl EncryptionPacket {
    pub fn new(encryption_data: Rc<EncryptionData>) -> Packet {
        let mut data = "".to_packet_string();

        //data.append(&mut (encryption_data.der_public_key.as_bytes().len() as i32).to_varint());
        //data.append(&mut encryption_data.der_public_key.as_bytes().to_vec());

        let rsa = Rsa::generate(1024).unwrap();
        let mut der = rsa.public_key_to_der().unwrap();

        data.append(&mut (der.len() as i32).to_varint());
        data.append(&mut der);

        data.append(&mut (encryption_data.token.len() as i32).to_varint());
        data.append(&mut encryption_data.token.clone());

        Packet {
            id: ClientLoginPacketId::Encryption as i32,
            data,
        }
    }
}
