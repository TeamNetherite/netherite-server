use crate::model::chat::Chat;
use crate::packet::{Packet, S2CPacket};
use crate::server::{Server, ServerPlayerNet};

#[stdto::bytes(endian = "big")]
pub struct Disconnect {
    reason: Chat
}

impl Packet for Disconnect {
    const ID: i32 = 0x17;
}

impl S2CPacket for Disconnect {
    async fn send(self, server: &mut Server, client: &mut ServerPlayerNet) {

    }
}