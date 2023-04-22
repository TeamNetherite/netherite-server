mod player;

use crate::error::NetheriteError;
use crate::packet::{Packet, SerializedPacket};
use bytes::Bytes;
pub use player::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::pin::{pin, Pin};
use std::sync::Arc;
use stdto::ToBytes;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

pub struct Server {
    tcp: TcpListener,
    net: Arc<Mutex<ServerNet>>,
}

impl Server {
    pub async fn new(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            tcp: TcpListener::bind(addr).await?,
            net: Arc::new(Mutex::new(ServerNet::new())),
        })
    }

    async fn accept_player(
        &mut self,
        addr: &SocketAddr,
        stream: TcpStream,
    ) -> Result<(), NetheriteError> {
        let mut net = self.net.lock().await;
        if net.nets.contains_key(addr) {
            return Err(NetheriteError::DoubleLogin);
        }
        let mut player = ServerPlayerNet::new(stream).await?;
        player.process_packets();
        net.nets.insert(addr.clone(), player);

        Ok(())
    }

    pub(crate) async fn handle_packet(packet: SerializedPacket) {
        
    }

    pub async fn send_packet<P: Packet + ToBytes + Serialize>(
        &mut self,
        packet: P,
        addr: &SocketAddr,
    ) -> Result<(), NetheriteError> {
        if let Some(player) = self.net.lock().await.nets.get_mut(addr) {
            player
                .send_packet(packet)
        } else {
            Err(NetheriteError::PlayerNotFound(addr.clone()))
        }
    }
}

struct ServerNet {
    nets: HashMap<SocketAddr, ServerPlayerNet>,
}

impl ServerNet {
    fn new() -> Self {
        Self {
            nets: HashMap::new(),
        }
    }
}
