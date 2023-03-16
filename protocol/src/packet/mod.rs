mod ser;
pub use ser::*;

pub mod login;
mod state;
pub mod disconnect;

pub use state::*;

use crate::server::{Server, ServerPlayerNet};

pub trait Packet {
    const ID: i32;
}

pub trait S2CPacket: Packet {
    async fn send(self, server: &mut Server, client: &mut ServerPlayerNet);
}

pub trait C2SPacket: Packet {
    async fn receive(server: &mut Server, client: &mut ServerPlayerNet);
}