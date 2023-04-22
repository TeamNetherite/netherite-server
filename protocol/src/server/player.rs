use crate::error::NetheriteError;
use crate::packet::login::Handshake;
use crate::packet::{Packet, SerializedPacket, State};
use crate::server::Server;
use bytes::Bytes;
use futures::SinkExt;
use futures_sink::Sink;
use serde::Serialize;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::pin::{pin, Pin};
use std::sync::Arc;
use stdto::ToBytes;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    RwLock,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Framed};
use uuid::Uuid;

type Tx = UnboundedSender<Bytes>;
type Rx = UnboundedReceiver<Bytes>;

pub struct ServerPlayerNet {
    // connection
    addr: SocketAddr,
    frame: Framed<TcpStream, BytesCodec>,
    uuid: Uuid,

    pub state: State,
}

impl ServerPlayerNet {
    pub async fn new(stream: TcpStream) -> io::Result<Self> {
        let addr = stream.peer_addr()?;
        let frame = Framed::new(stream, BytesCodec::new());

        Ok(Self {
            addr,
            frame,
            state: State::Handshaking,
        })
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub async fn disconnect(&mut self) -> Result<(), NetheriteError> {
        self.state = State::Disconnected;

        self.frame.into_inner().shutdown().await?;

        Ok(())
    }

    async fn _launch(self: Arc<RwLock<Self>>, server: Arc<RwLock<Server>>) {
        loop {
            let result: Result<(), NetheriteError> = 'packet: {
                if let Some(frame) = self.read().await.frame.next().await {
                    if let Ok(bytes) = frame {
                        let packet: SerializedPacket =
                            <SerializedPacket as ToBytes>::from_bytes(bytes);

                        match self.read().await.state {
                            State::Disconnected => {
                                log::error!("received packet in `Disconnected` state")
                            }
                            State::Handshaking => {
                                match packet.packet_id() {
                                    0x00 => {
                                        let handshake_serverbound =
                                            Handshake::try_from_bytes(packet.data())?;
                                        if crate::PROTOCOL_VERSIONS.contains(
                                            &handshake_serverbound.protocol_version.value(),
                                        ) {
                                            self.write().await.state =
                                                handshake_serverbound.next_state();
                                        } else {
                                            self.write().await.disconnect()?;
                                            break 'packet Err(NetheriteError::UnsupportedProto(
                                                handshake_serverbound.protocol_version.value(),
                                            ));
                                        }
                                    }

                                    _ => log::error!("unknown packet: {}", packet),
                                };
                            }
                            State::Login => {}
                            State::Status => {}
                        };
                    };
                };

                Ok(())
            };
            if let Err(e) = result {
                log::error!("ERROR: {e}");
            }
        }
    }

    pub async fn send_packet<P: Packet + ToBytes + Serialize>(
        &mut self,
        packet: P,
    ) -> Result<(), NetheriteError> {
        Ok(self
            .frame
            .send(Bytes::from(packet.try_to_be_bytes()?))
            .await?)
    }

    pub(crate) fn process_packets(self: &Arc<RwLock<Self>>, server: &Arc<RwLock<Server>>) {
        tokio::spawn(self.clone()._launch(server.clone()));
    }
}
