use packet::{packet_enum, DeserializePacket};

packet_enum! { ClientHandshake

    #[derive(Debug, DeserializePacket)]
    #[id(0)]
    pub struct Handshake {
        #[variable]
        protocol_version: i32,
        address: String,
        port: u16,
        #[variable]
        next_state: i32,
    }
}
