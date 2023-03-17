mod ser;

pub use ser::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stdto::ToBytes;

pub mod disconnect;
pub mod login;
mod state;

pub use state::*;

use crate::server::{Server, ServerPlayerNet};

pub trait Packet {
    const ID: i32;
}

pub trait S2CPacket: Packet {
    async fn send(self, server: &mut Server, client: &mut ServerPlayerNet)
    where
        Self: Sized + Serialize + ToBytes,
    {
        server.send_packet::<Self>(self, client.addr());
    }
}

pub trait C2SPacket: Packet + DeserializeOwned + ToBytes {
    async fn receive(server: &mut Server, client: &mut ServerPlayerNet) {
        server
            .receive_packet::<Self>(client.addr())
            .await
            .map(|a| a.process(server, client));
    }

    async fn process(self, server: &mut Server, client: &mut ServerPlayerNet) {}
}
