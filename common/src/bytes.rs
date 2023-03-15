use bytes::Bytes;
use serde::ser::StdError;
use serde::Serializer;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct MinecraftBytesSerializer {
    bytes: Vec<u8>,
}

impl MinecraftBytesSerializer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

pub struct BytesError(pub String);

impl StdError for BytesError {}

impl Debug for BytesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Display for BytesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl serde::ser::Error for BytesError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        BytesError(msg.to_string())
    }
}

impl Serializer for MinecraftBytesSerializer {
    type Error = BytesError;
    type Ok = ();

    fn is_human_readable(&self) -> bool {
        false
    }

    fn serialize_bool(mut self, val: bool) -> Result<Self::Ok, Self::Error> {
        self.bytes.push(if val { 1 } else { 0 });

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.bytes;

        Ok(())
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        self.bytes.extend({
            let mut buf = [0u8; 2];
            v.encode_utf8(&mut buf);
            buf
        });

        Ok(())
    }

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.bytes.extend(v.to_be_bytes());

        Ok(())
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.bytes.extend(v.to_be_bytes());

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8((v + 128) as u8)
    }

    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.bytes.extend(((v + 32_768) as u16).to_be_bytes());

        Ok(())
    }
}
