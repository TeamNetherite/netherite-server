use bytes::BytesMut;
use crate::packet::Packet;
use crate::types::VarInt;
use stdto::{AsBytes, ToBytes};
use tokio_util::codec::{Decoder, Encoder};

#[stdto::bytes(endian = "big")]
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

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn into_data(self) -> Vec<u8> {
        self.data
    }
}

pub struct PacketCodec;

impl Decoder for PacketCodec {
    type Item = SerializedPacket;
    type Error = stdto::error::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        <SerializedPacket as ToBytes>::try_from_bytes(src).map(|a| Some(a))
    }
}

impl Encoder<SerializedPacket> for PacketCodec {
    type Error = stdto::error::Error;

    fn encode(&mut self, item: SerializedPacket, dst: &mut BytesMut) -> Result<(), Self::Error> {
        <SerializedPacket as ToBytes>::try_to_bytes_into(&item, dst)
    }
}