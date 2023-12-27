use packet::{packet_enum, DeserializePacket};

packet_enum! { ClientHandshake
    #[derive(Debug, DeserializePacket)]
    #[id(0x00)]
    pub struct Handshake {
        #[variable]
        pub protocol_version: i32,
        pub address: String,
        pub port: u16,
        #[variable]
        pub next_state: i32,
    }
}
