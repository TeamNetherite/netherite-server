mod ser;

pub use ser::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stdto::ToBytes;

pub mod disconnect;
pub mod login;
mod state;
pub mod status;

pub use state::*;

use crate::server::{Server, ServerPlayerNet};

pub enum PacketType {
    /// Client to Server packet
    Serverbound,
    /// Server to Client packet
    Clientbound
}

pub trait Packet {
    const ID: i32;
    const STATE: State;
    const TYPE: PacketType;
}