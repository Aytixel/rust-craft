use packet::{packet_enum, SerializePacket};

packet_enum! { ServerStatus
    #[derive(Debug, SerializePacket)]
    #[id(0x00)]
    pub struct StatusResponse {
        pub json_response: String
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x01)]
    pub struct PingResponse {
        pub payload: i64
    }
}
