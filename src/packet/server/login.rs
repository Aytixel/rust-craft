use packet::{packet_enum, SerializePacket};
use uuid::Uuid;

packet_enum! { ServerLogin
    #[derive(Debug, SerializePacket)]
    #[id(0x00)]
    pub struct Disconnect {
        reason: String
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x01)]
    pub struct EncryptionRequest {
        server_id: String,
        #[variable]
        public_key_length: i32,
        #[variable]
        verify_key_length: i32,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x02)]
    pub struct LoginSuccess {
        uuid: Uuid,
        username: String,
        #[variable]
        number_of_properties: i32,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x03)]
    pub struct SetCompression {
        #[variable]
        threshold: i32,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x04)]
    pub struct LoginPluginRequest {
        #[variable]
        message_id: i32,
        channel: String,
    }
}
