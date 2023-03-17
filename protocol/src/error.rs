use std::io::Error;
use std::net::SocketAddr;
use tokio::sync::mpsc::error::SendError;
use crate::packet::SerializedPacket;

#[derive(Debug, thiserror::Error)]
pub enum NetheriteError {
    #[error("IO error: `{0}`")]
    IOErr(#[from] Error),
    #[error("Packet send error: `{0}`")]
    PacketSendErr(#[from] SendError<SerializedPacket>),
    #[error("Double login - player with your address is already logged in!")]
    DoubleLogin,
    #[error("Player with address {0} was not found")]
    PlayerNotFound(SocketAddr)
}