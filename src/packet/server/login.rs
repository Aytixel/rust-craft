use packet::{packet_enum, SerializePacket};
use uuid::Uuid;

use crate::packet::r#struct::Property;

packet_enum! { ServerLogin
    #[derive(Debug, SerializePacket)]
    #[id(0x00)]
    pub struct Disconnect {
        pub reason: String
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x01)]
    pub struct EncryptionRequest {
        pub server_id: String,
        #[variable]
        pub public_key_length: i32,
        #[array(public_key_length)]
        pub public_key: Vec<u8>,
        #[variable]
        pub verify_key_length: i32,
        #[array(verify_key_length)]
        pub verify_key: Vec<u8>,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x02)]
    pub struct LoginSuccess {
        pub uuid: Uuid,
        pub username: String,
        #[variable]
        pub number_of_properties: i32,
        #[array(number_of_properties)]
        pub property: Vec<Property>,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x03)]
    pub struct SetCompression {
        #[variable]
        pub threshold: i32,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x04)]
    pub struct LoginPluginRequest {
        #[variable]
        pub message_id: i32,
        pub channel: String,
        pub data: Vec<u8>,
    }
}
