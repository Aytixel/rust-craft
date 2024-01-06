use packet::{packet_enum, DeserializePacket};
use uuid::Uuid;

packet_enum! { ClientConfiguration
    #[derive(Debug, DeserializePacket)]
    #[id(0x00)]
    pub struct ClientInformation {
        pub locale: String,
        pub view_distance: i8,
        #[variable]
        pub chat_mode: i32,
        pub chat_colors: bool,
        pub displayed_skin_parts: u8,
        #[variable]
        pub main_hand: i32,
        pub enable_text_filtering: bool,
        pub allow_server_listing: bool,
    }

    #[derive(Debug, DeserializePacket)]
    #[id(0x01)]
    pub struct PluginMessage {
        pub channel: String,
        pub data: Vec<u8>,
    }

    #[derive(Debug, DeserializePacket)]
    #[id(0x02)]
    pub struct FinishConfiguration {}

    #[derive(Debug, DeserializePacket)]
    #[id(0x03)]
    pub struct KeepAlive {
        pub keep_alive_id: i64,
    }

    #[derive(Debug, DeserializePacket)]
    #[id(0x04)]
    pub struct Pong {
        pub id: i32,
    }

    #[derive(Debug, DeserializePacket)]
    #[id(0x05)]
    pub struct ResourcePackResponse {
        pub uuid: Uuid,
        #[variable]
        pub result: i32,
    }
}
