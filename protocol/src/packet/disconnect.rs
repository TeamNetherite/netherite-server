use crate::model::chat::Chat;
use crate::packet::{Packet, S2CPacket};

#[stdto::bytes(endian = "big")]
pub struct Disconnect {
    reason: Chat,
}

impl Packet for Disconnect {
    const ID: i32 = 0x17;
}

impl S2CPacket for Disconnect {}
