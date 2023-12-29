use packet::{packet_enum, DeserializePacket};
use uuid::Uuid;

packet_enum! { ClientLogin
    #[derive(Debug, DeserializePacket)]
    #[id(0x00)]
    pub struct LoginStart {
        pub name: String,
        pub player_uuid: Uuid,
    }

    #[derive(Debug, DeserializePacket)]
    #[id(0x01)]
    pub struct EncryptionResponse {
        #[variable]
        pub shared_secret_length: i32,
        #[variable]
        pub verify_token_length: i32
    }

    #[derive(Debug, DeserializePacket)]
    #[id(0x02)]
    pub struct LoginPluginResponse {
        #[variable]
        pub message_id: i32,
        pub successful: bool,
    }

    #[derive(Debug, DeserializePacket)]
    #[id(0x03)]
    pub struct LoginAcknowledged {}
}
