mod player;

use crate::error::NetheriteError;
use crate::packet::{Packet, SerializedPacket};
use bytes::Bytes;
pub use player::*;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use stdto::ToBytes;
use tokio::net::{TcpListener, TcpStream};
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
        let mut ok = self.net.lock().await;
        if ok.nets.contains_key(addr) {
            return Err(NetheriteError::DoubleLogin);
        }
        let mut player = ServerPlayerNet::new(stream).await?;
        player.process_packets();
        ok.nets.insert(addr.clone(), player);

        Ok(())
    }

    pub async fn receive_packet<P: Packet + ToBytes + DeserializeOwned>(
        &mut self,
        addr: &SocketAddr,
    ) -> Option<P> {
        if let Some(player) = self.net.lock().await.nets.get_mut(addr) {
            loop {
                let pack = player.get_packets.recv().await;
                if let Some(pack) = pack {
                    if pack.packet_id() == P::ID {
                        return P::try_from_bytes(pack.into_data()).ok()
                    }
                }
            }
        }

        None
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
