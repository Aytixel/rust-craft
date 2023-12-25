use packet::DeserializePacket;

#[derive(Debug, DeserializePacket)]
pub struct Handshake {
    #[variable]
    protocol_version: i32,
    address: String,
    port: u16,
    #[variable]
    next_state: i32,
}
