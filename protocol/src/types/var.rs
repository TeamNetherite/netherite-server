use std::fmt::Formatter;
use std::intrinsics::rotate_right;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{DeserializeOwned, Error, Visitor};
use stdto::{AsBytes, ToBytes};

pub const SEGMENT_BITS: i32 = 0x7F;
pub const CONTINUE_BIT: i32 = 0x80;

pub struct VarInt(i32);

impl VarInt {
    pub fn new(value: i32) -> Self {
        VarInt(value)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

impl ToBytes for VarInt {
    fn try_from_bytes(bytes: impl AsBytes) -> Result<VarInt, stdto::error::Error> where Self: DeserializeOwned {
        let mut bytes = bytes.to_bytes().into_iter();

        let mut value = 0;
        let mut position = 0;
        let mut current_byte: i32;

        loop {
            current_byte = bytes.next().unwrap() as i32;
            value |= (current_byte & SEGMENT_BITS) << position;

            if (current_byte & CONTINUE_BIT) == 0 { break }

            position += 7;

            if position >= 32 { return Err(stdto::error::Error::OddLength) }
        }

        Ok(VarInt(value))
    }

    fn to_bytes(&self) -> Vec<u8> where Self: Serialize {
        let mut value = self.0;
        let mut bytes = vec![];

        loop {
            if (value & !SEGMENT_BITS) == 0 {
                bytes.push(value as u8);
                break;
            }

            bytes.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

            value = rotate_right(value, 7);
        }

        return bytes
    }
}

impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_bytes(self.to_bytes().as_byte_slice())
    }
}

impl<'a> Deserialize<'a> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        struct V;

        impl Visitor<'_> for V {
            type Value = VarInt;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "ok err")
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: Error {
                VarInt::try_from_bytes(v).map_err(|e| E::custom(e))
            }
        }

        deserializer.deserialize_byte_buf(
            V
        )
    }
}