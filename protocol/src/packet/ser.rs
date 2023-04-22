use std::fmt::{Display, Formatter};
use std::io::Write;
use bytes::BytesMut;
use crate::packet::Packet;
use crate::types::VarInt;
use stdto::{AsBytes, ToBytes};
use tokio_util::codec::{Decoder, Encoder};

#[stdto::bytes(endian = "big")]
#[derive(Debug)]
pub struct SerializedPacket {
    length: VarInt,
    packet_id: VarInt,
    data: Vec<u8>,
}

impl SerializedPacket {
    pub fn new<P: Packet>(data: Vec<u8>) -> Self {
        let pid = VarInt::new(P::ID);
        Self {
            length: VarInt::new(pid.to_bytes().len() as i32 + data.len() as i32),
            packet_id: pid,
            data,
        }
    }

    pub fn len(&self) -> i32 {
        self.length.value()
    }

    pub fn packet_id(&self) -> i32 {
        self.packet_id.value()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn into_data(self) -> Vec<u8> {
        self.data
    }
}

impl Display for SerializedPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet(id={}, len={}, data={:#?})", self.packet_id(), self.len(), self.data())
    }
}
