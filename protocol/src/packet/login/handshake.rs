use netherite_common::macros::Packet;
use crate::packet::{Packet, State};
use crate::server::{Server, ServerPlayerNet};
use crate::types::VarInt;

#[derive(Packet)]
#[stdto::bytes]
#[packet(serverbound 0x00 in State::Handshaking)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    next_state: VarInt,
}

impl Handshake {
    pub fn next_state(&self) -> State {
        State::try_from(self.next_state.value()).unwrap_or(State::Login)
    }
}
