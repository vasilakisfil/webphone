use common::bytes::Bytes;
use std::net::SocketAddr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdpTuple {
    pub bytes: Bytes,
    pub peer: SocketAddr,
}

impl From<(Bytes, SocketAddr)> for UdpTuple {
    fn from(tuple: (Bytes, SocketAddr)) -> Self {
        Self {
            bytes: tuple.0,
            peer: tuple.1,
        }
    }
}

impl Into<(Bytes, SocketAddr)> for UdpTuple {
    fn into(self) -> (Bytes, SocketAddr) {
        (self.bytes, self.peer)
    }
}

use common::futures::SinkExt;
use common::futures::stream::{SplitSink, SplitStream};
use common::futures_util::stream::StreamExt;
use common::tokio_util::codec::BytesCodec;
use common::tokio_util::udp::UdpFramed;
type UdpSink = SplitSink<UdpFramed<BytesCodec>, (Bytes, SocketAddr)>;
pub struct ServerHandle {
    udp_sink: UdpSink,
}

impl ServerHandle {
    fn new(udp_sink: UdpSink) -> Self {
        Self { udp_sink }
    }

    pub async fn send(&mut self, udp_tuple: impl Into<UdpTuple>) {
        if self.udp_sink.send(udp_tuple.into().into()).await.is_err() {
            common::log::error!("failed to send to udp socket");
        }
    }
}

impl From<UdpSink> for ServerHandle {
    fn from(udp_sink: UdpSink) -> Self {
        Self::new(udp_sink)
    }
}
