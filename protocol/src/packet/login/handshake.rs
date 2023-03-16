use crate::packet::{C2SPacket, Packet, State};
use crate::server::{Server, ServerPlayerNet};
use crate::types::VarInt;

#[stdto::bytes]
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

impl Packet for Handshake {
    const ID: i32 = 0x00;
}

impl C2SPacket for Handshake {
    async fn receive(server: &mut Server, client: &mut ServerPlayerNet) {
        if let Some(packet) = server.receive_packet::<Self>(client.addr()).await {
            
            client.state = packet.next_state();
        }
    }
}