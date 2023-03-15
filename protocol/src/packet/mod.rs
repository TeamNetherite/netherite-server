mod ser;
pub use ser::*;

use crate::server::Server;

pub trait Packet {
    const ID: i32;
}

pub trait S2CPacket: Packet {
    fn send(self, server: &mut Server);
}