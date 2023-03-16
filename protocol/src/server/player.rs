use crate::packet::{SerializedPacket, State};
use bytes::Bytes;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::pin::{pin, Pin};
use stdto::ToBytes;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Framed};
use crate::error::NetheriteError;

type Tx = UnboundedSender<Bytes>;
type Rx = UnboundedReceiver<Bytes>;

pub struct ServerPlayerNet {
    // connection
    addr: SocketAddr,
    frame: Framed<TcpStream, BytesCodec>,
    tx: Tx,
    rx: Rx,

    pub get_packets: UnboundedReceiver<SerializedPacket>,
    pub send_packets: UnboundedSender<SerializedPacket>,

    get_packets_transmit: UnboundedSender<SerializedPacket>,
    send_packets_recv: UnboundedReceiver<SerializedPacket>,

    pub state: State
}

impl ServerPlayerNet {
    pub async fn new(stream: TcpStream) -> io::Result<Self> {
        let addr = stream.peer_addr()?;
        let frame = Framed::new(stream, BytesCodec::new());
        let (tx, rx) = unbounded_channel::<Bytes>();
        let (gptx, gprx) = unbounded_channel();
        let (sptx, sprx) = unbounded_channel();

        Ok(Self {
            addr,
            frame,
            tx,
            rx,
            get_packets: gprx,
            send_packets: sptx,
            get_packets_transmit: gptx,
            send_packets_recv: sprx,
            state: State::None
        })
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub async fn disconnect(mut self) -> Result<(), NetheriteError> {
        self.state = State::None;
        
        self.frame.into_inner().shutdown().await?;

        Ok(())
    }

    async fn _launch(mut self: Pin<&mut Self>) {
        loop {
            // process frames
            if let Some(frame) = self.frame.next().await {
                if let Ok(bytes) = frame {
                    let _ = self.tx.send(bytes.into());
                };
            };

            // process incoming packets from rx and send to self.gptx
            if let Some(bytes) = self.rx.recv().await {
                let _ = self
                    .get_packets_transmit
                    .send(<SerializedPacket as ToBytes>::from_bytes(bytes));
            };

            // process packets from send_packets and send to self.frame
            if let Some(send_packet) = self.send_packets_recv.recv().await {
                let _ = self
                    .frame
                    .write_buffer_mut()
                    .extend_from_slice(&<SerializedPacket as ToBytes>::to_be_bytes(&send_packet));
            }
        }
    }

    pub(crate) fn process_packets(&mut self) {
        tokio::spawn(unsafe { Pin::new_unchecked(self) }._launch());
    }
}
