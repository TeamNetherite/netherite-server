use serde::{Deserialize, Serialize};
use netherite_common::macros::Packet;
use crate::packet::{Packet};
use crate::packet::state::State;

#[stdto::bytes(endian = "big")]
#[derive(Deserialize, Packet)]
#[packet(serverbound 0x00 in State::Status)]
pub struct C2SRequestStatus;