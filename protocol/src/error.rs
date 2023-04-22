use crate::packet::{SerializedPacket, State};
use bytes::Bytes;
use std::io::Error;
use std::net::SocketAddr;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, thiserror::Error)]
pub enum NetheriteError {
    #[error("IO error: `{0}`")]
    IOErr(#[from] Error),
    #[error("Packet send error: `{0}`")]
    PacketSendErr(#[from] SendError<SerializedPacket>),
    #[error("Bytes error: `{0}`")]
    StdtoError(#[from] stdto::error::Error),
    #[error("Double login - player with your address is already logged in!")]
    DoubleLogin,
    #[error("Player with address {0} was not found")]
    PlayerNotFound(SocketAddr),
    #[error("Unsupported protocol version")]
    UnsupportedProto(i32),
    #[error("Unknown packet received (in state {state}): Packet(id = {id}, len = {len}, compressed = {compressed}, data = {data})")]
    UnknownPacket {
        id: i32,
        len: i32,
        data: Bytes,
        compressed: bool,
        state: State,
    },
}

impl NetheriteError {
    pub fn is_fatal(&self) -> bool {
        false
    }
}
