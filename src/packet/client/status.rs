use packet::{packet_enum, DeserializePacket};

packet_enum! { ClientStatus
    #[derive(Debug, DeserializePacket)]
    #[id(0x00)]
    pub struct StatusRequest {}

    #[derive(Debug, DeserializePacket)]
    #[id(0x01)]
    pub struct PingRequest {
        payload: i64
    }
}
