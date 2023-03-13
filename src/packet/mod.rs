use num_derive::FromPrimitive;

pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

#[derive(FromPrimitive)]
pub enum HandshakePacketId {
    Handshake,
}

#[derive(FromPrimitive)]
pub enum StatusPacketId {
    Status,
    Ping,
}

#[derive(FromPrimitive)]
pub enum LoginPacketId {}

#[derive(FromPrimitive)]
pub enum PlayPacketId {}
